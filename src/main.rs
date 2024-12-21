use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Hash, PartialEq, Eq)]
struct URL{
    url: String,
    hash: u64,
}

impl Default for URL{
    fn default() -> Self {
        URL{
            url: "".to_string(),
            hash: 0,
        }
    }
}

fn main() {
    // create new hasher
    let mut o_default_hasher_1 = DefaultHasher::new();
    let mut o_default_hasher_2 = DefaultHasher::new();

    let mut st_test_url_1: URL = url_builder("www.google.com".to_string());
    let mut st_test_url_2: URL = url_builder("www.github.com".to_string());
    
    // create hash
    st_test_url_1.hash(&mut o_default_hasher_1);
    st_test_url_2.hash(&mut o_default_hasher_2);
    
    st_test_url_1.hash = o_default_hasher_1.finish(); 
    st_test_url_2.hash = o_default_hasher_2.finish(); 
}


/// builds the url struct
fn url_builder(url: String) -> URL{
   URL { url, ..Default::default()} 
}
