use std::str::FromStr;

use crate::language::Value;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    /// Like `(255, 255, 255, 255)`
    RgbaU8(u8, u8, u8, u8),
    /// Like `(1.0, 1.0, 1.0, 1.0)`
    RgbaF32(f32, f32, f32, f32),
    /// Like `(0xFFFFFFFF)`
    Hex(u32),
}

impl Default for Color {
    fn default() -> Self {
        Self::Hex(0x000000FF)
    }
}

impl From<Color> for iced::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::RgbaU8(r, g, b, a) => iced::Color::from_rgba8(r, g, b, a as f32 / 255.0),
            Color::RgbaF32(r, g, b, a) => iced::Color::from_rgba(r, g, b, a),
            Color::Hex(value) => {
                let r = (value >> 24) & 0xFF;
                let g = (value >> 16) & 0xFF;
                let b = (value >> 8) & 0xFF;
                let a = value & 0xFF;
                iced::Color::from_rgba8(r as u8, g as u8, b as u8, a as f32 / 255.0)
            }
        }
    }
}

impl From<Color> for iced::Background {
    fn from(value: Color) -> Self {
        let color = value.into();
        iced::Background::Color(color)
    }
}

pub struct ParseHexError;

impl FromStr for Color {
    type Err = ParseHexError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.strip_prefix("#").unwrap_or(string);
        let value = u32::from_str_radix(string, 16).map_err(|_| ParseHexError)?;

        // Treat RGB values as RGBA values with A = FF
        if string.len() > 6 {
            Ok(Self::Hex(value))
        } else {
            Ok(Self::Hex(value << 2 | 0xFF))
        }
    }
}

impl From<&Value> for Color {
    fn from(value: &Value) -> Self {
        match value {
            Value::String(color) => Color::from_str(color).unwrap_or_default(),
            Value::Array(rgba) => {
                let rgba = rgba.read().expect("Lock poisoned");
                if let Some(Value::Int(r)) = rgba.first()
                    && let Some(Value::Int(g)) = rgba.get(1)
                    && let Some(Value::Int(b)) = rgba.get(2)
                    && let Value::Int(a) = rgba.get(3).unwrap_or(&Value::Int(255))
                {
                    Color::RgbaU8(*r as u8, *g as u8, *b as u8, *a as u8)
                } else {
                    Color::default()
                }
            }
            _ => Color::default(),
        }
    }
}
