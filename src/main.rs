#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
use crate::models::{NewWorkout, Workout};
use models::WorkoutFormData;

use actix_web::middleware::Logger;
use actix_web::{
  get, guard, http::header, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use askama_actix::Template;
use chrono::naive::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::info;
use serde::Deserialize;
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
async fn create_new_workout<'a>(
  (req, state, form): (HttpRequest, web::Data<AppState>, web::Json<WorkoutFormData>),
) -> impl Responder {
  use schema::workouts::dsl::*;

  info!("Got form {:?}", form);
  let connection = &state
    .db_pool
    .get()
    .expect("Failed to get database connection.");

  let new_note = match &form.note {
    Some(inner) => Some(inner.as_str()),
    None => None,
  };

  let new_workout_struct = NewWorkout {
    name: &form.name,
    session_rpe: form.session_rpe,
    note: new_note,
    date: NaiveDate::parse_from_str(&form.date, "%Y-%m-%d").expect("Failed to parse date."),
    program_id: None,
  };

  let result = diesel::insert_into(workouts)
    .values(&new_workout_struct)
    .get_result::<Workout>(connection)
    .expect("Error saving new workout.");
  info!("Got result {:?}", result);

  let row_id = result.id;
  let url = req.url_for("edit_workout", &[row_id.to_string()]).unwrap();
  HttpResponse::Found()
    .header(header::LOCATION, url.as_str())
    .finish()
}

#[derive(Template)]
#[template(path = "edit_workout.html")]
struct EditWorkoutTemplate {
  workout: Workout,
}

async fn edit_workout(
  (state, web::Path(workout_id)): (web::Data<AppState>, web::Path<i32>),
) -> impl Responder {
  use schema::workouts::dsl::*;

  info!("Editing workout {}", workout_id);

  let connection = &state
    .db_pool
    .get()
    .expect("Failed to get database connection.");
  let mut result = workouts
    .filter(id.eq(workout_id))
    .load::<Workout>(connection)
    .expect("Error loading workout");
  EditWorkoutTemplate {
    workout: result.swap_remove(0), // FIXME: Potential panic if no result
  }
}

#[derive(Deserialize, Debug)]
struct ExerciseData {
  movements: Vec<MovementData>,
}

#[derive(Deserialize, Debug)]
struct MovementData {
  movement: String,
  sets: Vec<SetData>,
}

#[derive(Deserialize, Debug)]
struct SetData {
  weight: i32,
  reps: i32,
  rpe: Option<f64>,
}

#[post("/workout/{id}")]
async fn update_workout(
  (req, state, web::Path(workout_id)): (
    web::Json<ExerciseData>,
    web::Data<AppState>,
    web::Path<i32>,
  ),
) -> impl Responder {
  use schema::workouts::dsl::*;
  info!("Req {:?}", req);

  HttpResponse::Ok()
}

struct AppState {
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
      .service(
        web::scope("/app")
          .service(new_workout)
          .service(create_new_workout)
          .service(update_workout)
          .service(
            web::resource("/workout/{id}")
              .name("edit_workout")
              .guard(guard::Get())
              .route(web::get().to(edit_workout)),
          ),
      )
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
