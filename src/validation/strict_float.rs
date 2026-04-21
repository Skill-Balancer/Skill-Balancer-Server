use std::fmt;
use serde::de::{self, Deserializer, Visitor};


// Custom Serde Deserializer for strict float deserialization
pub fn strict_float_validation<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StrictFloatValueVisitor)
}


struct StrictFloatValueVisitor;
impl<'de> Visitor<'de> for StrictFloatValueVisitor {
    type Value = Option<f32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a float")
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v as f32))
    }
}