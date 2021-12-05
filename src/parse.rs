use tiny_http::Request;
use crate::{statics, watch, upload};
use crate::utils::{ResponseType, CONTENT_TYPES as CT};
use std::io::{Error, ErrorKind::NotFound};

pub fn create_response(r: &Request, split: Vec<&str>) -> Result<ResponseType, Error>{
    let response = match split[0] {
        "w" => watch::req(r)?,
        "u" => upload::req(r)?,
        "" => statics::file(vec!["html", "index.html"], 200, CT.html)?,
        "watch" => statics::file(split, 200, CT.html)?,
        "upload" => statics::file(split, 200, CT.html)?,
        "assets" => statics::file(split, 200, CT.js)?,
        "css" => statics::file(split, 200, CT.css)?,
        _ => Err(Error::new(NotFound, "File not found"))?,
    };
    Ok(response)
}
