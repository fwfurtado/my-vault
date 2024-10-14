use std::sync::atomic::AtomicI32;

static IDENTITY: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone)]
pub struct User {
    id: i32,
    username: String,
}

impl User {
    pub fn identity(username: String) -> Self {
        let id = IDENTITY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        User {
            id,
            username,
        }
    }

    pub fn new(id: i32, username: String) -> Self {
        User {
            id,
            username,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}