# url_shortener
url_shortener that i used to get back into rust

## TODO:
- [x] hash urls
- [ ] csv file handling
- [ ] webserver/redirectserver
- [ ] website

## basic program flow
- give url in webview
- url is hashed and a url-shortener link with the hashed url at the end is returned
- url is stored right next to the url-shortener link in a csv file
- if url-shortener link is pressed, the program checks if the link is in the csv file, if so it redirects to the corresponding full-url-link