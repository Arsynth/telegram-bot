use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ApproveChatJoinRequest {
    chat_id: ChatId,
    user_id: UserId
}

impl ApproveChatJoinRequest {
    pub fn new<U>(chat_id: ChatId, user_id: U) -> Self
    where U: ToUserId,
    {
        Self {
            chat_id,
            user_id: user_id.to_user_id(),
        }
    }
}

impl Request for ApproveChatJoinRequest {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("approveChatJoinRequest"), self)
    }
}