use crate::domain::User;

use database::repository::UserRepository;
use std::sync::Arc;

pub trait UserGateway: Send + Sync {
    fn save(&self, user: &User) -> User;
    fn find_by_id(&self, id: i32) -> Option<User>;
    fn find_by_username(&self, username: &str) -> Option<User>;
}

pub fn new_user_gateway(user_repository: Arc<dyn UserRepository>) -> impl UserGateway {
    DefaultUserGateway {
        user_repository: Arc::clone(&user_repository),
    }
}
struct DefaultUserGateway {
    user_repository: Arc<dyn UserRepository>,
}

impl UserGateway for DefaultUserGateway {
    fn save(&self, user: &User) -> User {
        let db_user = self.user_repository.save(user.clone().into());
        User::from(db_user)
    }

    fn find_by_id(&self, id: i32) -> Option<User> {
        self.user_repository.find_by_id(id).map(User::from)
    }

    fn find_by_username(&self, username: &str) -> Option<User> {
        self.user_repository
            .find_by_username(username)
            .map(User::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::name::en::Name;
    use fake::Fake;
    use mockall::mock;

    mock! {
        UserRepositoryTesting{}

        impl UserRepository for UserRepositoryTesting {
            fn save(&self, user: database::entities::User) -> database::entities::User;

            fn find_by_id(&self, id: i32) -> Option<database::entities::User>;


            fn find_by_username(&self, username: &str) -> Option<database::entities::User>;
        }
    }

    #[test]
    fn should_save_user() {
        let mut repository_mock = MockUserRepositoryTesting::new();

        repository_mock
            .expect_save()
            .times(1)
            .returning(|user| user);

        let repository = Arc::new(repository_mock);

        let user = User::new("user".to_string());
        let user_gateway = new_user_gateway(repository);
        let saved_user = user_gateway.save(&user);

        assert_eq!(saved_user.username(), user.username());
    }

    #[test]
    fn should_find_user_by_username() {
        let username: String = Name().fake();

        let mut repository_mock = MockUserRepositoryTesting::new();

        repository_mock
            .expect_find_by_username()
            .times(1)
            .returning(move |username| {
                Some(database::entities::User::new(1, username.to_string()))
            });

        let user_repository = Arc::new(repository_mock);
        let user_gateway = new_user_gateway(user_repository);

        let saved_user = user_gateway.find_by_username(&username).unwrap();
        assert_eq!(saved_user.username(), &username);
    }
}
