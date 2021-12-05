use tiny_http::{Server};
use std::env;

mod parse;
mod statics;
mod upload;
mod watch;

mod cors;

mod utils;
use crate::utils::CONTENT_TYPES as CT;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ip: &String = &args[1];

    let server = Server::http(ip).unwrap();
    println!("Server listening on: {}", ip);

    for request in server.incoming_requests() {
        let split: Vec<&str> = request.url().split('/').collect::<Vec<&str>>()[1..].to_vec();
        println!("{:?}", split);

        let response = match parse::create_response(&request, split) {
            Ok(r) => r,
            Err(_e) => statics::file(vec!["html", "404.html"], 404, CT.html).unwrap(), 
        };
        match response {
            utils::ResponseType::File(r) => {
                cors::cors_respond(request, r).unwrap();
            },
            utils::ResponseType::Curs(r) => {
                cors::cors_respond(request, r). unwrap();
            }
        }
    }
}
