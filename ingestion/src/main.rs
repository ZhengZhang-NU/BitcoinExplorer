use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::Serialize;
use std::env;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use warp::Filter;

mod schema;
use schema::block_heights;

#[derive(Queryable, Identifiable, Insertable, Debug, AsChangeset, Serialize)]
#[diesel(table_name = block_heights)]
pub struct BlockHeight {
    pub id: i32,
    pub height: i32,
}

#[derive(Debug)]
struct CustomError(String);

impl warp::reject::Reject for CustomError {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Arc::new(r2d2::Pool::builder().build(manager).expect("Failed to create pool"));

    let rpc = reqwest::Client::new();
    let pool_clone = pool.clone();

    tokio::spawn(async move {
        loop {
            match fetch_and_store_block_height(&rpc, &pool_clone).await {
                Ok(_) => println!("Successfully inserted new block height"),
                Err(e) => eprintln!("Error inserting block height: {:?}", e),
            }
            sleep(Duration::from_secs(10)).await;
        }
    });

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

async fn fetch_and_store_block_height(
    rpc: &reqwest::Client,
    pool: &Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<(), warp::Rejection> {
    let response = rpc
        .get("https://blockchain.info/q/getblockcount")
        .send()
        .await
        .map_err(|e| warp::reject::custom(CustomError(format!("API request failed: {:?}", e))))?;

    if !response.status().is_success() {
        return Err(warp::reject::custom(CustomError(format!(
            "API request failed with status: {}",
            response.status()
        ))));
    }

    let block_height: i32 = response
        .text()
        .await
        .map_err(|e| warp::reject::custom(CustomError(format!("Invalid response: {:?}", e))))?
        .parse()
        .map_err(|e| warp::reject::custom(CustomError(format!("Invalid block height: {:?}", e))))?;

    println!("API response: {}", block_height);

    let mut conn = pool.get().map_err(|e| warp::reject::custom(CustomError(format!("DB pool error: {:?}", e))))?;

    let latest_height: Option<BlockHeight> = block_heights::table
        .order(block_heights::id.desc())
        .first::<BlockHeight>(&mut conn)
        .optional()
        .map_err(|e| warp::reject::custom(CustomError(format!("DB query error: {:?}", e))))?;

    if let Some(latest) = &latest_height {
        if latest.height == block_height {
            println!("Block height {} is already the latest, skipping insert.", block_height);
            return Ok(());
        }
    }

    let new_id = latest_height.as_ref().map_or(1, |latest| latest.id + 1);
    let new_height = BlockHeight { id: new_id, height: block_height };

    diesel::insert_into(block_heights::table)
        .values(&new_height)
        .execute(&mut conn)
        .map_err(|e| warp::reject::custom(CustomError(format!("DB insert error: {:?}", e))))?;

    Ok(())
}

async fn handle_get_block_height(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get().map_err(|e| warp::reject::custom(CustomError(format!("DB pool error: {:?}", e))))?;
    let results: Vec<BlockHeight> = block_heights::table
        .order(block_heights::id.desc())
        .load::<BlockHeight>(&mut conn)
        .map_err(|e| warp::reject::custom(CustomError(format!("DB query error: {:?}", e))))?;

    Ok(warp::reply::json(&results))
}
