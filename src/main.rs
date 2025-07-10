mod db_actor;

use actix::prelude::*;
use db_actor::{DbActor, GetUserById};
use dotenvy::dotenv;
use sqlx::PgPool;

#[actix::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("DB connection failed");

    // Start Actor
    let db_addr = DbActor { pool }.start();

    // Send Message
    let result = db_addr.send(GetUserById(2)).await;

    match result {
        Ok(Ok(Some(user))) => println!("User: {:?}", user),
        Ok(Ok(None)) => println!("No user found."),
        Ok(Err(e)) => println!("DB Error: {}", e),
        Err(e) => println!("Actor Error: {}", e),
    }

    Ok(())
}
