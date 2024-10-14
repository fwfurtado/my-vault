#[derive(Clone, Debug)]
pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        User {
            id,
            name,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}