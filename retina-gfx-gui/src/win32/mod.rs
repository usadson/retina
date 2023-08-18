// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::collections::HashMap;

use raw_window_handle::{
    HasRawDisplayHandle,
    HasRawWindowHandle,
    RawDisplayHandle,
    RawWindowHandle,
    Win32WindowHandle,
};

use winsafe::prelude::*;

use crate::{
    GuiAttachError,
    GuiManager,
    ContextMenu as RetinaContextMenu,
    context_menu::ContextMenuItemKind,
};

struct Win32GuiManager {
    h_instance: winsafe::HINSTANCE,
    window: winsafe::HWND,
    display: RawDisplayHandle,
    menu: Option<muda::Menu>,
}

unsafe impl HasRawDisplayHandle for Win32GuiManager {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.display
    }
}

unsafe impl HasRawWindowHandle for Win32GuiManager {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = Win32WindowHandle::empty();
        handle.hinstance = self.h_instance.ptr();
        handle.hwnd = self.hwnd().ptr();

        RawWindowHandle::Win32(handle)
    }
}

impl GuiManager for Win32GuiManager {
    fn open_context_menu(&mut self, menu_info: RetinaContextMenu) {
        let hwnd = self.window.ptr() as isize;

        if let Some(menu) = self.menu.take() {
            menu.detach_menu_subclass_from_hwnd(hwnd);
        }

        use muda::{
            ContextMenu,
            Menu,
            MenuEvent,
            MenuItem,
            PredefinedMenuItem,
        };

        let menu = Menu::new();
        let mut item_map = HashMap::new();
        let position = Some(muda::Position::Physical(muda::PhysicalPosition::new(
            menu_info.position().x as i32,
            menu_info.position().y as i32
        )));

        for item_info in menu_info.into_items() {

            match item_info.kind() {
                ContextMenuItemKind::Separator => {
                    let item = PredefinedMenuItem::separator();
                    item_map.insert(item.id().clone(), item_info);
                    menu.append(&item).unwrap();
                }

                ContextMenuItemKind::Text(item) => {
                    let item = MenuItem::new(item.title().to_string(), true, None);
                    item_map.insert(item.id().clone(), item_info);
                    menu.append(&item).unwrap();
                }
            }
        }

        menu.attach_menu_subclass_for_hwnd(hwnd);
        menu.show_context_menu_for_hwnd(hwnd, position);
        self.menu = Some(menu);

        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            if let Some(item) = item_map.get(event.id()) {
                match item.kind() {
                    ContextMenuItemKind::Separator => (),
                    ContextMenuItemKind::Text(item) => item.invoke_action(),
                }
            };
        }));
    }
}

impl winsafe::prelude::GuiWindow for Win32GuiManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn hwnd(&self) -> &winsafe::HWND {
        &self.window
    }
}

impl winsafe::prelude::GuiParent for Win32GuiManager {
    unsafe fn as_base(&self) -> *mut std::ffi::c_void {
        todo!()
    }

    fn on(&self) -> &winsafe::gui::events::WindowEventsAll {
        todo!()
    }
}

pub(crate) fn attach(
    window_handle: Win32WindowHandle,
    display: RawDisplayHandle,
) -> Result<Box<dyn GuiManager>, GuiAttachError> {
    let RawDisplayHandle::Windows(..) = display else {
        return Err(GuiAttachError::WindowAndDisplayNotMatchPlatform);
    };

    Ok(Box::new(Win32GuiManager {
        h_instance: unsafe { winsafe::HINSTANCE::from_ptr(window_handle.hinstance) },
        window: unsafe { winsafe::HWND::from_ptr(window_handle.hwnd) },
        display,
        menu: None,
    }))
}
