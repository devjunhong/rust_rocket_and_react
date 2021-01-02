#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket; 

use std::io;
use std::env; 
use std::path::{Path, PathBuf};

use rocket::response::NamedFile; 

#[get("/")]
fn index() -> io::Result<NamedFile> {
    let page_directory_path = get_directory_path();
    NamedFile::open(Path::new(&page_directory_path).join("index.html"))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = get_directory_path();
    NamedFile::open(Path::new(&page_directory_path).join(file))
}

fn get_directory_path() -> String {
    format!("{}/../frontend/build", env!("CARGO_MANIFEST_DIR"))
}

fn main() {
    rocket::ignite().mount("/", routes![index, files]).launch();
}