use std::fmt::Formatter;
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{Error, Visitor};


struct StringTimeVisitor;

impl<'de> Visitor<'de> for StringTimeVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Ok(v.replace(":", "-"))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(v.replace(":", "-"))
    }
}


fn time_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
    where D: Deserializer<'de>,
{
    deserializer.deserialize_str(StringTimeVisitor)
}


#[derive(Serialize, Deserialize)]
pub struct TemporalVideoMessage {
    #[serde(alias = "node")]
    pub node_id: u32,
    #[serde(alias = "camera")]
    pub camera_id: u32,
    pub path: String,
    #[serde(alias = "date")]
    pub video_date: String,
    #[serde(alias = "time")]
    #[serde(deserialize_with = "time_deserializer")]
    pub video_time: String
}
