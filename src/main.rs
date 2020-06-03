#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod db;
mod schema;
mod models;
mod routes;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![tests,search_tests,new_test])
        .launch();
}
