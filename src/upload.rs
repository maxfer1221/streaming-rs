use std::io::{Error, ErrorKind::{InvalidData, PermissionDenied}, Cursor};
use std::fs::File;
use tiny_http::{Request, Response, Method};
use crate::utils::ResponseType;
use json::{JsonValue, parse};

pub fn upload(r: &mut Request) -> std::io::Result<ResponseType> {
    parse_method(r)?;
    let mut content = String::new();
    r.as_reader().read_to_string(&mut content)?;
    let json: JsonValue = match parse(&content) {
        Ok(j) => j,
        _ => Err(Error::new(InvalidData, "Only JSON objects can be passed to this URI"))?,
    };
    let bytes = &json["bytes"];
    println!("{:?}", bytes);
    return Ok(ResponseType::Curs(Response::from_string("OK")));
}


pub fn parse_method(r: &Request) -> std::io::Result<()> {
    return match r.method() {
        Method::Post => Ok(()),
        _ => Err(Error::new(PermissionDenied, "This URI cannot handle this type of request. Only POST has been implemented")),
    }
}
