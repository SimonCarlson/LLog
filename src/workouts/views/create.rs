use actix_web::{get, http::header, post, web, HttpRequest, HttpResponse, Responder};
use askama_actix::Template;
use chrono::NaiveDate;
use diesel::prelude::*;
use log::info;

use crate::workouts::models::{NewWorkout, Workout, WorkoutFormData};
use crate::AppState;

#[derive(Template)]
#[template(path = "new.html")]
struct NewTemplate {}

#[get("/new")]
async fn new_workout() -> impl Responder {
  NewTemplate {}
}

#[post("/new")]
async fn create_new_workout<'a>(
  (req, state, form): (HttpRequest, web::Data<AppState>, web::Json<WorkoutFormData>),
) -> impl Responder {
  use crate::schema::workouts::dsl::*;

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
  let url = req.url_for("view_workout", &[row_id.to_string()]).unwrap();
  HttpResponse::Found()
    .header(header::LOCATION, url.as_str())
    .finish()
}
