use crate::entities::key::Key;
use crate::entities::UserId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use support::auto_increment;
use support::auto_increment::Identity;


pub trait KeyRepository: Send + Sync {
    fn save(&self, key: Key) -> i32;
    fn find_all_by_user_id(&self, user_id: UserId) -> Option<Vec<Key>>;
    fn find_by_username(&self, username: String) -> Option<Key>;
}

pub fn new_key_repository() -> impl KeyRepository {
    DefaultKeyRepository {
        identity: auto_increment::Int::new(),
        table: Arc::new(RwLock::new(HashMap::new())),
        username_index: Arc::new(RwLock::new(HashMap::new())),
    }
}

struct DefaultKeyRepository {
    identity: auto_increment::Int,
    table: Arc<RwLock<HashMap<UserId, Vec<Key>>>>,
    username_index: Arc<RwLock<HashMap<String, Key>>>,
}

impl KeyRepository for DefaultKeyRepository {
    fn save(&self, key: Key) -> i32 {
        let key = match key.id() {
            0 => {
                let id = self.identity.next();
                Key::new(id, key.secret().clone(), key.owned_by().clone())
            }
            _ => key.clone(),
        };

        let mut table = self.table.write().unwrap();
        let mut username_index = self.username_index.write().unwrap();

        let user_id = key.owned_by();
        let keys = table.entry(*user_id).or_insert(Vec::new());
        keys.push(key.clone());

        username_index.insert(key.secret().clone(), key.clone());

        key.id()
    }

    fn find_all_by_user_id(&self, user_id: UserId) -> Option<Vec<Key>> {
        let table = self.table.read().unwrap();
        table.get(&user_id).cloned()
    }

    fn find_by_username(&self, username: String) -> Option<Key> {
        let username_index = self.username_index.read().unwrap();
        username_index.get(&username).cloned()
    }
}