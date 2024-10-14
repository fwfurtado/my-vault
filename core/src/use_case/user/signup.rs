use crate::domain::User;
use crate::gateway::UserGateway;
use crate::use_case::user::UserError;
use std::sync::Arc;

pub struct SignUp {
    user_gateway: Arc<UserGateway>,
}


impl SignUp {
    pub fn new(user_gateway: Arc<UserGateway>) -> Self {
        SignUp { user_gateway }
    }

    pub fn execute(&self, name: String) -> Result<User, UserError> {
        let user = match self.user_gateway.find_by_username(&name) {
            Some(_) => return Err(UserError::AlreadyExists("User already exists".to_string())),
            None => User::identity(name.clone()),
        };

        self.user_gateway.save(&user);

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn should_create_user() {
        let user_gateway = Arc::new(UserGateway::new());
        let signup = SignUp::new(Arc::clone(&user_gateway));
        let user = signup.execute("user".to_string()).expect("valid user");

        assert_eq!(user.username(), "user");
    }
}