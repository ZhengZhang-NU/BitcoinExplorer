use std::collections::HashMap;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};// connection pool
use dotenv::dotenv;//.env
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tokio::sync::Mutex;//async lock
use warp::Filter;// http route
use chrono::{NaiveDateTime, TimeZone, Utc};
use diesel::pg::Pg;


mod schema;
use schema::{offchain_data, block_info, transactions, transaction_inputs, transaction_outputs};



#[derive(Queryable, Identifiable, Insertable, Debug, AsChangeset, Serialize, Selectable)]
#[diesel(table_name = offchain_data)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(Pg))]
pub struct OffchainData {
    pub id: i32,
    pub block_height: i32,
    pub btc_price: f64,
    pub market_sentiment: Option<f64>,
    pub volume: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub timestamp: NaiveDateTime,
}




async fn insert_or_update_offchain_data(pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>, data: OffchainData) -> Result<(), diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // Check if data already exists
    let existing_data = offchain_data::table
        .filter(offchain_data::block_height.eq(data.block_height))
        .filter(offchain_data::btc_price.eq(data.btc_price))
        .first::<OffchainData>(&mut conn)
        .optional()?;

    if existing_data.is_none() {
        // Insert new data without specifying the ID
        let new_data = (
            offchain_data::block_height.eq(data.block_height),
            offchain_data::btc_price.eq(data.btc_price),
            offchain_data::market_sentiment.eq(data.market_sentiment),
            offchain_data::volume.eq(data.volume),
            offchain_data::high.eq(data.high),
            offchain_data::low.eq(data.low),
            offchain_data::timestamp.eq(data.timestamp),
        );

        match diesel::insert_into(offchain_data::table)
            .values(&new_data)
            .execute(&mut conn) {
            Ok(_) => {
                println!("Data inserted successfully");
                Ok(())
            },
            Err(e) => {
                eprintln!("Error inserting data: {}", e);
                Err(e)
            }
        }
    } else {
        diesel::update(offchain_data::table.find(existing_data.unwrap().id))
            .set((
                offchain_data::market_sentiment.eq(data.market_sentiment),
                offchain_data::volume.eq(data.volume),
                offchain_data::high.eq(data.high),
                offchain_data::low.eq(data.low),
                offchain_data::timestamp.eq(data.timestamp),
            ))
            .execute(&mut conn)?;
        Ok(())
    }
}




async fn fetch_and_store_offchain_data(pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>, block_height: i32) {
    let client = reqwest::Client::new();
    println!("Fetching offchain data for block height: {}", block_height);
    let response = client.get("https://api.coingecko.com/api/v3/coins/bitcoin/market_chart")
        .query(&[("vs_currency", "usd"), ("days", "1"), ("interval", "daily")])
        .send().await;

    if let Ok(res) = response {
        if res.status().is_success() {
            if let Ok(offchain_data) = res.json::<serde_json::Value>().await {
                if let Some(prices) = offchain_data["prices"].as_array() {
                    if let Some(price) = prices.first() {
                        let btc_price = price[1].as_f64().unwrap_or(0.0);
                        let market_sentiment = offchain_data["market_caps"].as_array()
                            .and_then(|caps| caps.first())
                            .and_then(|cap| cap[1].as_f64());
                        let volume = offchain_data["total_volumes"].as_array()
                            .and_then(|volumes| volumes.first())
                            .and_then(|volume| volume[1].as_f64());
                        let high = offchain_data["prices"].as_array()
                            .and_then(|prices| prices.iter().map(|price| price[1].as_f64().unwrap_or(0.0)).max_by(|a, b| a.partial_cmp(b).unwrap()));
                        let low = offchain_data["prices"].as_array()
                            .and_then(|prices| prices.iter().map(|price| price[1].as_f64().unwrap_or(0.0)).min_by(|a, b| a.partial_cmp(b).unwrap()));
                        let timestamp = Utc::now().naive_utc();

                        let new_data = OffchainData {
                            id: 0,
                            block_height,
                            btc_price,
                            market_sentiment,
                            volume,
                            high,
                            low,
                            timestamp,
                        };

                        match insert_or_update_offchain_data(pool.clone(), new_data).await {
                            Ok(_) => println!("Offchain data processed successfully."),
                            Err(e) => eprintln!("Error processing offchain data: {}", e),
                        }
                    }
                }
            }
        }
    }
}



