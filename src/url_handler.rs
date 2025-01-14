//! handles the hashing and saving of urls

use std::hash::{DefaultHasher, Hash, Hasher};

/// struct that saves the plaintext url and its hash
#[derive(Hash, PartialEq, Eq)]
pub struct Url {
    pub url: String,
    pub urlhash: String,
}

impl Url {
    /// makes a new Url instance
    pub fn new(url: String) -> Url {
        Url {
            url,
            ..Default::default()
        }
    }

    /// creates the hash of the saved url
    pub fn create_hash(&mut self) {
        let mut default_hasher: DefaultHasher = DefaultHasher::new();

        self.hash(&mut default_hasher);

        self.urlhash = format!("{}", default_hasher.finish());
    }
}

impl Default for Url {
    fn default() -> Self {
        Url {
            url: "".to_string(),
            urlhash: "".to_string(),
        }
    }
}
