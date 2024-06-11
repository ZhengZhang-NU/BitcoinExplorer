use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tokio::sync::Mutex;
use warp::Filter;
use chrono::{NaiveDateTime, Utc, TimeZone};

mod schema;
use schema::block_info;

#[derive(Queryable, Identifiable, Insertable, Debug, AsChangeset, Serialize)]
#[diesel(table_name = block_info)]
pub struct BlockInfo {
    pub id: i32,
    pub height: i32,
    pub avg_tx_count: i32,
    pub difficulty: f64,
    pub block_time: i32,
    pub timestamp: NaiveDateTime,
    pub size: i32,
    pub weight: i32,
}

#[derive(Deserialize)]
struct ApiBlockInfo {
    height: i32,
    tx_count: i32,
    difficulty: f64,
    timestamp: i64,
    size: i32,
    weight: i32,
    mediantime: i64,
}

#[derive(Deserialize)]
struct BlockHashResponse {
    id: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Arc::new(r2d2::Pool::builder().build(manager).expect("Failed to create pool"));

    let height_url = "https://blockstream.info/api/blocks/tip/height";

    let is_fetching = Arc::new(Mutex::new(false));
    tokio::spawn(fetch_and_store_block_info(pool.clone(), height_url.to_string(), is_fetching.clone()));

    let block_info_route = warp::path("block-info")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_get_block_info)
        .with(warp::cors().allow_any_origin());

    warp::serve(block_info_route)
        .run(([127, 0, 0, 1], 8000))
        .await;
}

fn with_db(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> impl Filter<Extract = (Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn handle_get_block_info(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    let results: Vec<BlockInfo> = block_info::table
        .order(block_info::id.desc())
        .load::<BlockInfo>(&mut conn)
        .expect("Error loading block info");

    Ok(warp::reply::json(&results))
}

async fn fetch_and_store_block_info(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
    height_url: String,
    is_fetching: Arc<Mutex<bool>>,
) {
    let client = reqwest::Client::new();

    let mut interval = time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;

        let mut is_fetching_guard = is_fetching.lock().await;
        if *is_fetching_guard {
            continue;
        }
        *is_fetching_guard = true;

        match client.get(&height_url).send().await {
            Ok(res) if res.status().is_success() => {
                let height_text = res.text().await.unwrap_or_else(|_| "Unable to read response text".to_string());
                match height_text.trim().parse::<i32>() {
                    Ok(height) => {
                        let hash_url = format!("https://blockstream.info/api/block-height/{}", height);
                        match client.get(&hash_url).send().await {
                            Ok(hash_res) if hash_res.status().is_success() => {
                                let hash_text = hash_res.text().await.unwrap_or_else(|_| "Unable to read response text".to_string());
                                let hash_info = BlockHashResponse { id: hash_text.trim().to_string() };

                                let block_url = format!("https://blockstream.info/api/block/{}", hash_info.id);
                                match client.get(&block_url).send().await {
                                    Ok(block_res) if block_res.status().is_success() => {
                                        let block_text = block_res.text().await.unwrap_or_else(|_| "Unable to read response text".to_string());
                                        match serde_json::from_str::<ApiBlockInfo>(&block_text) {
                                            Ok(api_block_info) => {
                                                let mut conn = pool.get().expect("Failed to get connection from pool");
                                                let latest_info: Option<BlockInfo> = block_info::table
                                                    .order(block_info::id.desc())
                                                    .first(&mut conn)
                                                    .optional()
                                                    .expect("Error querying latest block info");

                                                let new_id = latest_info.as_ref().map_or(1, |latest| latest.id + 1);

                                                let timestamp = Utc.timestamp_opt(api_block_info.timestamp, 0).unwrap();

                                                let new_info = BlockInfo {
                                                    id: new_id,
                                                    height: api_block_info.height,
                                                    avg_tx_count: api_block_info.tx_count,
                                                    difficulty: api_block_info.difficulty,
                                                    block_time: api_block_info.mediantime as i32,
                                                    timestamp: timestamp.naive_utc(),
                                                    size: api_block_info.size,
                                                    weight: api_block_info.weight,
                                                };

                                                match diesel::insert_into(block_info::table)
                                                    .values(&new_info)
                                                    .execute(&mut conn)
                                                {
                                                    Ok(_) => println!("Successfully inserted new block info"),
                                                    Err(e) => eprintln!("Error inserting block info: {}", e),
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("Error parsing block response: {}, response text: {}", e, block_text);
                                            }
                                        }
                                    }
                                    Ok(block_res) => {
                                        let status = block_res.status();
                                        let text = block_res.text().await.unwrap_or_else(|_| "Unable to read response text".to_string());
                                        eprintln!("Failed to get block info. HTTP Status: {}, response text: {}", status, text);
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to get block info: {}", e);
                                    }
                                }
                            }
                            Ok(hash_res) => {
                                let status = hash_res.status();
                                let text = hash_res.text().await.unwrap_or_else(|_| "Unable to read response text".to_string());
                                eprintln!("Failed to get block hash. HTTP Status: {}, response text: {}", status, text);
                            }
                            Err(e) => {
                                eprintln!("Failed to get block hash: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error parsing height response: {}, response text: {}", e, height_text);
                    }
                }
            }
            Ok(res) => {
                let status = res.status();
                let text = res.text().await.unwrap_or_else(|_| "Unable to read response text".to_string());
                eprintln!("Failed to get block height. HTTP Status: {}, response text: {}", status, text);
            }
            Err(e) => {
                eprintln!("Failed to get block height: {}", e);
            }
        }

        *is_fetching_guard = false;
    }
}
