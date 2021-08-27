ALTER TABLE workouts ADD date DATE NOT NULL;
ALTER TABLE workouts ADD UNIQUE (name, date);
ALTER TABLE workouts DROP CONSTRAINT IF EXISTS workouts_created_at_name_key;
