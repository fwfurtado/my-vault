use crate::domain::Key;
use database::repository::KeyRepository;
use std::sync::Arc;

pub trait KeyGateway: Send + Sync {
    fn save(&self, key: &Key);
    fn find_by_username(&self, username: String) -> Option<Key>;
}

pub fn new_key_gateway(key_repository: Arc<dyn KeyRepository>) -> impl KeyGateway {
    DefaultKeyGateway {
        key_repository: Arc::clone(&key_repository)
    }
}

struct DefaultKeyGateway {
    key_repository: Arc<dyn KeyRepository>,
}

impl KeyGateway for DefaultKeyGateway {
    fn save(&self, key: &Key) {
        self.key_repository.save(key.clone().into());
    }

    fn find_by_username(&self, username: String) -> Option<Key> {
        self.key_repository.find_by_username(username.clone()).map(Key::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::User;
    use database::repository::KeyRepository;
    use fake::faker::internet::raw::UserAgent;
    use fake::locales::EN;
    use fake::Fake;
    use mockall::mock;
    use rstest::rstest;

    mock! {
        KeyRepositoryTesting{}
        impl KeyRepository for KeyRepositoryTesting {
            fn save(&self, key: database::entities::Key) -> i32;
            fn find_by_username(&self, username: String) -> Option<database::entities::Key>;
            fn find_all_by_user_id(&self, user_id: database::entities::UserId) -> Option<Vec<database::entities::Key>>;

        }
    }


    #[rstest]
    fn should_save_key() {
        let mut repository_mock = MockKeyRepositoryTesting::new();

        repository_mock.expect_save()
            .times(1)
            .return_const(0);

        let key = cuid::cuid2_slug();
        let username = UserAgent(EN).fake();

        let user = User::new(username);

        let gateway = new_key_gateway(Arc::new(repository_mock));

        gateway.save(&Key::new(key, user));
    }
}