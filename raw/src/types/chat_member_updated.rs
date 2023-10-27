use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
    pub via_chat_folder_invite_link: Option<bool>
}