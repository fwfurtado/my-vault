use crate::domain::Secret;
use database::repository::SecretRepository;
use std::sync::Arc;

pub trait SecretGateway: Send + Sync {
    fn save(&self, secret: &Secret);
}

pub fn new_secret_gateway(secret_repository: Arc<dyn SecretRepository>) -> impl SecretGateway {
    DefaultSecretGateway {
        secret_repository: Arc::clone(&secret_repository),
    }
}


struct DefaultSecretGateway {
    secret_repository: Arc<dyn SecretRepository>,
}

impl SecretGateway for DefaultSecretGateway {
    fn save(&self, secret: &Secret) {
        self.secret_repository.save(secret.clone().into());
    }
}