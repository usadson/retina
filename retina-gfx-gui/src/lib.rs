// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use raw_window_handle::{
    HasRawDisplayHandle,
    HasRawWindowHandle,
    RawWindowHandle,
};

#[cfg(windows)]
mod win32;

mod attach_error;

pub use self::attach_error::GuiAttachError;

pub trait GuiManager: HasRawDisplayHandle + HasRawWindowHandle {

}

pub fn attach<W>(window: W) -> Result<Box<dyn GuiManager>, GuiAttachError>
        where W: HasRawWindowHandle + HasRawDisplayHandle {
    #[cfg(windows)]
    if let RawWindowHandle::Win32(handle) = window.raw_window_handle() {
        return win32::attach(handle, window.raw_display_handle());
    }

    Err(GuiAttachError::UnsupportedPlatform)
}
