pub mod s_url;

use std::hash::{Hash, Hasher, DefaultHasher};

fn main() {
    // create new hasher
    let mut o_default_hasher_1 = DefaultHasher::new();
    let mut o_default_hasher_2 = DefaultHasher::new();

    let mut st_test_url_1: s_url::URL = s_url::builder("www.google.com".to_string());
    let mut st_test_url_2: s_url::URL = s_url::builder("www.github.com".to_string());
    
    // create hash
    st_test_url_1.hash(&mut o_default_hasher_1);
    st_test_url_2.hash(&mut o_default_hasher_2);
    
    st_test_url_1.hash = o_default_hasher_1.finish(); 
    st_test_url_2.hash = o_default_hasher_2.finish(); 
}