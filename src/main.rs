#[macro_use]
extern crate diesel;

pub mod schema;
pub mod workouts;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use std::env;

pub struct AppState {
  db_pool: Pool<ConnectionManager<PgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  Builder::from_env(Env::default().default_filter_or("debug")).init();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  let pool = Pool::builder()
    .max_size(10)
    .build(manager)
    .expect("Failed to create Postgres connection pool");

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .data(AppState {
        db_pool: pool.clone(),
      })
      .configure(workouts::configure)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
