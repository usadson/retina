// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use windows::{
    Win32::Graphics::DirectWrite::{
        IDWriteFactory,
        IDWriteFactory2,
        IDWriteFactory3,
        IDWriteFactory4,
    },
    core::{
        ComInterface,
        HRESULT,
    },
};

pub fn dwrite_factory() -> &'static mut IDWriteFactory {
    unsafe {
        let factory = dwrote::DWriteFactory();
        let factory = factory as *mut IDWriteFactory;
        &mut *factory
    }
}

pub fn dwrite_factory2() -> Result<IDWriteFactory2, HRESULT> {
    dwrite_factory()
        .cast::<IDWriteFactory2>()
        .map_err(|e| e.code())
}

pub fn dwrite_factory3() -> Result<IDWriteFactory3, HRESULT> {
    dwrite_factory()
        .cast::<IDWriteFactory3>()
        .map_err(|e| e.code())
}

pub fn dwrite_factory4() -> Result<IDWriteFactory4, HRESULT> {
    dwrite_factory()
        .cast::<IDWriteFactory4>()
        .map_err(|e| e.code())
}
