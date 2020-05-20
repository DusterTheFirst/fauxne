use super::id::UserID;
use bitflags::bitflags;
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
pub struct User {
    /// The user's id
    pub id: UserID,
    /// The user's username, not unique across the platform
    pub username: String,
    /// the user's 4-digit discord-tag
    pub discriminator: String,
    /// the user's avatar hash
    pub avatar: Option<String>,
    /// whether the user belongs to an OAuth2 application
    pub bot: Option<bool>,
    /// whether the user is an Official Discord System user (part of the urgent message system)
    pub system: Option<bool>,
    /// whether the user has two factor enabled on their account
    pub mfa_enabled: Option<bool>,
    /// the user's chosen language option
    pub locale: Option<String>,
    /// whether the email on this account has been verified	email
    pub verified: Option<bool>,
    /// the user's email
    pub email: Option<String>,
    /// the flags on a user's account
    pub flags: Option<UserFlags>,
    /// the type of Nitro subscription on a user's account
    pub premium_type: Option<PremiumType>,
    /// the public flags on a user's account
    pub public_flags: Option<UserFlags>,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(UserFlags::from_bits_truncate(Deserialize::deserialize(
            deserializer,
        )?))
    }
}
