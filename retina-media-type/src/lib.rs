// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod image;

pub use image::sniff_in_an_image_context;
use mime::Mime;

/// An XML MIME type is any MIME type whose subtype ends in "+xml" or whose
/// essence is "text/xml" or "application/xml". [RFC7303]
pub fn is_xml_mime_type(mime: &Mime) -> bool {
    if mime == &mime::TEXT_XML {
        return true;
    }

    if mime.type_() == mime::APPLICATION && mime.type_() == mime::XML {
        return true;
    }

    mime.suffix() == Some(mime::XML)
}
