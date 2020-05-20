//! The bitfield of intents to use

use bitflags::bitflags;
use serde::{Serialize, Serializer};

bitflags! {
    pub struct Intent: u16 {
        /// - GUILD_CREATE
        /// - GUILD_UPDATE
        /// - GUILD_DELETE
        /// - GUILD_ROLE_CREATE
        /// - GUILD_ROLE_UPDATE
        /// - GUILD_ROLE_DELETE
        /// - CHANNEL_CREATE
        /// - CHANNEL_UPDATE
        /// - CHANNEL_DELETE
        /// - CHANNEL_PINS_UPDATE
        const GUILDS = 1 << 0;

        /// - GUILD_MEMBER_ADD
        /// - GUILD_MEMBER_UPDATE
        /// - GUILD_MEMBER_REMOVE
        const GUILD_MEMBERS = 1 << 1;

        /// - GUILD_BAN_ADD
        /// - GUILD_BAN_REMOVE
        const GUILD_BANS = 1 << 2;

        /// - GUILD_EMOJIS_UPDATE
        const GUILD_EMOJIS = 1 << 3;

        /// - GUILD_INTEGRATIONS_UPDATE
        const GUILD_INTEGRATIONS = 1 << 4;

        /// - WEBHOOKS_UPDATE
        const GUILD_WEBHOOKS = 1 << 5;

        /// - INVITE_CREATE
        /// - INVITE_DELETE
        const GUILD_INVITES = 1 << 6;

        /// - VOICE_STATE_UPDATE
        const GUILD_VOICE_STATES = 1 << 7;

        /// - PRESENCE_UPDATE
        const GUILD_PRESENCES = 1 << 8;

        /// - MESSAGE_CREATE
        /// - MESSAGE_UPDATE
        /// - MESSAGE_DELETE
        /// - MESSAGE_DELETE_BULK
        const GUILD_MESSAGES = 1 << 9;

        /// - MESSAGE_REACTION_ADD
        /// - MESSAGE_REACTION_REMOVE
        /// - MESSAGE_REACTION_REMOVE_ALL
        /// - MESSAGE_REACTION_REMOVE_EMOJI
        const GUILD_MESSAGE_REACTIONS = 1 << 10;

        /// - TYPING_START
        const GUILD_MESSAGE_TYPING = 1 << 11;

        /// - CHANNEL_CREATE
        /// - MESSAGE_CREATE
        /// - MESSAGE_UPDATE
        /// - MESSAGE_DELETE
        /// - CHANNEL_PINS_UPDATE
        const DIRECT_MESSAGES = 1 << 12;

        /// - MESSAGE_REACTION_ADD
        /// - MESSAGE_REACTION_REMOVE
        /// - MESSAGE_REACTION_REMOVE_ALL
        /// - MESSAGE_REACTION_REMOVE_EMOJI
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;

        /// - TYPING_START
        const DIRECT_MESSAGE_TYPING = 1 << 14;
    }
}

impl Serialize for Intent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.bits)
    }
}
