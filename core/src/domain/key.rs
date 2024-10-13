use crate::domain::User;

pub struct Key {
    id: i32,
    secret: String,
    owned_by: User,
}


impl Key {
    pub fn new(id: i32, secret: String, user: User) -> Self {
        Key {
            id,
            secret,
            owned_by: user,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn secret(&self) -> &String {
        &self.secret
    }

    pub fn owned_by(&self) -> &User {
        &self.owned_by
    }

    pub fn encode(&self, text: String) -> String {
        format!("{}:{}", self.secret, text)
    }

    pub fn decode(&self, text: String) -> Option<String> {
        text.strip_prefix(
            format!("{}:", &self.secret).as_str()
        ).map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn should_encode_text_with_secret() {
        let user = User::new(1, "user".to_string());
        let key = Key::new(1, "secret".to_string(), user);
        let encoded = key.encode("text".to_string());
        assert_eq!(encoded, "secret:text");
    }

    #[rstest]
    fn should_decode_text_with_secret() {
        let user = User::new(1, "user".to_string());
        let key = Key::new(1, "secret".to_string(), user);
        let decoded = key.decode("secret:text".to_string());
        assert_eq!(decoded, Some("text".to_string()));
    }

    #[rstest]
    fn should_return_none_when_decoding_text_without_secret() {
        let user = User::new(1, "user".to_string());
        let key = Key::new(1, "secret".to_string(), user);
        let decoded = key.decode("text".to_string());
        assert_eq!(decoded, None);
    }
}