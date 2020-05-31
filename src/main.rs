use testmaker_rust::{db, filter};

#[tokio::main]
async fn main() {
    let database = db::init_db();

    warp::serve(filter::tests_api(database)).run(([127, 0, 0, 1], 3030)).await;
}
