use serde::de::{Deserialize, Deserializer, Error};

use crate::types::*;

/// This object represents a Telegram user or bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: UserId,
    /// User‘s or bot’s first name.
    pub first_name: String,
    /// User‘s or bot’s last name.
    pub last_name: Option<String>,
    /// User‘s or bot’s username.
    pub username: Option<String>,
    /// True, if this user is a bot.
    pub is_bot: bool,
    /// IETF language tag of the user's language
    pub language_code: Option<String>,
}

/// This object represents a group.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct Group {
    /// Unique identifier for this chat.
    pub id: GroupId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: bool,
    /// Invite link for this group, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    pub invite_link: Option<String>,
}

/// This object represents a supergroup.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct Supergroup {
    /// Unique identifier for this chat.
    pub id: SupergroupId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// Username for supergroup.
    pub username: Option<String>,
    /// Invite link for this supergroup, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    pub invite_link: Option<String>,
}

/// This object represents a channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct Channel {
    /// Unique identifier for this chat.
    pub id: ChannelId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// Username for channel.
    pub username: Option<String>,
    /// Invite link for this channel, specific to this bot.
    /// You can generate a new invite link by using the
    /// export_invite_link method.
    pub invite_link: Option<String>,
}

/// This object represents a private, group or supergroup.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum MessageChat {
    Private(User),
    Group(Group),
    Supergroup(Supergroup),
    Channel(Channel),
    #[doc(hidden)]
    Unknown(RawChat),
}

impl MessageChat {
    pub fn id(&self) -> ChatId {
        match *self {
            MessageChat::Private(ref x) => x.id.into(),
            MessageChat::Group(ref x) => x.id.into(),
            MessageChat::Supergroup(ref x) => x.id.into(),
            MessageChat::Unknown(ref x) => x.id.into(),
            MessageChat::Channel(ref x) => x.id.into(),
        }
    }
}

/// This object represents a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Chat {
    Private(User),
    Group(Group),
    Supergroup(Supergroup),
    Channel(Channel),
    #[doc(hidden)]
    Unknown(RawChat),
}

impl Chat {
    pub fn id(&self) -> ChatId {
        match *self {
            Chat::Private(ref x) => x.id.into(),
            Chat::Group(ref x) => x.id.into(),
            Chat::Supergroup(ref x) => x.id.into(),
            Chat::Channel(ref x) => x.id.into(),
            Chat::Unknown(ref x) => x.id.into(),
        }
    }

    pub fn name(&self) -> String {
        match *self {
            Chat::Private(ref user) => {
                format!("{} {}", user.first_name, user.last_name.clone().unwrap_or_default())
                    .trim_end()
                    .into()
            }
            Chat::Group(ref group) => group.title.clone(),
            Chat::Supergroup(ref supergroup) => supergroup.title.clone(),
            Chat::Channel(ref channel) => channel.title.clone(),
            Chat::Unknown(ref raw_chat) => format!(
                "{} {}",
                raw_chat.first_name.clone().unwrap_or_default(),
                raw_chat.last_name.clone().unwrap_or_default()
            )
            .trim_end()
            .into(),
        }
    }
}

impl<'de> Deserialize<'de> for Chat {
    fn deserialize<D>(deserializer: D) -> Result<Chat, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawChat = Deserialize::deserialize(deserializer)?;

        macro_rules! required_field {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        }

        Ok(match raw.type_.as_ref() {
            "private" => Chat::Private(User {
                id: raw.id.into(),
                username: raw.username,
                first_name: required_field!(first_name),
                last_name: raw.last_name,
                is_bot: false,
                language_code: raw.language_code,
            }),
            "group" => Chat::Group(Group {
                id: raw.id.into(),
                title: required_field!(title),
                all_members_are_administrators: required_field!(all_members_are_administrators),
                invite_link: raw.invite_link,
            }),
            "supergroup" => Chat::Supergroup(Supergroup {
                id: raw.id.into(),
                title: required_field!(title),
                username: raw.username,
                invite_link: raw.invite_link,
            }),
            "channel" => Chat::Channel(Channel {
                id: raw.id.into(),
                title: required_field!(title),
                username: raw.username,
                invite_link: raw.invite_link,
            }),
            _ => Chat::Unknown(raw),
        })
    }
}

/// This object represents a chat, directly mapped.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct RawChat {
    /// Unique identifier for this chat.
    pub id: Integer,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub type_: String,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// Invite link for this chat, specific to this bot.
    /// Does not apply to private chats.
    pub invite_link: Option<String>,
    /// IETF language tag of the other party in a private chat
    pub language_code: Option<String>,
    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: Option<bool>,
}
