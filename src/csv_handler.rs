//! Handles csv file input and output

//use crate::url_handler::Url;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::vec;

/// struct that contains the properties of the csv file that contains the url hashes
/// name: the name of the file
/// content the content of the file
pub struct UrlCsvFile<'a> {
    pub name: &'a str,
    pub url_hashes: Vec<Cow<'a, str>>,
    pub url_plaintext: Vec<Cow<'a, str>>,
    file: Option<File>,
}

impl<'a> UrlCsvFile<'a> {
    /// makes a new CsvFile instance
    pub fn new(filename: Option<&'a str>) -> UrlCsvFile<'a> {
        match filename {
            Some(name) => UrlCsvFile {
                name,
                ..Default::default()
            },
            None => UrlCsvFile {
                ..Default::default()
            },
        }
    }

    /// creates the csv file
    pub fn create(&mut self) -> Result<bool, std::io::Error> {
        // check if file exists
        self.file = match File::open(self.name) {
            Ok(file) => Some(file),
            // if not create and open it
            Err(_) => {
                File::create(self.name)?;
                Some(File::open(self.name)?)
            }
        };

        Ok(true)
    }

    /// writes list of urls and url hashes to the csv file
    pub fn write_url(&mut self) {
        // file should already exist
        let mut file = match &self.file {
            Some(file) => file,
            None => panic!(
                "Error: please call 'create' first before trying to write to a non existing file"
            ),
        };

        // combine every element from url_hashes and url_plaintext, split by a ','
        let contents: Vec<String> = self
            .url_hashes
            .iter()
            .zip(self.url_plaintext.iter())
            .map(|(a, b)| format!("{},{}", a, b))
            .collect();

        for line in contents {
            // should be able to write file without problems
            writeln!(file, "{line}").unwrap();
        }
    }

    /// reads the file and fills the url fields of the struct
    pub fn read_url(&mut self) {
        // file should already exist
        let file = match &self.file {
            Some(file) => file,
            None => panic!(
                "Error: please call 'create' first before trying to read from a non existing file"
            ),
        };

        let contents = BufReader::new(file);

        // parse csv file
        for line in contents.lines() {
            let line = line.expect("Error: line could not be read");

            let linevec: Vec<&str> = line.split(',').collect();
            self.url_hashes.push(Cow::Owned(linevec[0].to_string()));

            // remove ending new line char bevore further processing
            self.url_plaintext
                .push(Cow::Owned(linevec[1].trim().to_string()));
        }
    }
}

/// technicaly don't need that
impl Default for UrlCsvFile<'_> {
    fn default() -> Self {
        UrlCsvFile {
            name: "url_hash_data.csv",
            url_hashes: vec![Cow::Borrowed("")],
            url_plaintext: vec![Cow::Borrowed("")],
            file: None,
        }
    }
}
