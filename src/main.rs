mod db;
mod schema;
mod models;

use models::Test;
use diesel::prelude::*;

#[macro_use]
extern crate diesel;

fn main() {
    use self::schema::tests::dsl::*;

    let db = db::establish_connection();

    let results = tests.limit(5).load::<Test>(&db).expect("Error loading posts");

    println!("Displaying {} tests", results.len());
    for test in results {
        println!("{}", test.name);
        println!("-----------\n");
    }
}
