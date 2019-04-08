#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::NamedFile;
use std::io;

#[get("/")]
fn homepage() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn main() {
    rocket::ignite().mount("/", routes![homepage]).launch();
}
