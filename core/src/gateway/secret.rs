use crate::domain::Secret;
use database::repository::SecretRepository;
use std::sync::Arc;

pub trait SecretGateway: Send + Sync {
    fn save(&self, secret: &Secret) -> i32;
    fn find_by_url(&self, url: String) -> Option<Secret>;
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
    fn save(&self, secret: &Secret) -> i32 {
        self.secret_repository.save(secret.clone().into())
    }

    fn find_by_url(&self, url: String) -> Option<Secret> {
        self.secret_repository
            .find_by_url(url)
            .map(|secret| Secret::from(secret))
    }
}
