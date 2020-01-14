pub struct Session{
    token: String,
    user_id: Option<u64>,

}

impl Session{
    pub fn from_token(token: &str) -> Self {
        return Session{
            token: token.to_string(),
            user_id: None,

        }
    }

    pub fn new(token: String, user_id: Option<u64>) -> Self {
        return Session{token, user_id}
    }

    pub fn token(&self) -> String{
        self.token.clone()
    }
}





