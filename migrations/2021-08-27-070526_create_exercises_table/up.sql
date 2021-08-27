CREATE TABLE exercises (
  id SERIAL PRIMARY KEY,
  workout_id INT NOT NULL REFERENCES workouts(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  name VARCHAR(100) NOT NULL,
  note VARCHAR(1000),
  ordinal INT NOT NULL,
  date DATE NOT NULL,

  UNIQUE (name, date)
  )
