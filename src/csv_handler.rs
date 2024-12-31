//! Handles csv file input and output

//use crate::url_handler::Url;
use crate::url_handler::Url;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::vec;

/// struct that contains the properties of the csv file that contains the url hashes
/// name: the name of the file
/// content the content of the file
pub struct UrlCsvFile<'a> {
    pub name: &'a str,
    pub urls: Vec<Url>,
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
    pub fn create(&mut self) {
        //TODO: this is not fine, pls handle that error correctly
        self.file = Some(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                //INFO: this option needs to be false, because if we read and the file is truncated to length 0
                //      when opened, then there is no data to be read, meaning there is no data to be written 
                //      back later
                .truncate(false)
                .open(self.name)
                .unwrap(),
        );
    }

    /// writes list of urls and url hashes to the csv file
    pub fn write_url(&mut self) {
        // check if file descriptor exists
        let mut file = match &self.file {
            Some(file) => file,
            None => panic!("please call 'create' before trying to write or read the csv data file"),
        };

        // prepare urls field for writing to file
        let contents: Vec<String> = self
            .urls
            .iter()
            .map(|url| {
                format! {"{},{}\n", url.urlhash, url.url}
            })
            .collect();

        for line in contents {
            file.write_all(line.as_bytes()).unwrap();
        }
    }

    /// reads the file and fills the url fields of the struct
    pub fn read_url(&mut self) {
        // check if file descriptor already exists
        let file = match &self.file {
            Some(file) => file,
            None => panic!("please call 'create' before trying to write or read the csv data file"),
        };

        // before reading, clear memory, so that stuff is overwritten instead of appended
        self.urls.clear();

        //let contents = BufReader::new(file);

        // parse csv file
        for line in read_to_string(self.name).unwrap().lines() {
            //let line = line.expect("Error: line could not be read");

            let linevec: Vec<&str> = line.split(',').collect();

            // remove ending new line char bevore further processing
            self.urls.push(Url {
                urlhash: linevec[0].to_string(),
                url: linevec[1].trim().to_string(),
            });
        }
    }
}

/// technicaly don't need that
impl Default for UrlCsvFile<'_> {
    fn default() -> Self {
        UrlCsvFile {
            name: "url_hash_data.csv",
            urls: vec![],
            file: None,
        }
    }
}
