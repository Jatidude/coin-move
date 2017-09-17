#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rusqlite;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use rocket::{Rocket, State, Request};
use rocket::response::Redirect;
use rusqlite::{Connection, Error};
use rocket_contrib::Template;
use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

type DbConn = Mutex<Connection>;

fn init_database(conn: &Connection) {
	conn.execute("CREATE TABLE users (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  password        TEXT NOT NULL,
                  balance         REAL
                  )", &[])
        .expect("create users table");
}

#[derive(Serialize)]
struct User {
	id: i32,
	name: String,
	pw: String,
	balance: f64
}

#[get("/")]
fn index() -> io::Result<NamedFile>{
	NamedFile::open("static/index.html")
}

#[get("/signup")]
fn index() -> io::Result<NamedFile>{
	NamedFile::open("static/signup.html")
}

#[get("/login")]
fn index() -> io::Result<NamedFile>{
	NamedFile::open("static/login.html")
}



fn rocket() -> Rocket {
    // Open a new in-memory SQLite database.
    let conn = Connection::open_in_memory().expect("in memory db");

    // Initialize the `entries` table in the in-memory database.
    init_database(&conn);

    // Have Rocket manage the database pool.
    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/", routes![index])
        .attach(Template::fairing())
}
