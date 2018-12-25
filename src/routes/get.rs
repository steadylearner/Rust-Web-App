use std::io;
use rocket::response::{NamedFile};

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/share")]
pub fn signle_page_app() -> io::Result<NamedFile> {
    NamedFile::open("static/web/index.html")
}


