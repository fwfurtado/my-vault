use crate::gateway::secret::SecretGateway;
use std::sync::Arc;

mod create;

#[derive(Debug)]
pub enum SecretError {
    NotFoundUrl,
}

pub struct SecretsUseCase {
    secret_gateway: Arc<dyn SecretGateway>,
}