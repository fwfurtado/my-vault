#[derive(Debug, Clone)]
pub struct User {
    id: Option<i32>,
    username: String,
}

impl From<database::entities::UserId> for User {
    fn from(user_id: database::entities::UserId) -> Self {
        User {
            id: Some(user_id),
            username: String::new(), //TODO: Not implemented yet. This will be hard to implement cause we don't have a way to get the username from the database at this point.
        }
    }
}

impl From<database::entities::User> for User {
    fn from(user: database::entities::User) -> Self {
        User {
            id: Some(user.id()),
            username: user.name().to_string(),
        }
    }
}

impl Into<database::entities::User> for User {
    fn into(self) -> database::entities::User {
        database::entities::User::new(self.id.unwrap_or(0), self.username)
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            username: "".to_string(),
        }
    }
}

impl User {
    pub fn with(id: u16, username: String) -> Self {
        User {
            id: Some(id.into()),
            username,
        }
    }

    pub fn new(username: String) -> Self {
        User { id: None, username }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}
