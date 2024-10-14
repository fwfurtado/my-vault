use crate::domain::Secret;
use crate::use_case::secrets::{SecretError, SecretUseCase};

impl SecretUseCase {
    pub fn create_secret(
        &self,
        username: String,
        url: String,
        text: String,
    ) -> Result<i32, SecretError> {
        let key = match self.key_gateway.find_by_username(username.clone()) {
            Some(key) => key,
            None => return Err(SecretError::UserNotFound),
        };

        let secret = Secret::plain_text(url, text, key);

        let id = self.secret_gateway.save(&secret);

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Key, Secret, User};
    use crate::gateway::{KeyGateway, SecretGateway};
    use fake::faker::internet::en::{Password, Username};
    use fake::{Fake, Faker};
    use mockall::mock;
    use std::sync::Arc;

    mock! {
        KeyGatewayTesting {}
        impl KeyGateway for KeyGatewayTesting {
                fn save(&self, key: &Key);
                fn find_by_username(&self, username: String) -> Option<Key>;
        }
    }

    mock! {
        SecretGatewayTesting {}
        impl SecretGateway for SecretGatewayTesting {
            fn save(&self, secret: &Secret) -> i32;
        }
    }

    #[test]
    fn should_return_error_when_key_not_found() {
        let mut key_gateway = MockKeyGatewayTesting::new();
        let secret_gateway = MockSecretGatewayTesting::new();

        key_gateway
            .expect_find_by_username()
            .times(1)
            .return_const(None);

        let use_case = SecretUseCase::new(Arc::new(secret_gateway), Arc::new(key_gateway));

        let username = Username().fake();
        let plain_text_password = Password(16..32).fake();
        let url = Faker.fake::<url::Url>().to_string();

        let result = use_case.create_secret(username, url, plain_text_password);

        assert!(matches!(result, Err(SecretError::UserNotFound)));
    }

    #[test]
    fn should_create_a_new_secret() {
        let mut key_gateway = MockKeyGatewayTesting::new();
        let mut secret_gateway = MockSecretGatewayTesting::new();

        let username: String = Username().fake();
        let user_id: u16 = Faker.fake();

        let user = User::with(user_id, username.clone());

        let key = Key::random_for_user(user);

        key_gateway
            .expect_find_by_username()
            .times(1)
            .return_const(Some(key));

        let secret_id: i32 = Faker.fake::<u16>().into();
        secret_gateway
            .expect_save()
            .times(1)
            .return_const(secret_id);

        let use_case = SecretUseCase::new(Arc::new(secret_gateway), Arc::new(key_gateway));

        let plain_text_password = Password(16..32).fake();
        let url: url::Url = Faker.fake();
        let url_str = url.to_string();

        let result = use_case.create_secret(username.clone(), url_str, plain_text_password);

        assert!(matches!(result, Ok(n) if n == secret_id));
    }
}
