CREATE TYPE modifier_units AS ENUM ('kg', 'cm');
ALTER TABLE modifiers ALTER COLUMN unit TYPE modifier_units USING unit::modifier_units;
