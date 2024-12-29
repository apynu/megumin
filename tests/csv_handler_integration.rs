#[cfg(test)]
mod tests {
    //use super::*;
    //use std::borrow::Cow;
    use url_shortener::csv_handler::UrlCsvFile;
    use url_shortener::url_handler::Url;

    #[test]
    fn test_csv_handler() {
        let mut csv_file = UrlCsvFile::new(Some("test.csv"));
        
        // get file discriptor
        csv_file.create();

        // read contents into urls vector
        csv_file.read_url();

        let mut url = Url::new("https://google.com".to_string());

        url.create_hash();

        // add new content to urls vector
        csv_file.urls.push(url);

        // write urls vector to file
        csv_file.write_url();

        //assert_eq!(csv_file.urls[0].urlhash, );
        assert_eq!(csv_file.urls[0].url, "https://google.com".to_string());
    }
}
