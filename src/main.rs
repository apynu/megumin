use url_shortener::csv_handler::UrlCsvFile;

use actix_web::{
    get,
    web::{self, redirect},
    App, HttpServer, Responder,
};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{url_hash}")]
async fn redirecter(url_hash: web::Path<String>) -> impl Responder {
    let mut redirect_url = String::new();

    let mut csv_file = UrlCsvFile::new(None);

    // get file descriptor
    csv_file.create();

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
