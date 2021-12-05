use tiny_http::{Request, Response, Header};

pub fn cors_respond<R: std::io::Read>(req: Request, mut res: Response<R>) -> std::io::Result<()> {
    res.add_header(Header::from_bytes(
        &b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
    res.add_header(Header::from_bytes(
        &b"Access-Control-Allow-Headers"[..], &b"*"[..]).unwrap());
    res.add_header(Header::from_bytes(
        &b"Access-Control-Request-Headers"[..], &b"*"[..]).unwrap());
    req.respond(res)?;
    Ok(())
}
