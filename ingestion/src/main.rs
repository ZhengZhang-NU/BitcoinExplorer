use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::Serialize;
use std::env;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tokio::sync::Mutex;
use warp::Filter;

mod schema;
use schema::block_heights;

#[derive(Queryable, Identifiable, Insertable, Debug, AsChangeset, Serialize)]
#[diesel(table_name = block_heights)]
pub struct BlockHeight {
    pub id: i32,  // Block height record ID
    pub height: i32,  // Block height value
}

#[tokio::main]
async fn main() {
    dotenv().ok();  // Load environment variables from .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Arc::new(r2d2::Pool::builder().build(manager).expect("Failed to create pool"));

    let rpc_url = "https://blockstream.info/api/blocks/tip/height";  //URL for fetching block height

    let is_fetching = Arc::new(Mutex::new(false));  //The Mutex to control concurrent access
    tokio::spawn(fetch_and_store_block_height(pool.clone(), rpc_url.to_string(), is_fetching.clone()));  // Spawn async task

    let block_height_route = warp::path("block-height")  // Define warp route
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_get_block_height)
        .with(warp::cors().allow_any_origin());

    warp::serve(block_height_route)
        .run(([127, 0, 0, 1], 8000))
        .await;
}

fn with_db(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> impl Filter<Extract = (Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())  //Clone database pool for warp route
}

async fn handle_get_block_height( // It gets a database connection from the pool, queries the block heights, and returns them as a JSON response.
                                  pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    let results: Vec<BlockHeight> = block_heights::table
        .order(block_heights::id.desc())  //Order results by descending ID
        .load::<BlockHeight>(&mut conn)
        .expect("Error loading block heights");

    Ok(warp::reply::json(&results))  //Return results as JSON
}

async fn fetch_and_store_block_height(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
    rpc_url: String,
    is_fetching: Arc<Mutex<bool>>,
) {
    let client = reqwest::Client::new();  // HTTP client for fetching data

    let mut interval = time::interval(Duration::from_secs(10));  //Interval for periodic fetching
    loop {
        interval.tick().await;

        let mut is_fetching_guard = is_fetching.lock().await;
        if *is_fetching_guard {
            continue;  // Skip if already fetching
        }
        *is_fetching_guard = true;

        let response = client.get(&rpc_url).send().await;  //Send HTTP GET request
        match response {
            Ok(res) => {
                if res.status().is_success() {  // Check if HTTP status is success
                    let text = res.text().await.unwrap_or_else(|_| "Unable to read response text".to_string());
                    match serde_json::from_str::<u32>(&text) {  // Parse response JSON
                        Ok(height) => {
                            let mut conn = pool.get().expect("Failed to get connection from pool");
                            let latest_height: Option<BlockHeight> = block_heights::table
                                .order(block_heights::id.desc())
                                .first(&mut conn)
                                .optional()
                                .expect("Error querying latest block height");

                            let new_id = latest_height.as_ref().map_or(1, |latest| latest.id + 1);

                            let new_height = BlockHeight {
                                id: new_id,
                                height: height as i32,
                            };

                            match diesel::insert_into(block_heights::table)
                                .values(&new_height)
                                .execute(&mut conn)
                            {
                                Ok(_) => println!("Successfully inserted new block height"),
                                Err(e) => eprintln!("Error inserting block height: {}", e),
                            }
                        }
                        Err(e) => {
                            eprintln!("Error parsing response: {}, response text: {}", e, text);  // Log parsing error
                        }
                    }
                } else {
                    let status = res.status();
                    let text = res.text().await.unwrap_or_else(|_| "Unable to read response text".to_string());
                    eprintln!("Failed to get block height. HTTP Status: {}, response text: {}", status, text);  // Log HTTP error
                }
            }
            Err(e) => {
                eprintln!("Failed to get block height: {}", e);  // Log request error
            }
        }

        *is_fetching_guard = false;  // Release mutex lock
    }
}
