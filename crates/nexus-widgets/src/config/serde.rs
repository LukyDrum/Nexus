use serde::{Deserialize, Deserializer, Serializer};

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
    u32::from_str_radix(string, 16).map_err(serde::de::Error::custom)
}
