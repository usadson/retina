// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    rc::Rc,
    sync::mpsc::{Receiver, Sender},
};

use log::{error, info};
use retina_compositor::Compositor;
use retina_dom::{NodeKind, HtmlElementKind};
use retina_fetch::Fetch;
use retina_gfx::{canvas::CanvasPaintingContext, Color};
use retina_layout::{LayoutBox, LayoutGenerator};
use retina_style::{Stylesheet, CascadeOrigin, CssReferencePixels};
use retina_style_parser::CssParsable;
use url::Url;

use crate::{PageCommand, PageMessage, PageProgress};

pub(crate) struct Page {
    pub(crate) command_receiver: Receiver<PageCommand>,
    pub(crate) message_sender: Sender<PageMessage>,

    pub(crate) url: Url,
    pub(crate) document: Option<Rc<NodeKind>>,
    pub(crate) style_sheets: Option<Vec<Stylesheet>>,
    pub(crate) layout_root: Option<LayoutBox>,

    pub(crate) canvas: CanvasPaintingContext,
    pub(crate) compositor: Compositor,
    pub(crate) fetch: Fetch,
}

type ErrorKind = Box<dyn std::error::Error>;

impl Page {
    pub(crate) async fn start(&mut self) -> Result<(), ErrorKind> {
        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::Initial,
        })?;

        info!("Loading page: {:?}", self.url);

        self.load_page().await?;

        self.parse_stylesheets().await?;
        info!("Stylesheets: {:#?}", self.style_sheets.as_ref().unwrap());

        self.generate_layout_tree().await?;

        self.paint()?;

        self.message_sender.send(PageMessage::Progress { progress: PageProgress::Ready })?;

        while let Ok(command) = self.command_receiver.recv() {
            self.handle_command(command).await?;

            // If there are commands sent consecutively, handle them before sending
            // `PageProgress::Ready`.
            while let Ok(command) = self.command_receiver.try_recv() {
                self.handle_command(command).await?;
            }

            self.message_sender.send(PageMessage::Progress { progress: PageProgress::Ready })?;
        }

        error!("Command pipeline dead!");

        Ok(())
    }

    pub(crate) async fn generate_layout_tree(&mut self) -> Result<(), ErrorKind> {
        self.layout_root = Some(
            LayoutGenerator::generate(
                Rc::clone(self.document.as_ref().unwrap()),
                &self.style_sheets.as_ref().unwrap(),
                CssReferencePixels::new(self.canvas.size().width as _),
                CssReferencePixels::new(self.canvas.size().height as _),
            )
        );

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::LayoutGenerated,
        })?;

        Ok(())
    }

    pub(crate) async fn handle_command(&mut self, command: PageCommand) -> Result<(), ErrorKind> {
        info!("Received command: {command:#?}");

        match command {
            PageCommand::ResizeCanvas { size } => {
                self.canvas.resize(size);
                self.paint()?;
            }
        }

        Ok(())
    }

    pub(crate) async fn load_page(&mut self) -> Result<(), ErrorKind> {
        let mut document = self.fetch.fetch_document(self.url.clone()).await?;

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::Fetched,
        })?;

        let mut reader = document.body().await;

        self.document = Some(
            retina_dom::Parser::parse_with_reader(&mut reader)
        );

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::ParsedHtml,
        })?;

        Ok(())
    }

    pub(crate) fn paint(&mut self) -> Result<(), ErrorKind> {
        let Some(layout_root) = self.layout_root.as_ref() else {
            return Ok(());
        };

        // <https://html.spec.whatwg.org/multipage/rendering.html#phrasing-content-3:'background-color'>
        // > The initial value for the 'color' property is expected to be black.
        // > The initial value for the 'background-color' property is expected
        // > to be 'transparent'. The canvas's background is expected to be white.
        let mut painter = self.canvas.begin(Color::WHITE);

        self.compositor.paint(layout_root, &mut painter);

        painter.submit_and_present();

        self.message_sender.send(PageMessage::PaintReceived {
            texture_view: self.canvas.create_view(),
            texture_size: self.canvas.size(),
        })?;

        Ok(())
    }

    pub(crate) async fn parse_stylesheets(&mut self) -> Result<(), ErrorKind> {
        let mut stylesheets = vec![
            retina_style::Stylesheet::parse(
                CascadeOrigin::UserAgent,
                retina_user_agent::stylesheet::USER_AGENT_STYLESHEET_CODE
            ),
        ];

        println!("document: {:#?}", self.document);

        self.document.as_ref()
            .unwrap()
            .for_each_child_node_recursive(&mut |child, _depth| {
                if let Some(style) = child.as_html_element_kind().and_then(HtmlElementKind::as_style_element) {
                    stylesheets.push(retina_style::Stylesheet::parse(CascadeOrigin::Author, style.style_content().as_ref()));
                }
            }, 0);

        self.style_sheets = Some(stylesheets);

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::ParsedCss,
        })?;

        Ok(())
    }
}
