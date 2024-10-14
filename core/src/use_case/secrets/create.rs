use crate::use_case::secrets::{SecretError, SecretsUseCase};

impl SecretsUseCase {
    #[allow(unused_variables)]
    pub fn create_secret(&self, url: String, text: String) -> Result<String, SecretError> {
        Err(SecretError::NotFoundUrl)
    }
}