#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn homepage() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file))
}


fn main() {
    rocket::ignite().mount("/", routes![homepage,files]).launch();
}
