use actix_web::{get, post, HttpResponse, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/home")]
pub async fn home() -> impl Responder {
    NamedFile::open(PathBuf::from("static/index.html"))
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}