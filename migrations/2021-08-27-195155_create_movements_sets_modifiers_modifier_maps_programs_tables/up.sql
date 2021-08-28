CREATE TABLE IF NOT EXISTS movements (
  id SERIAL PRIMARY KEY,
  uses_weight BOOLEAN NOT NULL DEFAULT TRUE,
  uses_distance BOOLEAN NOT NULL DEFAULT FALSE,
  uses_duration BOOLEAN NOT NULL DEFAULT FALSE,
  name VARCHAR(100),

  UNIQUE(name)
);

ALTER TABLE exercises ADD IF NOT EXISTS movement_id INT NOT NULL REFERENCES movements(id);

CREATE TABLE IF NOT EXISTS sets (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  exercise_id INT NOT NULL REFERENCES exercises(id),
  reps INT,
  weight FLOAT,
  rpe FLOAT,
  duration INTERVAL,
  distance INT,
  ordinal INT NOT NULL
);

CREATE TYPE modifier_units AS ENUM ('kg', 'cm');

CREATE TABLE IF NOT EXISTS modifiers (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  prefix BOOLEAN NOT NULL,
  unit MODIFIER_UNITS,

  UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS modifier_maps (
  id SERIAL PRIMARY KEY,
  modifier_id INT NOT NULL REFERENCES modifiers(id),
  exercise_id INT NOT NULL REFERENCES exercises(id),
  value FLOAT
);

CREATE TABLE IF NOT EXISTS programs (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100),

  UNIQUE(name)
);

ALTER TABLE workouts ADD IF NOT EXISTS program_id INT REFERENCES programs(id);
