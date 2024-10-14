use crate::entities::UserId;

#[derive(Clone, Debug)]
pub struct Key {
    id: i32,
    secret: String,
    owned_by: UserId,
}

impl Key {
    pub fn new(id: i32, secret: String, owned_by: UserId) -> Self {
        Key {
            id,
            secret,
            owned_by,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn secret(&self) -> &String {
        &self.secret
    }

    pub fn owned_by(&self) -> &UserId {
        &self.owned_by
    }
}