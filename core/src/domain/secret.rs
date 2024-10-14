use crate::domain::Key;


#[derive(Clone)]
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

impl Secret {
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