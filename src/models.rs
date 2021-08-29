use chrono::naive::{NaiveDate, NaiveDateTime};
use serde::Deserialize;

#[derive(Queryable, Debug)]
pub struct Workout {
  pub id: i32,
  pub created_at: NaiveDateTime,
  pub name: String,
  pub session_rpe: Option<f64>,
  pub note: Option<String>,
  pub date: NaiveDate,
  pub program_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct WorkoutFormData {
  pub name: String,
  pub date: String,
  pub session_rpe: f64,
  pub note: String,
}

#[derive(Queryable)]
pub struct Exercise {
  pub id: i32,
  pub workout_id: i32,
  pub created_at: String,
  pub name: String,
  pub note: String,
  pub ordinal: i32,
  pub date: String,
  pub movement_id: i32,
}

#[derive(Queryable)]
pub struct Set {
  pub id: i32,
  pub created_at: String,
  pub exercise_id: i32,
  pub reps: i32,
  pub weight: f64,
  pub rpe: f64,
  pub duration: String,
  pub distance: i32,
  pub ordinal: i32,
}

#[derive(Queryable)]
pub struct Movement {
  pub id: i32,
  pub uses_weight: bool,
  pub uses_distance: bool,
  pub uses_duration: bool,
  pub name: String,
}

pub enum ModifierUnit {
  CM,
  KG,
}

#[derive(Queryable)]
pub struct Modifier {
  pub id: i32,
  pub name: String,
  pub prefix: bool,
  pub unit: ModifierUnit,
}

#[derive(Queryable)]
pub struct ModifierMap {
  pub id: i32,
  pub modifier_id: i32,
  pub exercise_id: i32,
  pub value: f64,
}

#[derive(Queryable)]
pub struct Program {
  pub id: i32,
  pub name: String,
}
