// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

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
};

struct Win32GuiManager {
    h_instance: winsafe::HINSTANCE,
    window: winsafe::HWND,
    display: RawDisplayHandle,
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
    }))
}
