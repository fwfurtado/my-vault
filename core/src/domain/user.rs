pub struct User {
    id: i32,
    username: String,
}

impl User {
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