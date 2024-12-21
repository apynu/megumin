
/// struct that saves the plaintext url and its hash
#[derive(Hash, PartialEq, Eq)]
pub struct URL{
    pub url: String,
    pub hash: u64,
}

impl Default for URL{
    fn default() -> Self {
        URL{
            url: "".to_string(),
            hash: 0,
        }
    }
}

/// builds the url struct
/// the hash field is default initialized
pub fn builder(url: String) -> URL{
   URL { url, ..Default::default()} 
}
