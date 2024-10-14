use crate::domain::User;
use std::sync::atomic::AtomicI32;

static IDENTITY: AtomicI32 = AtomicI32::new(0);

#[derive(Clone)]
pub struct Key {
    id: Option<i32>,
    encrypted_value: String,
    owned_by: User,
}

impl From<database::entities::Key> for Key {
    fn from(key: database::entities::Key) -> Self {
        Key {
            id: Some(key.id()),
            encrypted_value: key.secret().to_string(),
            owned_by: User::from(key.owned_by().clone()),
        }
    }
}

impl Into<database::entities::Key> for Key {
    fn into(self) -> database::entities::Key {
        database::entities::Key::new(
            self.id.unwrap_or(0),
            self.encrypted_value,
            self.owned_by.clone().id().unwrap(),
        )
    }
}


impl Into<database::entities::KeyId> for Key {
    fn into(self) -> database::entities::KeyId {
        self.id.unwrap_or(0)
    }
}

impl Key {
    pub fn random_for_user(user: User) -> Self {
        Key {
            id: None,
            encrypted_value: cuid::cuid2_slug(),
            owned_by: user,
        }
    }

    pub fn new(secret: String, user: User) -> Self {
        Key {
            id: None,
            encrypted_value: secret,
            owned_by: user,
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn secret(&self) -> &String {
        &self.encrypted_value
    }

    pub fn owned_by(&self) -> &User {
        &self.owned_by
    }

    pub fn encode(&self, text: String) -> String {
        format!("{}:{}", self.encrypted_value, text)
    }

    pub fn decode(&self, text: String) -> Option<String> {
        text.strip_prefix(
            format!("{}:", &self.encrypted_value).as_str()
        ).map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn should_encode_text_with_secret() {
        let user = User::default();
        let key = Key::new("secret".to_string(), user);
        let encoded = key.encode("text".to_string());
        assert_eq!(encoded, "secret:text");
    }

    #[rstest]
    fn should_decode_text_with_secret() {
        let user = User::default();
        let key = Key::new("secret".to_string(), user);
        let decoded = key.decode("secret:text".to_string());
        assert_eq!(decoded, Some("text".to_string()));
    }

    #[rstest]
    fn should_return_none_when_decoding_text_without_secret() {
        let user = User::default();
        let key = Key::new("secret".to_string(), user);
        let decoded = key.decode("text".to_string());
        assert_eq!(decoded, None);
    }
}