async fn handle_get_offchain_data(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    let results: Vec<OffchainData> = offchain_data::table
        .order(offchain_data::id.desc())
        .load::<OffchainData>(&mut conn)
        .expect("Error loading offchain data");

    Ok(warp::reply::json(&results))
}


//==============================


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

#[derive(Queryable, Identifiable, Insertable, Debug, AsChangeset, Serialize)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub block_height: i32,
    pub hash: String,
    pub btc: f64,
    pub fee: i64,
    pub time: i64,
}

#[derive(Queryable, Identifiable, Insertable, Debug, AsChangeset, Serialize)]
#[diesel(table_name = transaction_inputs)]
pub struct TransactionInput {
    pub id: i32,
    pub transaction_id: i32,
    pub previous_output: String,
    pub value: i64,
}

#[derive(Queryable, Identifiable, Insertable, Debug, AsChangeset, Serialize)]
#[diesel(table_name = transaction_outputs)]
pub struct TransactionOutput {
    pub id: i32,
    pub transaction_id: i32,
    pub address: String,
    pub value: i64,
}


#[derive(Deserialize)]
struct ApiBlockInfo {
    id: String,
    height: i32,
    version: i32,
    timestamp: i64,
    tx_count: i32,
    size: i32,
    weight: i32,
    merkle_root: String,
    previousblockhash: String,
    mediantime: i64,
    nonce: i64,
    bits: i32,
    difficulty: f64,
    tx: Option<Vec<ApiTransaction>>,
}

#[derive(Deserialize)]
struct ApiTransaction {
    txid: String,
    fee: i64,
    vin: Vec<ApiTransactionInput>,
    vout: Vec<ApiTransactionOutput>,
}

#[derive(Deserialize)]
struct ApiTransactionInput {
    txid: String,
    vout: u32,
    sequence: i64,
    value: Option<i64>,
    prevout: Option<PrevOut>,

}


#[derive(Deserialize)]
struct PrevOut {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: String,
    value: i64,
}


#[derive(Deserialize, Debug)]
struct ApiTransactionOutput {
    value: f64,
    n: Option<u32>,
    script_pub_key: Option<ApiScriptPubKey>,
}



#[derive(Deserialize, Debug)]
struct ApiScriptPubKey {
    hex: String,
    asm: String,
    addresses: Vec<String>,
}


#[derive(Deserialize)]
struct BlockHashResponse {
    id: String,
}

#[derive(Serialize)]
struct BlockDetailData {
    block_info: BlockInfo,
    transactions: Vec<Transaction>,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
}

#[tokio::main]
async fn main() {
    println!("Starting the application...");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    println!("Creating connection pool...");
    let pool = Arc::new(r2d2::Pool::builder().build(manager).expect("Failed to create pool"));

    println!("Creating synchronization mechanism...");
    let is_fetching = Arc::new(Mutex::new(false));

    println!("Spawning tasks...");
    let pool_clone_for_block_info = Arc::clone(&pool);
    let is_fetching_clone = Arc::clone(&is_fetching);
    tokio::spawn(async move {
        fetch_and_store_block_info(pool_clone_for_block_info, "https://blockstream.info/api/blocks/tip/height".to_string(), is_fetching_clone).await;
    });

    let pool_clone_for_offchain = Arc::clone(&pool);
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10)); // every 10s run
        loop {
            interval.tick().await;

            println!("Fetching offchain data...");
            // get the real height
            let height_url = "https://blockstream.info/api/blocks/tip/height";
            let client = reqwest::Client::new();
            if let Ok(response) = client.get(height_url).send().await {
                if let Ok(height_text) = response.text().await {
                    if let Ok(block_height) = height_text.trim().parse::<i32>() {
                        println!("Fetched block height: {}", block_height);
                        fetch_and_store_offchain_data(pool_clone_for_offchain.clone(), block_height).await;
                    }
                }
            }
        }
    });

    println!("Setting up routes...");
    let block_info_route = warp::path("block-info")
        .and(warp::get())
        .and(with_db(Arc::clone(&pool)))
        .and_then(handle_get_block_info)
        .with(warp::cors().allow_any_origin());

    let block_detail_route = warp::path!("block" / i32)
        .and(warp::get())
        .and(with_db(Arc::clone(&pool)))
        .and_then(|height, pool| handle_get_block_detail(pool, height))
        .with(warp::cors().allow_any_origin());

    let offchain_data_route = warp::path("offchain-data")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_get_offchain_data)
        .with(warp::cors().allow_any_origin());

    println!("Starting server...");
    warp::serve(block_info_route.or(block_detail_route).or(offchain_data_route))
        .run(([0, 0, 0, 0], 8000))
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
    println!("Handling get block info...");
    let mut conn = pool.get().expect("Failed to get connection from pool");
    let results: Vec<BlockInfo> = block_info::table
        .order(block_info::id.desc())
        .load::<BlockInfo>(&mut conn)
        .expect("Error loading block info");

    println!("Returning block info...");
    Ok(warp::reply::json(&results))
}

