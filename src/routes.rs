use rocket_contrib::json::Json;
use diesel::prelude::*;

use super::db;
use super::models::Test;
use super::schema;

#[get("/tests")]
pub fn tests() -> Json<Vec<Test>> {
    use self::schema::tests::dsl::*;
    let connection = db::establish_connection();
    let results = tests.limit(5).load::<Test>(&connection).expect("Error loading posts");

    Json(results)
}