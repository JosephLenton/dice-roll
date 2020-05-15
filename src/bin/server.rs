#![feature(box_syntax, box_patterns)]

use ::actix_web::{get, web, App, HttpServer, Responder};
use ::dice_roll;
use ::std::env;
use ::std::io;
use std::net::{SocketAddr, ToSocketAddrs};

const DEFAULT_PORT: u16 = 80;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let address = get_address();
    println!("Running on ... {}", address);
    HttpServer::new(|| App::new().service(index))
        .bind(address)?
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

fn get_address() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], get_port()))
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap()
}

fn get_port() -> u16 {
    match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => {
            println!("Couldn't find port, using ... {}", DEFAULT_PORT);
            DEFAULT_PORT
        }
    }
}
