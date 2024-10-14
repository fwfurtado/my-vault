use crate::entities::user::User;
use crate::entities::UserId;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use support::auto_increment;
use support::auto_increment::Identity;

pub trait UserRepository: Send + Sync {
    fn save(&self, user: User) -> User;
    fn find_by_id(&self, id: i32) -> Option<User>;
    fn find_by_username(&self, username: &str) -> Option<User>;
}

pub fn new_user_repository() -> impl UserRepository {
    DefaultUserRepository {
        identity: auto_increment::Int::new(),
        table: Arc::new(RwLock::new(HashMap::new())),
        username_index: Arc::new(RwLock::new(HashMap::new())),
    }
}

struct DefaultUserRepository {
    identity: auto_increment::Int,
    table: Arc<RwLock<HashMap<UserId, User>>>,
    username_index: Arc<RwLock<HashMap<String, User>>>,
}

impl UserRepository for DefaultUserRepository {
    fn save(&self, user: User) -> User {
        let user = match user.id() {
            0 => {
                let id = self.identity.next();
                User::new(id, user.name().clone())
            }
            _ => user.clone(),
        };

        let mut table = self.table.write().unwrap();
        let mut username_index = self.username_index.write().unwrap();

        table.insert(user.id(), user.clone());
        username_index.insert(user.name().clone(), user.clone());

        user
    }

    fn find_by_id(&self, id: i32) -> Option<User> {
        let table = self.table.read().unwrap();
        table.get(&id).cloned()
    }

    fn find_by_username(&self, username: &str) -> Option<User> {
        let username_index = self.username_index.read().unwrap();
        username_index.get(username).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn should_save_user() {
        let repository = new_user_repository();
        let user = User::new(0, "user".to_string());

        let user = repository.save(user.clone());
        let id = user.id();

        let found = repository.find_by_id(id).unwrap();
        assert_eq!(found.name(), user.name());
        assert_eq!(found.id(), id);
    }

    #[rstest]
    fn should_find_user_by_id() {
        let repository = new_user_repository();
        let user = User::new(0, "user".to_string());

        let user = repository.save(user.clone());
        let id = user.id();

        let found = repository.find_by_id(id).unwrap();
        assert_eq!(found.name(), user.name());
    }

    #[rstest]
    fn should_find_user_by_username() {
        let repository = new_user_repository();
        let user = User::new(0, "user".to_string());

        repository.save(user.clone());

        let found = repository.find_by_username(user.name()).unwrap();
        assert_eq!(found.name(), user.name());
    }
}