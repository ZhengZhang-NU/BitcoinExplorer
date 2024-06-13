use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tokio::sync::Mutex;
use warp::Filter;
use chrono::{NaiveDateTime, TimeZone, Utc};

mod schema;
use schema::{block_info, transactions, transaction_inputs, transaction_outputs};

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


#[derive(Deserialize)]
struct ApiTransactionOutput {
    value: f64,
    n: Option<u32>,
    script_pub_key: Option<ApiScriptPubKey>,
}


#[derive(Deserialize)]
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

    let block_detail_route = warp::path!("block" / i32)
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(|height, pool| handle_get_block_detail(pool, height))
        .with(warp::cors().allow_any_origin());

    warp::serve(block_info_route.or(block_detail_route))
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

async fn handle_get_block_detail(
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
    height: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let block_info_result: Option<BlockInfo> = block_info::table
        .filter(block_info::height.eq(height))
        .first(&mut conn)
        .optional()
        .expect("Error loading block info");

    if let Some(block_info) = block_info_result {
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

        Ok(warp::reply::json(&block_detail))
    } else {
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
                                                                    for tx in txs {
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

                                                                            // 从 `vin` 中获取 `prevout` 的 `value` 属性
                                                                            let value = vin.prevout.as_ref().map_or(0, |prevout| prevout.value);

                                                                            println!("Inserting input with value: {}", value);  // 打印插入的值

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



                                                                        for vout in tx.vout {
                                                                            let latest_output: Option<TransactionOutput> = transaction_outputs::table
                                                                                .order(transaction_outputs::id.desc())
                                                                                .first(&mut conn)
                                                                                .optional()
                                                                                .expect("Error querying latest transaction output");

                                                                            let new_output_id = latest_output.as_ref().map_or(1, |latest| latest.id + 1);

                                                                            let address = vout.script_pub_key
                                                                                .as_ref()
                                                                                .map(|script| script.addresses.join(", "))
                                                                                .unwrap_or_default();

                                                                            let new_output = TransactionOutput {
                                                                                id: new_output_id,
                                                                                transaction_id: new_tx_id,
                                                                                address,
                                                                                value: vout.value as i64,
                                                                            };

                                                                            match diesel::insert_into(transaction_outputs::table)
                                                                                .values(&new_output)
                                                                                .execute(&mut conn)
                                                                            {
                                                                                Ok(_) => println!("Successfully inserted new transaction output"),
                                                                                Err(e) => eprintln!("Error inserting transaction output: {}", e),
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
            }
        }

        *is_fetching_guard = false;
    }
}
