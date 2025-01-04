use megumin::csv_handler::UrlCsvFile;
use megumin::url_handler::Url;

use actix_web::{
    get,
    web::{self, redirect},
    App, HttpServer, Responder,
};

#[get("/")]
async fn index() -> impl Responder {
    "<h1>Hello, World!<\\h1>"
}

#[get("/{url_hash}")]
async fn redirecter(url_hash: web::Path<String>) -> impl Responder {
    let mut redirect_url = String::new();

    let mut csv_file = UrlCsvFile::new(Some("./urls.csv"));

    // get file descriptor
    csv_file.create();

    csv_file.read_url();

    let mut new_url = Url::new("https://github.com/".to_string());

    new_url.create_hash();

    csv_file.urls.push(new_url);

    csv_file.write_url();

    csv_file.read_url();

    for url in csv_file.urls {
        if url.urlhash == format!("{}", url_hash) {
            redirect_url = url.url;
        }
    }

    redirect(format!("/{}", &url_hash), redirect_url)
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| App::new().service(index).service(redirecter))
        .bind(("localhost", 8080))
        .unwrap()
        .run()
        .await
        .unwrap()
}
