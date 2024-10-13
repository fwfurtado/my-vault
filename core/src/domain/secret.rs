use crate::domain::Key;
use std::collections::HashSet;

pub struct Secret {
    id: i32,
    urls: HashSet<String>,
    value: String,
    key: Key,
}

impl Secret {
    pub fn new(id: i32, url: String, value: String, key: Key) -> Self {
        Secret {
            id,
            urls: HashSet::from([url]),
            value,
            key,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn urls(&self) -> &HashSet<String> {
        &self.urls
    }

    pub fn add_url(&mut self, url: String) {
        self.urls.insert(url);
    }

    pub fn has_url(&self, url: &str) -> bool {
        self.urls.contains(url)
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::User;
    use rstest::rstest;

    #[rstest]
    fn should_add_url_to_secret() {
        let user = User::new(1, "user".to_string());
        let key = Key::new(1, "secret".to_string(), user);
        let mut secret = Secret::new(1, "http://localhost".to_string(), "value".to_string(), key);
        secret.add_url("http://example.com".to_string());
        assert!(secret.has_url("http://example.com"));
    }
}