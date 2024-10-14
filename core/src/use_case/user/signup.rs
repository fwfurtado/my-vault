use crate::domain::{Key, User};
use crate::gateway::key::KeyGateway;
use crate::gateway::user::UserGateway;
use crate::use_case::user::UserError;
use std::sync::Arc;

pub struct SignUp {
    user_gateway: Arc<dyn UserGateway>,
    keys_gateway: Arc<dyn KeyGateway>,
}


impl SignUp {
    pub fn new(
        user_gateway: Arc<dyn UserGateway>,
        keys_gateway: Arc<dyn KeyGateway>,
    ) -> Self {
        SignUp {
            user_gateway,
            keys_gateway,
        }
    }

    pub fn execute(&self, name: String) -> Result<User, UserError> {
        let user = match self.user_gateway.find_by_username(&name) {
            Some(_) => return Err(UserError::AlreadyExists("User already exists".to_string())),
            None => User::new(name),
        };


        let user = self.user_gateway.save(&user);

        let user_key = Key::random_for_user(user.clone());

        self.keys_gateway.save(&user_key);

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use database::repository::{KeyRepository, UserRepository};
    use mockall::mock;

    mock! {
        UserGatewayTesting{}
        impl UserGateway for UserGatewayTesting {
            fn save(&self, user: &User) -> User;
            fn find_by_id(&self, id: i32) -> Option<User>;
            fn find_by_username(&self, username: &str) -> Option<User>;
        }
    }

    mock! {
        KeyGatewayTesting{}
        impl KeyGateway for KeyGatewayTesting {
            fn save(&self, key: &Key);
            fn find_by_username(&self, username: String) -> Option<Key>;
        }
    }



    #[test]
    fn should_create_user() {
        let mut mock_user_gateway = MockUserGatewayTesting::new();

        mock_user_gateway.expect_find_by_username()
            .times(1)
            .return_const(None);

        mock_user_gateway.expect_save()
            .times(1)
            .returning(|user| user.clone());


        let mut mock_key_gateway = MockKeyGatewayTesting::new();

        mock_key_gateway.expect_save()
            .times(1)
            .return_const(());

        let user_gateway = Arc::new(mock_user_gateway);
        let key_gateway = Arc::new(mock_key_gateway);


        let signup = SignUp::new(user_gateway, key_gateway);
        let user = signup.execute("user".to_string()).expect("valid user");

        assert_eq!(user.username(), "user");
    }
}