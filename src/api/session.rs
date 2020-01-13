pub struct Session{
    token: String,
    user_id: Option<u64>,
    version: String,
}

impl Session{
    pub fn from_token(token: &str) -> Self {
        return Session{
            token: token.to_string(),
            user_id: None,
            version: "5.91".to_string()
        }
    }

    pub fn new(token: String, version: String) -> Self {
        return Session{token, user_id: None, version}
    }
}





