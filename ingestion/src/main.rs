#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

mod schema;
use schema::block_heights;

#[derive(Queryable, Identifiable, Insertable, Debug, AsChangeset)]
#[table_name = "block_heights"]
pub struct BlockHeight {
    pub id: i32,
    pub height: i32,
}

fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect("Error connecting to the database");

    // Example data to insert
    let new_height = BlockHeight { id: 1, height: 100 };

    // Inserting new record
    diesel::insert_into(block_heights::table)
        .values(&new_height)
        .execute(&connection)
        .expect("Error inserting block height");

    println!("Inserted block height: {:?}", new_height);

    // Querying all heights
    let results = block_heights::table
        .load::<BlockHeight>(&connection)
        .expect("Error loading block heights");

    println!("Current block heights:");
    for block in results {
        println!("{:?}", block);
    }
}
