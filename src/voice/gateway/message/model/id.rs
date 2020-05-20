use derive_more::Into;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Into, Copy, Clone)]
pub struct SequenceID(u64);

#[derive(Debug, Serialize, Deserialize, Into, Copy, Clone)]
pub struct GuildID(Snowflake);

#[derive(Debug, Serialize, Deserialize, Into, Copy, Clone)]
pub struct ChannelId(Snowflake);

#[derive(Debug, Serialize, Deserialize, Into, Copy, Clone)]
pub struct ApplicationID(Snowflake);

#[derive(Debug, Serialize, Deserialize, Into, Copy, Clone)]
pub struct UserID(Snowflake);

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
// TODO: Custom ser and de as string
pub struct Snowflake(u64);
