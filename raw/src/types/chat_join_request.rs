use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub name: Option<String>,
    pub expire_date: Option<i64>,
    pub member_limit: Option<i64>,
    pub pending_join_request_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub user_chat_id: ChatId,
    /// Date the request was sent in Unix time
    pub date: i64,
    pub bio: Option<String>,
    pub invite_link: Option<ChatInviteLink>
}