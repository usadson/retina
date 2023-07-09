// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use log::info;
use retina_common::Color;
use retina_style::{CascadeOrigin, Stylesheet, CssColor};

use self::internal::CssNamedColorWrapper;

pub trait CssAttributeStrExtensions {
    fn html_interpret_as_hexadecimal_number(&self) -> Option<u8>;

    /// <https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#rules-for-parsing-a-legacy-colour-value>
    fn parse_legacy_color_value(&self) -> Option<CssColor>;

    fn parse_named_color(&self) -> Option<CssColor>;
}

impl CssAttributeStrExtensions for str {
    fn html_interpret_as_hexadecimal_number(&self) -> Option<u8> {
        let first = self.chars().nth(0)?.as_hexdigit()?;
        let second = self.chars().nth(1)?.as_hexdigit()?;

        Some(first * 0x10 + second)
    }

    fn parse_legacy_color_value(&self) -> Option<CssColor> {
        // 1. Let input be the string being parsed.
        // input => self

        // 2. If input is the empty string, then return an error.
        if self.is_empty() {
            return None;
        }

        // 3. Strip leading and trailing ASCII whitespace from input.
        let input = self.trim_matches(|c: char| c.is_ascii_whitespace());

        // 4. If input is an ASCII case-insensitive match for the string
        //    "transparent", then return an error.
        if input.eq_ignore_ascii_case("transparent") {
            return None;
        }

        // 5. If input is an ASCII case-insensitive match for one of the named
        //    colors, then return the simple color corresponding to that
        //    keyword.
        if let Some(color) = self.parse_named_color() {
            return Some(color);
        }

        // 6. If input's code point length is four, and the first character in
        //    input is U+0023 (#), and the last three characters of input are
        //    all ASCII hex digits, then:
        if self.len() == 4 && &self[0..1] == "#" {
            let r = self.chars().nth(1).as_ref().and_then(HexExt::as_hexdigit).map(|val| val * 17);
            let g = self.chars().nth(2).as_ref().and_then(HexExt::as_hexdigit).map(|val| val * 17);
            let b = self.chars().nth(3).as_ref().and_then(HexExt::as_hexdigit).map(|val| val * 17);
            if let Some(color) = r.zip(b.zip(g)) {
                return Some(CssColor::Color(Color::rgb_bytes(color.0, color.1.0, color.1.1)))
            }
        }

        // 7. Replace any code points greater than U+FFFF in input (i.e., any
        //    characters that are not in the basic multilingual plane) with the
        //    two-character string "00".
        let mut input = input.replace(|c| c as u32 > 0xFFFF, "00");
        info!("7: {input}");

        // 8. If input's code point length is greater than 128, truncate input,
        //    leaving only the first 128 characters.
        input.truncate(128);
        info!("8: {input}");

        // 9. If the first character in input is a U+0023 NUMBER SIGN character
        //    (#), remove it.
        if input.starts_with('#') {
            input = input[1..].to_string();
        }
        info!("9: {input}");

        // 10. Replace any character in input that is not an ASCII hex digit
        //     with the character U+0030 DIGIT ZERO (0).
        input = input.replace(|c: char| !c.is_ascii_hexdigit(), "0");
        info!("10: {input}");

        // 11. While input's code point length is zero or not a multiple of
        //     three, append a U+0030 DIGIT ZERO (0) character to input.
        while input.is_empty() || input.len() % 3 != 0 {
            input.push('0');
        }
        info!("11: {input}");

        // 12. Split input into three strings of equal code point length, to
        //     obtain three components. Let length be the code point length
        //     that all of those components have (one third the code point
        //     length of input).
        let mut length = input.len() / 3;
        let mut red = &input[0..length];
        let mut green = &input[length..length * 2];
        let mut blue = &input[length * 2..];

        info!("12: {red}, {green}, {blue}");

        // 13. If length is greater than 8, then remove the leading length-8
        //     characters in each component, and let length be 8.
        if length > 8 {
            // TODO is this right?
            length = 8;
            red = &red[0..8];
            green = &green[0..8];
            blue = &blue[0..8];
        }

        info!("13: {red}, {green}, {blue}");

        // 14. While length is greater than two and the first character in each
        //     component is a U+0030 DIGIT ZERO (0) character, remove that
        //     character and reduce length by one.
        while length > 2 && red.starts_with('0') && green.starts_with('0') && blue.starts_with('0') {
            length -= 1;
            red = &red[1..];
            green = &green[1..];
            blue = &blue[1..];
        }

        info!("14: {red}, {green}, {blue}");

        // 15. If length is still greater than two, truncate each component,
        //     leaving only the first two characters in each.
        if length > 2 {
            red = &red[0..2];
            green = &green[0..2];
            blue = &blue[0..2];
        }

        info!("15: {red}, {green}, {blue}");

        // 16. Let result be a simple color.
        // (Constructed at return)

        // 17. Interpret the first component as a hexadecimal number; let the
        //     red component of result be the resulting number.
        let red = red.html_interpret_as_hexadecimal_number().unwrap();

        // 18. Interpret the second component as a hexadecimal number; let the
        //     green component of result be the resulting number.
        let green = green.html_interpret_as_hexadecimal_number().unwrap();

        // 19. Interpret the third component as a hexadecimal number; let the
        //     blue component of result be the resulting number.
        let blue = blue.html_interpret_as_hexadecimal_number().unwrap();

        info!("Legacy color \"{self}\" parsed to ({red}, {green}, {blue}");

        Some(CssColor::Color(Color::rgb_bytes(red, green, blue)))
    }

