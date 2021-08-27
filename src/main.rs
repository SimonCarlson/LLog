pub mod models;
use models::WorkoutFormData;

#[macro_use]
extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpServer, Responder};
use askama_actix::Template;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::info;
use std::env;

#[derive(Template)]
#[template(path = "new.html")]
struct NewTemplate {}

#[get("/new")]
async fn new_workout() -> impl Responder {
  NewTemplate {}
}

#[derive(Template)]
#[template(path = "workout.html")]
struct WorkoutTemplate {
  name: String,
  date: String,
}

#[post("/new")]
async fn create_new_workout<'a>(form: web::Form<WorkoutFormData>) -> impl Responder {
  info!("Got form {:?}", form);
  WorkoutTemplate {
    name: form.name.clone(),
    date: form.date.clone(),
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  Builder::from_env(Env::default().default_filter_or("info")).init();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

  HttpServer::new(|| {
    App::new().wrap(Logger::default()).service(
      web::scope("/app")
        .service(new_workout)
        .service(create_new_workout),
    )
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
