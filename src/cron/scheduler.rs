use std::env;
use tokio_cron_scheduler::{Job, JobScheduler};
use reqwest::Client;
use serde_json::Value;
use sqlx::{Pool, Postgres};


pub async fn start_scheduler(pool: Pool<Postgres>) {
    let sched = JobScheduler::new().await.unwrap();

    let job = Job::new_async("0 */2 * * * *", move |_uuid, _l| {
        let pool = pool.clone();
        Box::pin(async move {
            if let Err(e) = fetch_and_store(&pool).await {
                eprintln!("Error in cron job: {:?}", e);
            }
        })
    })
    .unwrap();

    sched.add(job).await.unwrap();
    sched.start().await.unwrap();
}



pub async fn fetch_and_store(pool: &Pool<Postgres>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = std::env::var("WEATHERSTACK_API_KEY")
        .unwrap_or_else(|_| "fd5b4e3f1a87e29868907fcf54150507".to_string());

    let url = format!(
        "http://api.weatherstack.com/current?access_key={}&query=Indore&units=m",
        api_key
    );

    let resp: Value = client.get(&url)
        .send()
        .await?
        .json()
        .await?;

    let temp = resp["current"]["temperature"].as_f64().unwrap_or(0.0);
    let humidity = resp["current"]["humidity"].as_i64().unwrap_or(0);
    let desc = resp["current"]["weather_descriptions"][0]
        .as_str()
        .unwrap_or("N/A");

    sqlx::query(
    "INSERT INTO weather_logs (city, temperature, humidity, description, logged_at)
     VALUES ($1, $2, $3, $4, NOW())"
)
.bind("Indore")
.bind(temp)
.bind(humidity)
.bind(desc)
.execute(pool)
.await?;


    println!("Logged Indore weather: {}°C, {}% humidity, {}", temp, humidity, desc);
    Ok(())
}

