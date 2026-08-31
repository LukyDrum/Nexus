use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Color {
    /// Like `(255, 255, 255, 255)`
    RgbaU8(u8, u8, u8, #[serde(default = "default_alpha_u8")] u8),
    /// Like `(1.0, 1.0, 1.0, 1.0)`
    RgbaF32(f32, f32, f32, #[serde(default = "default_alpha_f32")] f32),
    /// Like `(0xFFFFFFFF)`
    Hex(
        #[serde(
            serialize_with = "serialize_hex_u32",
            deserialize_with = "deserialize_hex_u32"
        )]
        u32,
    ),
}

impl Default for Color {
    fn default() -> Self {
        Self::Hex(0x000000FF)
    }
}

fn default_alpha_u8() -> u8 {
    255
}

fn default_alpha_f32() -> f32 {
    1.0
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

pub(super) fn serialize_hex_u32<S>(x: &u32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("#{:x}", x))
}

pub(super) fn deserialize_hex_u32<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(d)?;
    let string = string.strip_prefix("#").unwrap_or(&string);
    let value = u32::from_str_radix(string, 16).map_err(serde::de::Error::custom)?;

    // Treat RGB values as RGBA values with A = FF
    if string.len() > 6 {
        Ok(value)
    } else {
        Ok(value << 2 | 0xFF)
    }
}
