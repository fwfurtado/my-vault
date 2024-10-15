use crate::domain::Secret;
use crate::use_case::{SecretError, SecretUseCase};

impl SecretUseCase {
    pub fn show_secret(&self, url: String) -> Result<Secret, SecretError> {
        match self.secret_gateway.find_by_url(url.clone()) {
            Some(secret) => Ok(secret.clone()),
            None => Err(SecretError::SecretNotFound),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::{Key, Secret, User};
    use crate::gateway::{KeyGateway, SecretGateway};
    use fake::faker::internet::en::Username;
    use fake::{Fake, Faker};
    use mockall::mock;
    use mockall::predicate::*;
    use std::sync::Arc;

    mock! {
        SecretGatewayTesting {}
        impl SecretGateway for SecretGatewayTesting {
            fn save(&self, secret: &Secret) -> i32;
            fn find_by_url(&self, url: String) -> Option<Secret>;
        }
    }

    mock! {
        KeyGatewayTesting {}
        impl KeyGateway for KeyGatewayTesting {
            fn save(&self, key: &Key);
            fn find_by_username(&self, username: String) -> Option<Key>;
        }
    }

    #[test]
    fn should_return_secret() {
        let user_id: u16 = Faker.fake();
        let username: String = Username().fake();
        let user = User::with(user_id, username.clone());

        let secret = Secret::new(
            "https://example.com".to_string(),
            "value".to_string(),
            Key::random_for_user(user.clone()),
        );

        let key_gateway = MockKeyGatewayTesting::new();
        let mut secret_gateway = MockSecretGatewayTesting::new();

        secret_gateway
            .expect_find_by_url()
            .times(1)
            .with(eq("https://example.com".to_string()))
            .return_const(Some(secret.clone()));

        let use_case = SecretUseCase::new(Arc::new(secret_gateway), Arc::new(key_gateway));

        let result = use_case.show_secret("https://example.com".to_string());

        assert_eq!(result, Ok(secret));
    }

    #[test]
    fn should_return_error_when_secret_not_found() {
        let key_gateway = MockKeyGatewayTesting::new();
        let mut secret_gateway = MockSecretGatewayTesting::new();

        secret_gateway
            .expect_find_by_url()
            .times(1)
            .with(eq("https://example.com".to_string()))
            .return_const(None);

        let use_case = SecretUseCase::new(Arc::new(secret_gateway), Arc::new(key_gateway));

        let result = use_case.show_secret("https://example.com".to_string());

        assert_eq!(result, Err(SecretError::SecretNotFound));
    }
}
