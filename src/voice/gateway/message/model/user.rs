use super::id::Snowflake;
use bitflags::bitflags;
use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// The user's id
    id: Snowflake,
    /// The user's username, not unique across the platform
    username: String,
    /// the user's 4-digit discord-tag
    discriminator: String,
    /// the user's avatar hash
    avatar: Option<String>,
    /// whether the user belongs to an OAuth2 application
    bot: bool,
    /// whether the user is an Official Discord System user (part of the urgent message system)
    system: bool,
    /// whether the user has two factor enabled on their account
    mfa_enabled: bool,
    /// the user's chosen language option
    locale: String,
    /// whether the email on this account has been verified	email
    verified: bool,
    /// the user's email
    email: Option<String>,
    /// the flags on a user's account
    flags: UserFlags,
    /// the type of Nitro subscription on a user's account
    premium_type: PremiumType,
    /// the public flags on a user's account
    public_flags: UserFlags,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}

bitflags! {
    pub struct UserFlags: u32 {
        const NONE                      = 0;
        const DISCORD_EMPLOYEE          = 1 << 0;
        const DISCORD_PARTNER           = 1 << 1;
        const HYPESQUAD_EVENTS          = 1 << 2;
        const BUG_HUNTER_LEVEL_1        = 1 << 3;
        const HOUSE_BRAVERY             = 1 << 6;
        const HOUSE_BRILLIANCE          = 1 << 7;
        const HOUSE_BALANCE             = 1 << 8;
        const EARLY_SUPPORTER           = 1 << 9;
        const TEAM_USER                 = 1 << 10;
        const SYSTEM                    = 1 << 12;
        const BUG_HUNTER_LEVEL_2        = 1 << 14;
        const VERIFIED_BOT              = 1 << 16;
        const VERIFIED_BOT_DEVELOPER    = 1 << 17;
    }
}

impl Serialize for UserFlags {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(self.bits)
    }
}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(UserFlags::from_bits_truncate(Deserialize::deserialize(deserializer)?))
    }
}
