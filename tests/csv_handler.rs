#[cfg(test)]
mod tests {
    use url_shortener::csv_handler::UrlCsvFile;
    use url_shortener::url_handler::Url;

    #[test]
    fn reads_and_writes_data_to_file() {
        let mut csv_file = UrlCsvFile::new(Some("./tests/reads_and_writes_data_to_file.csv"));

        // get file discriptor
        csv_file.create();

        // read contents into urls vector
        csv_file.read_url();

        assert_eq!(csv_file.urls[0].url, "https://google.com".to_string());

        // initialize new url, to be added to csv file
        let mut url = Url::new("https://github.com".to_string());

        url.create_hash();

        // add new content to urls vector
        csv_file.urls.push(url);

        // write urls vector to file
        csv_file.write_url();

        csv_file.read_url();

        assert_eq!(csv_file.urls[1].url, "https://github.com".to_string());
    }

    #[test]
    fn writes_nothing_back_if_no_new_data_is_added() {
        let mut csv_file = UrlCsvFile::new(Some(
            "./tests/writes_nothing_back_if_no_new_data_is_added.csv",
        ));

        // get file discriptor
        csv_file.create();

        csv_file.read_url();

        assert_eq!(csv_file.urls[0].url, "https://google.com".to_string());

        // write urls vector to file
        csv_file.write_url();

        csv_file.read_url();

        assert_eq!(csv_file.urls.len(), 1);
    }
}
