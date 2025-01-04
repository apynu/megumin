# Megumin

![logo depicting konosuba character megumin in a magical circle holding her staff](https://github.com/apynu/megumin/blob/main/meta/logo.png?raw=true)

# WARNING: NOTHING IS IMPLEMENTED YET

Megumin is a url shortener for home use. It is fully customizeable and comes with a web-page that acts as user interface.
And yes the name is based on the fictional character Megumin from the anime konosuba (highly recommended btw)
### Features
- Fully customizeable (which open source program isn't? ^^)
- Web interface
- custom shortened links
- hashed shortened links
- links are stored in an easily accessible `csv` file
- straight forward setup with the `serverconfig.toml` file

### Installation
There are two ways of installing Megumin

##### 1. Online Installation
1. Get only the Binary from GitHub 
2. Run the binary via `./megumin`. This will do two things:
	1. Install the `index.html` file from GitHub and serve via the web-server
	2. Create a `urls.csv` file 

##### 2. Offline Installation
1. Get the `megumin` binary and the `index.html` file from GitHub
2. You have to define where you want `megumin` to look for the `index.html` file, default path in `serverconfig.toml` is: `./index.html`

#### serverconfig.toml
The `serverconfig.toml` is the config file for `megumin` (duh).
It contains three fields:
1. `serverip`: which IP-address the server should use, Default: `localhost`
2. `serverport`: which port the server should use, Default: `8080`
3. `index_file_path`: where the `index.html` file is stored, Default: `./index.html`

## Advanced
Here some stuff for the advanced user that wants to do more coding

### CSV file syntax
The syntax is **really** simple:
```csv
shortendurl1,fullurl1
shortendurl2,fullurl2
```
For Example:
```csv
github,https://github.com
google,https://duckduckgo.com
nextcloud,http://192.168.2.123:80
```

### TODO:
- [ ] Basically everything that is discussed here xD
- [ ] for more privacy focused people, the url and urlhash could be saved only to memory instead of to disk
- [ ] remove `basic program flow`

### basic program flow
- give url in webview
	- can create custom shortener link for a given url
	- can also choose to hash (even though i don't really need that)
	- there is a list of all used url shortener links
- if a new link with the identical name is made, then an exception will be thrown
- ~~url is hashed and a url-shortener link with the hashed url at the end is returneda~~
- url is stored right next to the url-shortener link in a csv file
- if url-shortener link is pressed, the program checks if the link is in the csv file, if so it redirects to the corresponding full-url-link

