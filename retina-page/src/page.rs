// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    rc::Rc,
    sync::mpsc::{Receiver, Sender}, borrow::Cow,
};

use retina_compositor::Compositor;
use retina_dom::{NodeKind, HtmlElementKind};
use retina_gfx::{canvas::CanvasPaintingContext, Color};
use retina_layout::{LayoutBox, LayoutGenerator};
use retina_style::{Stylesheet, CascadeOrigin, CssReferencePixels};
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
}

type ErrorKind = Box<dyn std::error::Error>;

impl Page {
    pub(crate) async fn start(&mut self) -> Result<(), ErrorKind> {
        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::Initial,
        })?;

        println!("[page] Starting page: {:?}", self.url);

        let page_data = self.load_page().await?;

        self.parse_html(page_data.as_ref()).await?;
        println!("[page] HTML parsed...");

        self.parse_stylesheets().await?;
        println!("[page] Stylesheets parsed...");

        self.generate_layout_tree().await?;
        println!("[page] Layout tree generated...");

        self.paint()?;
        println!("[page] painted...");

        println!("[page] Ready! Waiting on commands...");
        self.message_sender.send(PageMessage::Progress { progress: PageProgress::Ready })?;

        while let Ok(command) = self.command_receiver.recv() {
            println!("[page] Received command: {command:#?}");
        }

        println!("[page] Command pipeline dead!");

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

    pub(crate) async fn load_page(&mut self) -> Result<Cow<'static, str>, ErrorKind> {
        let data = match self.url.scheme() {
            "about:" => self.load_page_about(self.url.path()).await?,
            _ => self.load_page_about("not-found").await?,
        };

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::Fetched,
        })?;

        Ok(data)
    }

    pub(crate) async fn load_page_about(&self, page: &str) -> Result<Cow<'static, str>, ErrorKind> {
        Ok(match page {
            "not-found" => retina_user_agent::url_scheme::about::NOT_FOUND.into(),
            _ => retina_user_agent::url_scheme::about::NOT_FOUND.into(),
        })
    }

    pub(crate) fn paint(&mut self) -> Result<(), ErrorKind> {
        let Some(layout_root) = self.layout_root.as_ref() else {
            return Ok(());
        };

        let mut painter = self.canvas.begin(Color::TRANSPARENT);

        self.compositor.paint(layout_root, &mut painter);

        painter.submit_and_present();

        self.message_sender.send(PageMessage::PaintReceived {
            texture_view: self.canvas.create_view()
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

    pub(crate) async fn parse_html(&mut self, page: &str) -> Result<(), ErrorKind> {
        // TODO It would be better to perform this on a thread that may block,
        //      but since the DOM is currently represented as `Rc` and not
        //      `Arc`, this is currently impossible.
        self.document = Some(
            retina_dom::Parser::parse(page)
        );

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::ParsedHtml,
        })?;

        Ok(())
    }
}
