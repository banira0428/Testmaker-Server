use warp::{Rejection, Reply};

use crate::{Database, User};

pub async fn list_tests_handler(db: Database) -> Result<impl Reply, Rejection> {
    let db = db.lock().await;
    let tests = db
        .clone()
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<Test>>();
    Ok(warp::reply::json(&tests))
}

pub async fn put_test_handler(db: Database, id: u64, test: Test) -> Result<impl Reply, Rejection> {
    if id != test.id() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }
    let mut db = db.lock().await;
    db.insert(test.id(), test.clone());
    Ok(warp::reply::with_status(
        warp::reply::json(&test),
        warp::http::StatusCode::OK,
    ))
}