use derive_more::{From, Into};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, From, Into, Copy, Clone, PartialOrd, PartialEq)]
pub struct SequenceID(u64);

#[derive(Debug, Serialize, Deserialize, From, Into, Copy, Clone, PartialOrd, PartialEq)]
pub struct GuildID(Snowflake);

#[derive(Debug, Serialize, Deserialize, From, Into, Copy, Clone, PartialOrd, PartialEq)]
pub struct ChannelID(Snowflake);

#[derive(Debug, Serialize, Deserialize, From, Into, Copy, Clone, PartialOrd, PartialEq)]
pub struct UserID(Snowflake);

#[derive(Debug, Serialize, Deserialize, From, Into, Copy, Clone, PartialOrd, PartialEq)]
pub struct MessageID(Snowflake);

#[derive(Debug, Copy, Clone, From, Into, PartialOrd, PartialEq)]
#[repr(transparent)]
pub struct Snowflake(u64);

impl Serialize for Snowflake {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string: String = Deserialize::deserialize(deserializer)?;
        Ok(Snowflake(string.parse().unwrap_or_default()))
    }
}
