//! Handles csv file input and output

use std::file;
use std::vec;

/// struct that contains the properties of the csv file that contains the url hashes
/// name: the name of the file
/// content the content of the file
pub struct UrlCsvFile<'a>{
    pub name: &'a str,
    pub url_hashes: Vec<&'a str>,
    pub url_plaintext: Vec<&'a str>,

}

impl<'a> UrlCsvFile<'a>{

    /// makes a new CsvFile instance
    pub fn new(filename: Option<&'a str>) -> UrlCsvFile<'a>{
        match filename{
           Some(name) => UrlCsvFile{ name, ..Default::default()},
           None => UrlCsvFile { ..Default::default() },
        }
    }
    
    /// creates the csv file
    pub fn create(&self) -> Result<bool, std::io::Error>{
        todo!("")
    }
    
    /// writes a new url to the csv file
    pub fn write_url(&self){
       todo!("") 
    }
}

/// technicaly don't need that
impl Default for UrlCsvFile<'_>{
    fn default() -> Self {
        UrlCsvFile{ name: "url_hash_data.csv", url_hashes: vec![""], url_plaintext: vec![""] }
    }    
}