#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;

use std::env;

use rocket::config;
use rocket_contrib::json;

#[derive(Serialize)]
struct Response {
    msg: String,
}

#[derive(Serialize)]
struct OutputBody {
    res: String,
}

#[derive(Serialize)]
struct ResponseMessage {
    returnvalue: Response,
    outputs: OutputBody,
}

#[post("/RustFunction")]
fn hello() -> json::Json<ResponseMessage> {
    json::Json(ResponseMessage {
        returnvalue: Response{msg: "hello world!".to_string()},
        outputs: OutputBody{res: "ok".to_string()}
    })
}

#[get("/")]
fn index() -> &'static str {
    ""
}

fn main() {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Whoops - not a int..."),
        Err(_) => 8080,
    };

    let config = config::Config::build(config::Environment::Staging)
        .address("0.0.0.0")
        .port(port)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![index, hello])
        .launch();
}
