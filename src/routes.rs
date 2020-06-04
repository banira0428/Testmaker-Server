use rocket_contrib::json::Json;
use diesel::prelude::*;

use super::db;
use super::schema;
use super::models::Test;
use crate::models::NewTest;

#[get("/tests")]
pub fn tests() -> Json<Vec<Test>> {
    use self::schema::tests::dsl::*;
    let connection = db::establish_connection();
    let results = tests.limit(50).load::<Test>(&connection).expect("Error loading posts");

    Json(results)
}

#[get("/tests?<query>")]
pub fn search_tests(query: String) -> Json<Vec<Test>> {
    use self::schema::tests::dsl::*;
    let connection = db::establish_connection();
    let results = tests
        .filter(name.ilike(format!("%{}%", query)))
        .limit(50)
        .load::<Test>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[post("/tests", format = "application/json", data = "<test>")]
pub fn new_test(test: Json<NewTest>) -> String {
    use self::schema::tests;
    let connection = db::establish_connection();

    diesel::insert_into(tests::table)
        .values(&(test.into_inner()))
        .get_result::<Test>(&connection)
        .expect("Error saving new post");

    format!("Accepted post request!")
}

#[delete("/tests/<deleted_id>")]
pub fn delete_test(deleted_id: String) -> String {
    use self::schema::tests::dsl::*;

    let connection = db::establish_connection();

    diesel::delete(tests.filter(document_id.eq(deleted_id)))
        .execute(&connection)
        .expect("Error deleting test");

    format!("Accepted delete request!")
}