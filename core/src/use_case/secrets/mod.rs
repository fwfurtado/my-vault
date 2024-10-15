use crate::gateway::secret::SecretGateway;
use crate::gateway::KeyGateway;
use std::sync::Arc;

mod create;
mod show;

#[derive(Debug, PartialEq)]
pub enum SecretError {
    SecretNotFound,
    UserNotFound,
}

pub struct SecretUseCase {
    secret_gateway: Arc<dyn SecretGateway>,
    key_gateway: Arc<dyn KeyGateway>,
}

impl SecretUseCase {
    pub fn new(secret_gateway: Arc<dyn SecretGateway>, key_gateway: Arc<dyn KeyGateway>) -> Self {
        SecretUseCase {
            secret_gateway: Arc::clone(&secret_gateway),
            key_gateway: Arc::clone(&key_gateway),
        }
    }
}
