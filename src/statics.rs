use tiny_http::{Response, Header};
use crate::utils::{get_file, ResponseType};

pub fn file(name: Vec<&str>, code: u16, content_type: &str) -> std::io::Result<ResponseType> {
    println!("{:?}", name);
    let mut response = Response::from_file(get_file(name)?);
    response = response.with_status_code(code);
    response.add_header(Header::from_bytes(
        &b"Content-Type"[..], content_type.as_bytes()
    ).unwrap());
    Ok(ResponseType::from_file(response))
}
