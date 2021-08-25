CREATE TABLE workouts (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  name VARCHAR(100) NOT NULL,
  session_RPE float,
  note VARCHAR(1000),

  UNIQUE(created_at, name)
)