async fn handle_get_block_detail(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
    height: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Handling get block detail for height: {}", height);
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let block_info_result: Option<BlockInfo> = block_info::table
        .filter(block_info::height.eq(height))
        .first(&mut conn)
        .optional()
        .expect("Error loading block info");

    if let Some(block_info) = block_info_result {
        println!("Block info found for height: {}", height);
        let transactions_result: Vec<Transaction> = transactions::table
            .filter(transactions::block_height.eq(height))
            .load::<Transaction>(&mut conn)
            .expect("Error loading transactions");

        let inputs_result: Vec<TransactionInput> = transaction_inputs::table
            .filter(transaction_inputs::transaction_id.eq_any(transactions_result.iter().map(|tx| tx.id)))
            .load::<TransactionInput>(&mut conn)
            .expect("Error loading transaction inputs");

        let outputs_result: Vec<TransactionOutput> = transaction_outputs::table
            .filter(transaction_outputs::transaction_id.eq_any(transactions_result.iter().map(|tx| tx.id)))
            .load::<TransactionOutput>(&mut conn)
            .expect("Error loading transaction outputs");

        let block_detail = BlockDetailData {
            block_info,
            transactions: transactions_result,
            inputs: inputs_result,
            outputs: outputs_result,
        };

        println!("Returning block detail for height: {}", height);
        Ok(warp::reply::json(&block_detail))
    } else {
        println!("Block not found for height: {}", height);
        let not_found = warp::reply::json(&"Block not found");
        Ok(not_found)
    }
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

        let response = client.get(&height_url).send().await;
        if let Ok(res) = response {
            if res.status().is_success() {
                if let Ok(height_text) = res.text().await {
                    if let Ok(height) = height_text.trim().parse::<i32>() {
                        let hash_url = format!("https://blockstream.info/api/block-height/{}", height);
                        if let Ok(hash_res) = client.get(&hash_url).send().await {
                            if hash_res.status().is_success() {
                                if let Ok(hash_text) = hash_res.text().await {
                                    let hash_info = BlockHashResponse { id: hash_text.trim().to_string() };

                                    let block_url = format!("https://blockstream.info/api/block/{}", hash_info.id);
                                    if let Ok(block_res) = client.get(&block_url).send().await {
                                        if block_res.status().is_success() {
                                            if let Ok(block_text) = block_res.text().await {
                                                if let Ok(api_block_info) = serde_json::from_str::<ApiBlockInfo>(&block_text) {
                                                    let mut conn = pool.get().expect("Failed to get connection from pool");

                                                    let existing_block: Option<BlockInfo> = block_info::table
                                                        .filter(block_info::height.eq(api_block_info.height))
                                                        .first(&mut conn)
                                                        .optional()
                                                        .expect("Error querying block info");

                                                    if existing_block.is_none() {
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

                                                        diesel::insert_into(block_info::table)
                                                            .values(&new_info)
                                                            .execute(&mut conn)
                                                            .expect("Error inserting block info");
                                                    }

                                                    let txs_url = format!("https://blockstream.info/api/block/{}/txs", hash_info.id);
                                                    if let Ok(txs_res) = client.get(&txs_url).send().await {
                                                        if txs_res.status().is_success() {
                                                            if let Ok(txs_text) = txs_res.text().await {
                                                                if let Ok(txs) = serde_json::from_str::<Vec<ApiTransaction>>(&txs_text) {
                                                                    for mut tx in txs {
                                                                        let latest_tx: Option<Transaction> = transactions::table
                                                                            .order(transactions::id.desc())
                                                                            .first(&mut conn)
                                                                            .optional()
                                                                            .expect("Error querying latest transaction");

                                                                        let new_tx_id = latest_tx.as_ref().map_or(1, |latest| latest.id + 1);

                                                                        let new_tx = Transaction {
                                                                            id: new_tx_id,
                                                                            block_height: api_block_info.height,
                                                                            hash: tx.txid.clone(),
                                                                            btc: tx.vout.iter().map(|vout| vout.value).sum(),
                                                                            fee: tx.fee,
                                                                            time: api_block_info.timestamp,
                                                                        };

                                                                        diesel::insert_into(transactions::table)
                                                                            .values(&new_tx)
                                                                            .execute(&mut conn)
                                                                            .expect("Error inserting transaction");


                                                                        for vin in tx.vin {
                                                                            let latest_input: Option<TransactionInput> = transaction_inputs::table
                                                                                .order(transaction_inputs::id.desc())
                                                                                .first(&mut conn)
                                                                                .optional()
                                                                                .expect("Error querying latest transaction input");

                                                                            let new_input_id = latest_input.as_ref().map_or(1, |latest| latest.id + 1);

                                                                            let value = vin.prevout.as_ref().map_or(0, |prevout| prevout.value);



                                                                            let new_input = TransactionInput {
                                                                                id: new_input_id,
                                                                                transaction_id: new_tx_id,
                                                                                previous_output: vin.txid.clone(),
                                                                                value,
                                                                            };

                                                                            match diesel::insert_into(transaction_inputs::table)
                                                                                .values(&new_input)
                                                                                .execute(&mut conn)
                                                                            {
                                                                                Ok(_) => println!("Successfully inserted new transaction input"),
                                                                                Err(e) => eprintln!("Error inserting transaction input: {}", e),
                                                                            }
                                                                        }

//=========================================================================
                                                                        for vout in tx.vout {
                                                                            println!("Processing vout: {:?}", vout);
                                                                            if let Some(script) = &vout.script_pub_key {
                                                                                // println!("ScriptPubKey found: {:?}", script);

                                                                                let latest_output: Option<TransactionOutput> = transaction_outputs::table
                                                                                    .order(transaction_outputs::id.desc())
                                                                                    .first(&mut conn)
                                                                                    .optional()
                                                                                    .expect("Error querying latest transaction output");

                                                                                let new_output_id = latest_output.as_ref().map_or(1, |latest| latest.id + 1);

                                                                                let address = script.addresses.join(", ");

                                                                                let new_output = TransactionOutput {
                                                                                    id: new_output_id,
                                                                                    transaction_id: new_tx_id,
                                                                                    address: address.clone(),
                                                                                    value: vout.value as i64,
                                                                                };

                                                                                match diesel::insert_into(transaction_outputs::table)
                                                                                    .values(&new_output)
                                                                                    .execute(&mut conn)
                                                                                {
                                                                                    Ok(_) => println!("Successfully inserted new transaction output with address: {}", address),
                                                                                    Err(e) => eprintln!("Error inserting transaction output: {}", e),
                                                                                }
                                                                            } else {

                                                                                let latest_output: Option<TransactionOutput> = transaction_outputs::table
                                                                                    .order(transaction_outputs::id.desc())
                                                                                    .first(&mut conn)
                                                                                    .optional()
                                                                                    .expect("Error querying latest transaction output");

                                                                                let new_output_id = latest_output.as_ref().map_or(1, |latest| latest.id + 1);

                                                                                let new_output = TransactionOutput {
                                                                                    id: new_output_id,
                                                                                    transaction_id: new_tx_id,
                                                                                    address: "".to_string(),
                                                                                    value: vout.value as i64,
                                                                                };

                                                                                match diesel::insert_into(transaction_outputs::table)
                                                                                    .values(&new_output)
                                                                                    .execute(&mut conn)
                                                                                {
                                                                                    Ok(_) => println!("Successfully inserted new transaction output without address"),
                                                                                    Err(e) => eprintln!("Error inserting transaction output: {}", e),
                                                                                }
                                                                            }
                                                                        }





//==========================================================================

                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }



                    }
                }
            }
        }

        *is_fetching_guard = false;
    }
}

