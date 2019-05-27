#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::NamedFile;
use rocket::request::{self, Request, FromRequest};
use std::io;
use std::path::PathBuf;
use rocket::Outcome;
use rocket::http::Status;

#[derive(Debug, PartialEq)]
enum Host {
    CollinValley,
    DawnCronin,
    Indeterminate
}

impl<'a, 'r> FromRequest<'a, 'r> for Host {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Host, ()> {
        let hosts: Vec<_> = request.headers().get("host").collect();
        if hosts.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        let host = hosts[0];
        if host.contains("collinvalley") {
            return Outcome::Success(Host::CollinValley);
        } else if host.contains("dawncronin") {
            return Outcome::Success(Host::DawnCronin);
        } else{
            return Outcome::Success(Host::Indeterminate);
        }
    }
}

#[get("/")]
fn known_host_homepage(host: Host) -> io::Result<NamedFile> {
    print!("{:?}", host);
    match host {
        Host::CollinValley => { NamedFile::open("static/collinvalley/index.html") },
        Host::DawnCronin =>  { NamedFile::open("static/dawncronin/dawncroninhome.html") },
        Host::Indeterminate => { NamedFile::open("static/collinvalley/index.html") },
    }
}

#[get("/", rank=2)]
fn unknown_host_homepage() -> io::Result<NamedFile> {
    NamedFile::open("static/collinvalley/index.html")
}

#[get("/<file..>", rank=1)]
fn files(host: Host, file: PathBuf) -> io::Result<NamedFile> {
    let mut path = PathBuf::new();
    path.push("static");
    match host {
        Host::CollinValley => { path.push("collinvalley"); },
        Host::DawnCronin => { path.push("dawncronin"); },
        Host::Indeterminate => { path.push("collinvalley"); },
    }
    path.push(file);

    if path.is_dir() {
        path.push("index.html");
    }
    
    NamedFile::open(path)
}

fn main() {
    rocket::ignite().mount("/", routes![known_host_homepage,
                                        unknown_host_homepage,
                                        files])
                    .launch();
}
