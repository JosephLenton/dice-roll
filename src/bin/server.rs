#![feature(box_syntax, box_patterns)]

use ::actix_web::{get, web, App, HttpServer, Responder};
use ::dice_roll;
use ::std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/{input}")]
async fn index(info: web::Path<String>) -> impl Responder {
    let username = "You";
    let mut response: Vec<u8> = Vec::new();

    dice_roll::main(&username, &info, &mut response);

    String::from_utf8(response).unwrap()
}
