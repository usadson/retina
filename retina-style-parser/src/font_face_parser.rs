// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::Range;

use cssparser::Parser;
use retina_style::{
    CssFontFaceFormat,
    CssFontFaceDeclaration,
    CssFontFaceProperty,
    CssFontFaceSrc,
};
use strum::IntoEnumIterator;

use crate::{
    ParseError,
    RetinaStyleParseError,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontFaceParser;

impl<'i> cssparser::DeclarationParser<'i> for FontFaceParser {
    type Declaration = CssFontFaceDeclaration;
    type Error = RetinaStyleParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, cssparser::ParseError<'i, Self::Error>> {
        let property = CssFontFaceProperty::parse(name.as_ref())
            .ok_or_else(|| input.new_custom_error(RetinaStyleParseError::AtFontFaceInvalidProperty(name)))?;

        match property {
            CssFontFaceProperty::FontFamily => parse_declaration_font_family(input),
            CssFontFaceProperty::FontStyle => parse_declaration_font_style(input),
            CssFontFaceProperty::FontWeight => parse_declaration_font_weight(input),
            CssFontFaceProperty::Src => parse_declaration_src(input),
            CssFontFaceProperty::UnicodeRanges => parse_declaration_unicode_ranges(input),
        }
    }
}

impl<'i> cssparser::RuleBodyItemParser<'i, CssFontFaceDeclaration, RetinaStyleParseError<'i>> for FontFaceParser {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

impl<'i> cssparser::AtRuleParser<'i> for FontFaceParser {
    type Prelude = ();
    type AtRule = CssFontFaceDeclaration;
    type Error = RetinaStyleParseError<'i>;
    // ignored / errors upon
}

impl<'i> cssparser::QualifiedRuleParser<'i> for FontFaceParser {
    type Prelude = ();
    type QualifiedRule = CssFontFaceDeclaration;
    type Error = RetinaStyleParseError<'i>;
    // ignored / errors upon
}

pub(crate) fn parse_declaration_font_family<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssFontFaceDeclaration, ParseError<'i>> {
    let name = input.expect_ident_or_string()?;

    Ok(CssFontFaceDeclaration::FontFamily(name.as_ref().into()))
}

pub(crate) fn parse_declaration_font_style<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssFontFaceDeclaration, ParseError<'i>> {
    let value = super::value_parser::parse_font_style(input)?;
    Ok(CssFontFaceDeclaration::FontStyle(value))
}

pub(crate) fn parse_declaration_font_weight<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssFontFaceDeclaration, ParseError<'i>> {
    let value = input.expect_number()?;
    Ok(CssFontFaceDeclaration::FontWeight(value as _))
}

pub(crate) fn parse_declaration_src<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssFontFaceDeclaration, ParseError<'i>> {
    Ok(CssFontFaceDeclaration::Src {
        sources: input.parse_comma_separated(parse_declaration_src_item)?,
    })
}

pub(crate) fn parse_declaration_src_item<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssFontFaceSrc, ParseError<'i>> {
    if let Ok(src) = input.try_parse(parse_declaration_src_item_local) {
        return Ok(src);
    }

    parse_declaration_src_item_remote(input)
}

pub(crate) fn parse_declaration_src_item_local<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssFontFaceSrc, ParseError<'i>> {
    input.expect_function_matching("local")?;

    let name = input.parse_nested_block(|input| {
        let name = input.expect_ident_or_string()?;
        Ok(name.clone())
    })?;

    Ok(CssFontFaceSrc::Local(name.as_ref().into()))
}

pub(crate) fn parse_declaration_src_item_remote<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssFontFaceSrc, ParseError<'i>> {
    let url = input.expect_url()?;

    let format = if input.expect_function_matching("format").is_err() {
        CssFontFaceFormat::Unknown
    } else {
        input.parse_nested_block(parse_declaration_src_format)?
    };

    Ok(CssFontFaceSrc::WebFont {
        url: url.as_ref().into(),
        format,
    })
}

pub(crate) fn parse_declaration_src_format<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssFontFaceFormat, ParseError<'i>> {
    let ty = input.expect_ident_or_string()?;

    let format = CssFontFaceFormat::iter()
        .find(|s| {
            s.as_ref().eq_ignore_ascii_case(&ty)
        })
        .unwrap_or(CssFontFaceFormat::Unknown);

    Ok(format)
}

pub(crate) fn parse_declaration_unicode_ranges<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssFontFaceDeclaration, ParseError<'i>> {
    Ok(CssFontFaceDeclaration::UnicodeRanges(
        input.parse_comma_separated(parse_unicode_range)?
    ))
}

pub(crate) fn parse_unicode_range<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Range<u32>, ParseError<'i>> {
    let range = cssparser::UnicodeRange::parse(input)?;
    Ok(range.start..range.end)
}
