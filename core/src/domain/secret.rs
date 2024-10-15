use crate::domain::Key;

#[derive(PartialEq, Debug, Clone)]
pub struct Secret {
    id: Option<i32>,
    url: String,
    value: String,
    key: Key,
}

impl Into<database::entities::Secret> for Secret {
    fn into(self) -> database::entities::Secret {
        database::entities::Secret::new(
            self.id.unwrap_or(0),
            self.url,
            self.value,
            self.key.clone().into(),
        )
    }
}

impl From<database::entities::Secret> for Secret {
    fn from(secret: database::entities::Secret) -> Self {
        Secret {
            id: Some(secret.id()),
            url: secret.url().to_string(),
            value: secret.encrypted_value().clone(),
            key: Key::from(secret.key_id()),
        }
    }
}

impl Secret {
    pub fn plain_text(url: String, text: String, key: Key) -> Self {
        let encode = key.encode(text);

        Secret {
            id: None,
            url,
            value: encode,
            key,
        }
    }

    pub fn new(url: String, value: String, key: Key) -> Self {
        Secret {
            id: None,
            url,
            value,
            key,
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}
