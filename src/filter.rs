use warp::{Filter, Rejection, Reply};

use crate::{list_tests_handler, put_test_handler, Database};

pub fn tests_api(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list(db.clone()) || put_test(db)
}

fn tests() -> warp::filters::BoxedFilter<()> {
    warp::path("tests").boxed()
}

fn list(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    tests()
        .and(warp::get())
        .and_then(move || list_tests_handler(db.clone()))    // Handlerを呼び出す
}

fn put_test(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    tests()
        .and(user_id())    // User IdをPathから取得
        .and(warp::put())    // HTTP PUTメソッドを指定
        .and(warp::body::json())    // Request Bodyに含まれたJSONを取り出しUser型へ変換
        .and_then(move |id, body| put_user_handler(db.clone(), id, body))    // Handlerを呼び出す
}