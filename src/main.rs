use actix_web::{App, HttpServer, Responder, get};
use tokio::join;

mod cron;
mod db;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello from Actix Web!"
}

#[get("/health")]
async fn health() -> impl Responder {
    "OK"
}

#[get("/city")]
async fn city() -> impl Responder {
    "Indore: Sample route for city info"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = db::connect_db().await;

    let server = async {
        HttpServer::new(|| App::new().service(hello).service(health).service(city))
            .bind(("0.0.0.0", 3000))?
            .run()
            .await
    };

    let cron = async {
        cron::scheduler::start_scheduler(pool).await;
    };

    join!(server, cron);

    Ok(())
}
