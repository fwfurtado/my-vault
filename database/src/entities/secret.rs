use crate::entities::KeyId;

#[derive(Clone, Debug)]
pub struct Secret {
    id: i32,
    encrypted_value: String,
    key_id: KeyId,
    url: String,
}


impl Secret {
    pub fn new(id: i32, url: String, encrypted_value: String, key_id: KeyId) -> Self {
        Secret {
            id,
            encrypted_value,
            key_id,
            url,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn encrypted_value(&self) -> &String {
        &self.encrypted_value
    }

    pub fn key_id(&self) -> KeyId {
        self.key_id
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}