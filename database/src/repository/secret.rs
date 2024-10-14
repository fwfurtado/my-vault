use crate::entities::secret::Secret;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use support::auto_increment;
use support::auto_increment::Identity;

pub trait SecretRepository: Send + Sync {
    fn save(&self, secret: Secret) -> i32;
    fn find_by_url(&self, url: String) -> Option<Secret>;
}

pub fn new_secret_repository() -> impl SecretRepository {
    DefaultSecretRepository {
        identity: auto_increment::Int::new(),
        table: Arc::new(RwLock::new(HashMap::new())),
    }
}

struct DefaultSecretRepository {
    identity: auto_increment::Int,
    table: Arc<RwLock<HashMap<String, Secret>>>,
}

impl SecretRepository for DefaultSecretRepository {
    fn save(&self, secret: Secret) -> i32 {
        let secret = match secret.id() {
            0 => {
                let id = self.identity.next();
                Secret::new(id, secret.url().clone(), secret.encrypted_value().clone(), secret.key_id().clone())
            }
            _ => secret.clone(),
        };

        let mut table = self.table.write().unwrap();
        table.insert(secret.url().clone(), secret.clone());

        secret.id()
    }
    fn find_by_url(&self, url: String) -> Option<Secret> {
        let table = self.table.read().unwrap();
        table.get(&url).cloned()
    }
}