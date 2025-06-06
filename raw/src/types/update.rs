use crate::types::*;

/// This object represents an incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially.
    #[serde(rename = "update_id")]
    pub id: Integer,
    /// Kind of the incoming update.
    #[serde(flatten)]
    pub kind: UpdateKind,
}

impl Update {
    pub fn chat_id(&self) -> Option<ChatId> {
        self.kind.chat_id()
    }

    pub fn from(&self) -> Option<UserId> {
        self.kind.from()
    }
}

/// Kind of the incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    #[serde(rename = "message")]
    Message(Message),
    /// New version of a message that is known to the bot and was edited
    #[serde(rename = "edited_message")]
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    #[serde(rename = "channel_post")]
    ChannelPost(ChannelPost),
    /// New version of a channel post that is known to the bot and was edited
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost(ChannelPost),
    #[serde(rename = "inline_query")]
    InlineQuery(InlineQuery),
    #[serde(rename = "chosen_inline_result")]
    ChosenInlineResult(ChosenInlineResult),
    #[serde(rename = "callback_query")]
    CallbackQuery(CallbackQuery),
    #[serde(rename = "shipping_query")]
    ShippingQuery(ShippingQuery),
    #[serde(rename = "pre_checkout_query")]
    PreCheckoutQuery(PreCheckoutQuery),
    /// New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    #[serde(rename = "poll")]
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself
    #[serde(rename = "poll_answer")]
    PollAnswer(PollAnswer),
    #[serde(rename = "my_chat_member")]
    MyChatMemberUpdated(ChatMemberUpdated),
    #[serde(rename = "chat_member")]
    ChatMemberUpdated(ChatMemberUpdated),
    #[serde(rename = "chat_join_request")]
    ChatJoinRequest(ChatJoinRequest),
    #[doc(hidden)]
    Error(String),
    #[doc(hidden)]
    Unknown,
}

impl UpdateKind {
    pub fn chat_id(&self) -> Option<ChatId> {
        match self {
            UpdateKind::Message(msg) => Some(msg.chat.id()),
            UpdateKind::EditedMessage(msg) => Some(msg.chat.id()),
            UpdateKind::ChannelPost(post) => Some(post.chat.id.into()),
            UpdateKind::EditedChannelPost(post) => Some(post.chat.id.into()),
            UpdateKind::CallbackQuery(callback_query) => { 
                match callback_query.message {
                    Some(ref msg) => Some(msg.chat_id()),
                    None => None,
                }
            },
            UpdateKind::ChatJoinRequest(request) => Some(request.chat.id()),
            _ => None,
        }
    }

    pub fn from(&self) -> Option<UserId> {
        match self {
            UpdateKind::Message(msg) => Some(msg.from()?.id),
            UpdateKind::EditedMessage(msg) => Some(msg.from()?.id),
            UpdateKind::CallbackQuery(query) => Some(query.from.id),
            _ => None,
        }
    }
 }