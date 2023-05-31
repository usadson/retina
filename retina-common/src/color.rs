// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! A generic RGBA color with components ranging from 0.0 to 1.0 inclusive.

/// A generic RGBA color with components ranging from 0.0 to 1.0 inclusive.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}

//
// Constants
//
impl Color {
    /// A fully black color, i.e. #000000 or #000000FF.
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);

    /// A fully white color, i.e. #FFFFFF or #FFFFFFFF.
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);

    /// A fully transparent color, i.e. #00000000.
    pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);


    /// A fully green color, i.e. #FF0000 or #FF000000.
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);

    /// A fully green color, i.e. #00FF00 or #00FF0000.
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);

    /// A fully blue color, i.e. #0000FF or #0000FF00.
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);

    /// A fully magenta color, i.e. #FF00FF or #FF00FF00.
    pub const MAGENTA: Color = Color::rgb(1.0, 0.0, 1.0);
}

//
// Methods and functions
//
impl Color {
    /// Create a new [`Color`] with the given red, green, and blue components.
    pub const fn rgb(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue, alpha: 1.0 }
    }

    /// Create a new [`Color`] with the given red, green, blue, and alpha
    /// components.
    pub const fn rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self { red, green, blue, alpha }
    }

    pub fn rgb_decimal(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red: red as f64 / 255.0,
            green: green as f64 / 255.0,
            blue: blue as f64 / 255.0,
            alpha: 1.0
        }
    }

    /// Get the `red` RGB component of this color.
    pub const fn red(&self) -> f64 {
        self.red
    }

    /// Get the `green` RGB component of this color.
    pub const fn green(&self) -> f64 {
        self.green
    }

    /// Get the `blue` RGB component of this color.
    pub const fn blue(&self) -> f64 {
        self.blue
    }

    /// Get the `alpha` RGB component of this color.
    pub const fn alpha(&self) -> f64 {
        self.red
    }
}