    fn parse_named_color(&self) -> Option<CssColor> {
        cssparser::parse_color_keyword::<CssNamedColorWrapper>(self)
            .ok()?
            .0
    }
}

pub trait HexExt {
    fn as_hexdigit(&self) -> Option<u8>;
}

impl HexExt for char {
    fn as_hexdigit(&self) -> Option<u8> {
        if ('0'..='9').contains(self) {
            Some(*self as u8 - b'0')
        } else if ('A'..='F').contains(self) {
            Some(*self as u8 - b'A')
        } else if ('a'..='f').contains(self) {
            Some(*self as u8 - b'a')
        } else {
            None
        }
    }
}

pub trait CssParsable {
    fn parse(cascade_origin: CascadeOrigin, input: &str) -> Self;
}

impl CssParsable for Stylesheet {
    fn parse(cascade_origin: CascadeOrigin, input: &str) -> Self {
        crate::parse_stylesheet(cascade_origin, input)
    }
}

pub trait FromCssParser<T> {
    type Output;

    fn into(self) -> Self::Output;
}

pub fn convert_color(value: cssparser::Color) -> Option<CssColor> {
    match value {
        cssparser::Color::Rgba(rgba) => Some(convert_rgba(rgba)),
        _ => None,
    }
}

pub fn convert_rgba(value: cssparser::RGBA) -> retina_style::CssColor {
    let mut color = retina_common::Color::rgb_bytes(
        value.red.unwrap_or(0),
        value.green.unwrap_or(0),
        value.blue.unwrap_or(0),
    );

    if let Some(alpha) = value.alpha {
        color = color.with_alpha(alpha as _);
    }

    retina_style::CssColor::Color(color)
}

mod internal {
    use cssparser::FromParsedColor;
    use retina_common::Color;
    use retina_style::CssColor;

    pub struct CssNamedColorWrapper(pub Option<CssColor>);

    impl FromParsedColor for CssNamedColorWrapper {
        fn from_color_function(
            _color_space: cssparser::PredefinedColorSpace,
            _c1: Option<f32>,
            _c2: Option<f32>,
            _c3: Option<f32>,
            _alpha: Option<f32>,
        ) -> Self {
            Self(None)
        }

        fn from_current_color() -> Self {
            Self(None)
        }

        fn from_hsl(
            _hue: Option<f32>,
            _saturation: Option<f32>,
            _lightness: Option<f32>,
            _alpha: Option<f32>,
        ) -> Self {
            Self(None)
        }

        fn from_hwb(
            _hue: Option<f32>,
            _whiteness: Option<f32>,
            _blackness: Option<f32>,
            _alpha: Option<f32>,
        ) -> Self {
            Self(None)
        }

        fn from_lab(
            _lightness: Option<f32>,
            _a: Option<f32>,
            _b: Option<f32>,
            _alpha: Option<f32>,
        ) -> Self {
            Self(None)
        }

        fn from_lch(
            _lightness: Option<f32>,
            _chroma: Option<f32>,
            _hue: Option<f32>,
            _alpha: Option<f32>,
        ) -> Self {
            Self(None)
        }

        fn from_oklab(
            _lightness: Option<f32>,
            _a: Option<f32>,
            _b: Option<f32>,
            _alpha: Option<f32>,
        ) -> Self {
            Self(None)
        }

        fn from_oklch(
            _lightness: Option<f32>,
            _chroma: Option<f32>,
            _hue: Option<f32>,
            _alpha: Option<f32>,
        ) -> Self {
            Self(None)
        }

        fn from_rgba(
            red: Option<u8>,
            green: Option<u8>,
            blue: Option<u8>,
            alpha: Option<f32>
        ) -> Self {
            Self(Some(CssColor::Color(Color::rgba(
                red.unwrap_or_default() as _,
                green.unwrap_or_default() as _,
                blue.unwrap_or_default() as _,
                alpha.unwrap_or_default() as _,
            ))))
        }
    }
}
