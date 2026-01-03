use axum::Router;
use tokio::join;

mod routes;
mod db;
mod cron;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = db::connect_db().await;

    let app = routes::create_routes();

    let server = async {
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    };

    let cron = async {
        cron::scheduler::start_scheduler(pool).await;
    };

    join!(server, crom);
}