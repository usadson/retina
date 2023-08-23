/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

// use wio::com::ComPtr;

use dwrote::GdiInterop;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F;
use windows::Win32::Graphics::Direct2D::D2D1CreateFactory;
use windows::Win32::Graphics::Direct2D::D2D1_DEBUG_LEVEL_INFORMATION;
use windows::Win32::Graphics::Direct2D::D2D1_FACTORY_OPTIONS;
use windows::Win32::Graphics::Direct2D::D2D1_FACTORY_TYPE;
use windows::Win32::Graphics::Direct2D::D2D1_FACTORY_TYPE_MULTI_THREADED;
use windows::Win32::Graphics::Direct2D::ID2D1Brush;
use windows::Win32::Graphics::Direct2D::ID2D1DeviceContext4;
use windows::Win32::Graphics::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS_CFF;
use windows::Win32::Graphics::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS_COLR;
use windows::Win32::Graphics::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS_JPEG;
use windows::Win32::Graphics::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS_PNG;
use windows::Win32::Graphics::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8;
use windows::Win32::Graphics::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS_SVG;
use windows::Win32::Graphics::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS_TIFF;
use windows::Win32::Graphics::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE;
use windows::Win32::Graphics::DirectWrite::DWRITE_TEXTURE_TYPE;
use windows::Win32::Graphics::DirectWrite::IDWriteBitmapRenderTarget;
use windows::core::ComInterface;
use windows::{
    core::HRESULT,
    Win32::Graphics::DirectWrite::{
        IDWriteGlyphRunAnalysis,
        DWRITE_GLYPH_RUN,
        DWRITE_MATRIX,
        DWRITE_RENDERING_MODE,
        DWRITE_MEASURING_MODE,
    },
};

use crate::com::dwrite_factory;
use crate::com::dwrite_factory4;

pub struct ColorGlyphRunAnalysis {
    native: IDWriteGlyphRunAnalysis,
}

impl ColorGlyphRunAnalysis {
    pub fn create(
        glyph_run: &DWRITE_GLYPH_RUN,
        pixels_per_dip: f32,
        transform: Option<DWRITE_MATRIX>,
        rendering_mode: DWRITE_RENDERING_MODE,
        measuring_mode: DWRITE_MEASURING_MODE,
        baseline_x: f32,
        baseline_y: f32,
    ) -> Result<Self, HRESULT> {
        let factory = dwrite_factory4()?;

        let color_formats =  DWRITE_GLYPH_IMAGE_FORMATS_PNG
            | DWRITE_GLYPH_IMAGE_FORMATS_SVG
            | DWRITE_GLYPH_IMAGE_FORMATS_COLR
            | DWRITE_GLYPH_IMAGE_FORMATS_CFF
            | DWRITE_GLYPH_IMAGE_FORMATS_JPEG
            | DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8
            | DWRITE_GLYPH_IMAGE_FORMATS_TIFF
            | DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE
        ;

        let baseline = D2D_POINT_2F {
            x: baseline_x,
            y: baseline_y,
        };

        unsafe {
            let glyph_run = glyph_run as *const DWRITE_GLYPH_RUN;

            let native = factory.CreateGlyphRunAnalysis(
                glyph_run,
                pixels_per_dip,
                transform
                    .as_ref()
                    .map(|x| x as *const _),
                rendering_mode,
                measuring_mode,
                baseline_x,
                baseline_y,
            )?;


            let enumerator = factory.TranslateColorGlyphRun2(
                baseline,
                glyph_run,
                None,
                color_formats,
                measuring_mode,
                None,
                0,
            )?;

            /*
            let options = D2D1_FACTORY_OPTIONS {
                debugLevel: D2D1_DEBUG_LEVEL_INFORMATION
                ..Default::default(),
            };

            let interop = GdiInterop::create();
            let width = Default::default();
            let height = Default::default();
            let target = interop.create_bitmap_render_target(width, height);
            let target = target.as_ptr();
            let target = target as *mut IDWriteBitmapRenderTarget;
            let target = &*target;
            let target = target.cast::<ID2D1DeviceContext4>().unwrap();



            while enumerator.MoveNext()?.as_bool() {
                let color_run = enumerator.GetCurrentRun2()?;
                let color_run = &*color_run;

                // item.Base.

                match color_run.glyphImageFormat {
                    DWRITE_GLYPH_IMAGE_FORMATS_SVG => {
                        // target.DrawSvgGlyphRun(
                        //     Default::default(),
                        //     &color_run.Base.glyphRun,

                        // )
                    }
                    _ => (),
                }
            }
            */

            Ok(Self {
                native,
            })
        }
    }

    pub fn get_alpha_texture_bounds(
        &self,
        texture_type: DWRITE_TEXTURE_TYPE,
    ) -> Result<RECT, HRESULT> {
        unsafe {
            self.native.GetAlphaTextureBounds(texture_type)
                .map_err(|e| e.code())
        }
    }

    pub fn create_alpha_texture(
        &self,
        texture_type: DWRITE_TEXTURE_TYPE,
        rect: RECT,
    ) -> Result<Vec<u8>, HRESULT> {
        let rect_pixels = (rect.right - rect.left) * (rect.bottom - rect.top);
        let rect_bytes = rect_pixels
            * match texture_type.0 {
                DWRITE_TEXTURE_ALIASED_1x1 => 1,
                DWRITE_TEXTURE_CLEARTYPE_3x1 => 3,
                _ => panic!("bad texture type specified"),
            };

        let mut out_bytes: Vec<u8> = vec![0; rect_bytes as usize];

        unsafe {
            self.native.CreateAlphaTexture(texture_type, &rect, &mut out_bytes)
                .map_err(|e| e.code())?;
        }

        Ok(out_bytes)
    }
}
