#![feature(box_syntax, box_patterns)]
#![feature(proc_macro_hygiene, decl_macro)]

use ::dice_roll;
use ::rocket;
use ::rocket::config::{Config, Environment};
use ::rocket::request::{Form, FromForm};
use ::rocket::{get, post, routes};
use ::rocket_contrib::json::Json;
use ::serde::Serialize;
use ::std::env;

const DEFAULT_PORT: u16 = 80;
const DEFAULT_IP: &'static str = "0.0.0.0";

fn main() {
    let address = get_address();
    println!("Running on ... {}", address);

    rocket::custom(config())
        .mount("/", routes![roll_get, roll_post])
        .launch();
}

#[get("/roll/<input>")]
fn roll_get(input: String) -> String {
    let username = "You";
    let mut response: Vec<u8> = Vec::new();

    dice_roll::main(&username, &input, &mut response);

    String::from_utf8(response).unwrap()
}

#[post(
    "/roll",
    format = "application/x-www-form-urlencoded",
    data = "<command>"
)]
fn roll_post(command: Form<SlackCommand>) -> Json<SlackResponse> {
    let response_name = format!("<@{}>", command.user_id);
    let mut response: Vec<u8> = Vec::new();
    dice_roll::main(&response_name, &command.text, &mut response);

    Json(SlackResponse {
        response_type: SlackResponseType::InChannel,
        text: String::from_utf8(response).unwrap(),
    })
}

fn config() -> Config {
    Config::build(Environment::Staging)
        .address(get_address())
        .port(get_port())
        .finalize()
        .unwrap()
}

fn get_address() -> String {
    env::var("IP").unwrap_or_else(|_| DEFAULT_IP.to_string())
}

fn get_port() -> u16 {
    env::var("PORT")
        .map(|port_str| port_str.parse::<u16>().unwrap())
        .unwrap_or(DEFAULT_PORT)
}

#[derive(FromForm)]
pub struct SlackCommand {
    pub token: String,
    pub team_id: String,
    pub team_domain: String,
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackResponse {
    pub response_type: SlackResponseType,
    pub text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackResponseType {
    InChannel,
    Ephemeral,
}
