#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::NamedFile;
use rocket::request::{self, Request, FromRequest};
use std::io;
use std::path::{Path, PathBuf};
use rocket::Outcome;
use rocket::http::Status;

#[derive(Debug)]
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
        Host::CollinValley => { NamedFile::open("static/collinvalleyhome.html") },
        Host::DawnCronin =>  { NamedFile::open("static/dawncroninhome.html") },
        Host::Indeterminate => { NamedFile::open("static/collinvalleyhome.html") },
    }
}

#[get("/", rank=2)]
fn unknown_host_homepage() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file))
}


fn main() {
    rocket::ignite().mount("/", routes![known_host_homepage,
                                        unknown_host_homepage,
                                        files])
                    .launch();
}
