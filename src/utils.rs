use std::{fs::File, path::PathBuf, io::Cursor};
use std::io::{Error, ErrorKind::PermissionDenied};
use tiny_http::Response;

// content types statics
pub struct ContentType {
    pub html: &'static str,
    pub css: &'static str,
    pub js: &'static str,
}

// static content type object to hold 'content-type' header values
pub static CONTENT_TYPES: ContentType = ContentType {
    html: "text/html; charset=UTF-8",
    css: "text/css; charset=utf-8",
    js: "text/javascript; charset=utf-8",
};

// Response wrapper for splitting into vectors, for video transfer, and static files
pub enum ResponseType {
    File(Response<File>),
    Curs(Response<Cursor<Vec<u8>>>)
}

// simple Response -> ResponseType wrapping
impl ResponseType {
    pub fn from_file(r: Response<File>) -> Self {
        ResponseType::File(r)
    }
    pub fn from_curs(r: Response<Cursor<Vec<u8>>>) -> Self {
        ResponseType::Curs(r)
    }
}

fn get_true_wd() -> std::io::Result<PathBuf> {
    let mut wd: PathBuf = std::env::current_exe()?;
    wd.pop();
    wd.pop();
    wd.pop();
    Ok(wd)
}

// failable + safe file fetching
pub fn get_file(paths: Vec<&str>) -> std::io::Result<File> {
    let mut wd: PathBuf = get_true_wd()?;
    wd.push("statics");
    println!("{:?}", wd);
    for path in paths {
        if path == ".." {
           return Err(Error::new(PermissionDenied, "'..' cannot be used for file pathing"));
        }
        wd.push(path);
    }
    println!("{:?}", wd);
    Ok(File::open(wd)?)
}

pub fn create_chunk_files(video_id: String, chunk_count: u16) -> std::io::Result<Vec<File>> {
    let mut wd: PathBuf = get_true_wd()?;
    wd.push("videos/");
    if !video_id.eq("..".into()) { wd.push(video_id) }
    else { return Err(Error::new(PermissionDenied, "'..' cannot be used for file pathing")); }
    
    let mut files: Vec<File> = Vec::with_capacity(2);

    for i in 0..chunk_count {
        wd.set_file_name(i.to_string());
        wd.set_extension("mp4");
        let f = File::create(&wd)?;
        if i == 0 {
            files[0] = f;
        }
        else if (i as f32 / 2.0).ceil() == (chunk_count as f32 / 2.0).ceil() {
            files[1] = f;
        };
    }
    Ok(files)
}

pub fn get_chunk_file() -> std::io::Result<()> {
    unimplemented!("get_chunk_file not implemented");
    Ok(())
}
