// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::io::{BufRead, Seek};

use mime::Mime;

use crate::is_xml_mime_type;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageKind {
    Avif,
    Bmp,
    Dds,
    Farbfeld,
    Gif,
    Hdr,
    Ico,
    Jpeg,
    OpenExr,
    Png,
    Pnm,
    Qoi,
    Svg,
    Tga,
    Tiff,
    WebP,
}

impl ImageKind {
    pub fn to_bitmap_image_format(&self) -> Option<image::ImageFormat> {
        use image::ImageFormat;

        Some(match self {
            Self::Avif => ImageFormat::Avif,
            Self::Bmp => ImageFormat::Bmp,
            Self::Dds => ImageFormat::Dds,
            Self::Farbfeld => ImageFormat::Farbfeld,
            Self::Gif => ImageFormat::Gif,
            Self::Hdr => ImageFormat::Hdr,
            Self::Ico => ImageFormat::Ico,
            Self::Jpeg => ImageFormat::Jpeg,
            Self::OpenExr => ImageFormat::OpenExr,
            Self::Png => ImageFormat::Png,
            Self::Pnm => ImageFormat::Pnm,
            Self::Qoi => ImageFormat::Qoi,
            Self::Svg => return None,
            Self::Tga => ImageFormat::Tga,
            Self::Tiff => ImageFormat::Tiff,
            Self::WebP => ImageFormat::WebP,
        })
    }
}

/// [Algorithm][algo] as defined by the MIME Sniffing Standard.
///
/// [algo]: https://mimesniff.spec.whatwg.org/#image-type-pattern-matching-algorithm
fn match_image_type_pattern_by_pattern<R>(resource: &mut R) -> Option<ImageKind>
        where R: BufRead {
    let mut first_four_bytes = [0u8; 4];
    if resource.read_exact(&mut first_four_bytes).is_err() {
        return None;
    }

    if &first_four_bytes[0..3] == &[0xFF, 0xD8, 0xFF] {
        return Some(ImageKind::Jpeg);
    }

    if first_four_bytes == [0, 0, 1, 0] || first_four_bytes == [0, 0, 2, 0] {
        return Some(ImageKind::Ico);
    }

    if &first_four_bytes[0..2] == b"BM" {
        return Some(ImageKind::Bmp);
    }

    let mut second_two_bytes = [0u8; 2];
    if resource.read_exact(&mut second_two_bytes).is_err() {
        return None;
    }

    if match_image_type_pattern_for_gif(&first_four_bytes, &second_two_bytes) {
        return Some(ImageKind::Gif);
    }

    let mut third_two_bytes = [0u8; 2];
    if resource.read_exact(&mut third_two_bytes).is_err() {
        return None;
    }

    if match_image_type_pattern_for_png(&first_four_bytes, &second_two_bytes, &third_two_bytes) {
        return Some(ImageKind::Png);
    }

    let mut last_six_bytes = [0u8; 6];
    if resource.read_exact(&mut last_six_bytes).is_err() {
        return None;
    }

    if first_four_bytes == [b'R', b'I', b'F', b'F'] && &last_six_bytes == b"WEBPVP" {
        return Some(ImageKind::WebP);
    }

    None
}

fn match_image_type_pattern_for_gif(first_four_bytes: &[u8; 4], second_two_bytes: &[u8; 2]) -> bool {
    if first_four_bytes != &[b'G', b'I', b'F', b'8'] {
        return false;
    }

    if second_two_bytes == &[b'7', b'a'] {
        return true;
    }

    if second_two_bytes == &[b'9', b'a'] {
        return true;
    }

    false
}

fn match_image_type_pattern_for_png(
    first_four_bytes: &[u8; 4],
    second_two_bytes: &[u8; 2],
    third_two_bytes: &[u8; 2],
) -> bool {
    if first_four_bytes[0] != 0x89 {
        return false;
    }

    if &first_four_bytes[1..4] != b"PNG" {
        return false;
    }

    second_two_bytes == &[0x0D, 0x0A] && third_two_bytes == &[0x1A, 0x0A]
}

pub fn media_type_to_image_format(media_type: &Mime) -> Option<ImageKind> {
    if media_type.type_() != mime::IMAGE {
        return None;
    }

    match media_type.subtype().as_str() {
        "avif" => return Some(ImageKind::Avif),

        "dds" => return Some(ImageKind::Dds),

        "exr" => return Some(ImageKind::OpenExr),

        "farbfeld" => return Some(ImageKind::Farbfeld),

        "hdr" => return Some(ImageKind::Hdr),

        "pnm" => return Some(ImageKind::Pnm),

        "tga" => return Some(ImageKind::Tga),

        "tiff" => return Some(ImageKind::Tiff),

        "qoi" => return Some(ImageKind::Qoi),

        "image/x-icon" | "vnd.microsoft.icon" => return Some(ImageKind::Ico),

        "webp" => return Some(ImageKind::WebP),

        _ => (),
    }

    Some(match media_type.subtype() {
        mime::BMP => ImageKind::Bmp,
        mime::GIF => ImageKind::Gif,
        mime::JPEG => ImageKind::Jpeg,
        mime::PNG => ImageKind::Png,

        _ => return None,
    })
}

/// To determine the computed MIME type of a resource with an image MIME type
/// execute [the following rules][spec] for sniffing images specifically.
///
/// [spec]: https://mimesniff.spec.whatwg.org/#rules-for-sniffing-images-specifically
pub fn sniff_in_an_image_context<'mime, R>(resource: &mut R, media_type: &Mime) -> Option<ImageKind>
        where R: BufRead + Seek {
    // 1. An XML MIME type is any MIME type whose subtype ends in "+xml" or
    //    whose essence is "text/xml" or "application/xml". [RFC7303]
    if is_xml_mime_type(media_type) {
        if media_type.subtype() == mime::SVG {
            return Some(ImageKind::Svg);
        }

        return None;
    }

    // 2. Let image-type-matched be the result of executing the image type
    // pattern matching algorithm with the resource header as the byte sequence
    // to be matched.

    if let Some(image_kind) = match_image_type_pattern_by_pattern(resource) {
        return Some(image_kind);
    }

    media_type_to_image_format(media_type)
}
