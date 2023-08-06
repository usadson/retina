// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod image;

pub use image::sniff_in_an_image_context;
use mime::Mime;

pub use mime;

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

pub trait MimeExtensions {
    fn is_svg(&self) -> bool;
    fn is_xml_mime_type(&self) -> bool;
}

impl MimeExtensions for Mime {
    fn is_svg(&self) -> bool {
        self.is_xml_mime_type()
            && self.subtype() == mime::SVG
    }

    fn is_xml_mime_type(&self) -> bool {
        is_xml_mime_type(self)
    }
}
