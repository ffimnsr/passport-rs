
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String,
}

pub struct Chat {
    pub id: i64,
    pub chat_type: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub struct Message {
    pub message_id: i64,
    pub date: i64,
    pub text: Option<String>,
    pub chat: Chat,
    pub from: User,
}
