use serde::Deserialize;

#[derive(Queryable)]
pub struct Workout {
  pub id: i32,
  pub created_at: String,
  pub name: String,
  pub date: String,
  pub session_rpe: f32,
  pub note: String,
}

#[derive(Deserialize, Debug)]
pub struct WorkoutFormData {
  pub name: String,
  pub date: String,
  pub session_rpe: f32,
  pub note: String,
}
