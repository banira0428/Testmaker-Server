use warp::{Filter, Rejection, Reply};

use super::db::Database;
use super::handler::list_tests_handler;
use super::handler::put_test_handler;

pub fn tests_api(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list(db.clone()).or(put_test(db))
}

fn tests() -> warp::filters::BoxedFilter<()> {
    warp::path("tests").boxed()
}

fn test_id() -> warp::filters::BoxedFilter<(u64,)> {
    warp::path::param().boxed()
}

fn list(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    tests()
        .and(warp::get())
        .and_then(move || list_tests_handler(db.clone()))
}

fn put_test(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    tests()
        .and(test_id())
        .and(warp::put())
        .and(warp::body::json())
        .and_then(move |id, body| put_test_handler(db.clone(), id, body))
}