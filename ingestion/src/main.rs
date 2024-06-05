use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::Serialize;
use std::env;
use std::sync::Arc;
use tokio::time::{self, Duration};
use warp::Filter;

mod schema;
use schema::block_heights;

#[derive(Queryable, Identifiable, Insertable, Debug, AsChangeset, Serialize)]
#[diesel(table_name = block_heights)]
pub struct BlockHeight {
    pub id: i32,
    pub height: i32,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Arc::new(r2d2::Pool::builder().build(manager).expect("Failed to create pool"));

    let rpc_url = "https://blockstream.info/api/blocks/tip/height";

    tokio::spawn(fetch_and_store_block_height(pool.clone(), rpc_url.to_string()));

    let block_height_route = warp::path("block-height")
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
    warp::any().map(move || pool.clone())
}

async fn handle_get_block_height(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    let results: Vec<BlockHeight> = block_heights::table
        .order(block_heights::id.desc())
        .load::<BlockHeight>(&mut conn)
        .expect("Error loading block heights");

    Ok(warp::reply::json(&results))
}

async fn fetch_and_store_block_height(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
    rpc_url: String,
) {
    let client = reqwest::Client::new();

    let mut interval = time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        let response = client.get(&rpc_url).send().await;
        match response {
            Ok(res) => {
                let height: u32 = match res.json().await {
                    Ok(height) => height,
                    Err(e) => {
                        eprintln!("Error parsing response: {}", e);
                        continue;
                    }
                };

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
                eprintln!("Failed to get block height: {}", e);
            }
        }
    }
}
