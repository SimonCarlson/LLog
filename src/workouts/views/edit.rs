use actix_web::{post, web, HttpResponse, Responder};
use askama_actix::Template;
use diesel::prelude::*;
use log::info;

use crate::workouts::models::{ExerciseFormData, Workout};
use crate::AppState;

#[derive(Template)]
#[template(path = "edit_workout.html")]
struct EditWorkoutTemplate {
  workout: Workout,
}

pub async fn view_workout(
  (state, web::Path(workout_id)): (web::Data<AppState>, web::Path<i32>),
) -> impl Responder {
  use crate::schema::workouts::dsl::*;

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

#[post("/workout/{id}")]
pub async fn update_workout(
  (req, _state, web::Path(_workout_id)): (
    web::Json<ExerciseFormData>,
    web::Data<AppState>,
    web::Path<i32>,
  ),
) -> impl Responder {
  info!("Req {:?}", req);

  HttpResponse::Ok()
}
