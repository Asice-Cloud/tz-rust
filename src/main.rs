use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use router::{hello, echo, manual_hello};
use crate::router::home;
mod router;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logging)
            .service(hello)
            .service(echo)
            .service(home)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("0.0.0.0", 8088))?
        .run()
        .await
}