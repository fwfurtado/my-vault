use crate::domain::User;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct UserGateway {
    users: Arc<RwLock<HashMap<i32, User>>>,
    username_index: Arc<RwLock<HashMap<String, User>>>,
}

impl UserGateway {
    pub fn new() -> Self {
        let users = HashMap::new();
        let username_index = HashMap::new();

        UserGateway {
            users: Arc::new(RwLock::new(users)),
            username_index: Arc::new(RwLock::new(username_index)),
        }
    }

    pub fn save(&self, user: &User) {
        let username = user.username().clone();
        let id = user.id();

        let mut table = self.users.write().unwrap();
        table.insert(id, user.clone());

        let mut index = self.username_index.write().unwrap();
        index.insert(username, user.clone());
    }

    pub fn find_by_id(&self, id: i32) -> Option<User> {
        self.users.read().unwrap().get(&id).cloned()
    }

    pub fn find_by_username(&self, username: &str) -> Option<User> {
        self.username_index.read().unwrap().get(username).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn should_save_user() {
        let user = User::identity("user".to_string());
        let user_gateway = UserGateway::new();
        user_gateway.save(&user);

        let saved_user = user_gateway.find_by_id(user.id()).unwrap();
        assert_eq!(saved_user.username(), "user");
    }

    #[rstest]
    fn should_find_user_by_username() {
        let user = User::identity("user".to_string());
        let user_gateway = UserGateway::new();
        user_gateway.save(&user);

        let saved_user = user_gateway.find_by_username("user").unwrap();
        assert_eq!(saved_user.username(), "user");
    }
}