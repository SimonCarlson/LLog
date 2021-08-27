table! {
    exercises (id) {
        id -> Int4,
        workout_id -> Int4,
        created_at -> Timestamptz,
        name -> Varchar,
        note -> Nullable<Varchar>,
        ordinal -> Int4,
        date -> Date,
        movement_id -> Int4,
    }
}

table! {
    modifier_maps (id) {
        id -> Int4,
        modifier_id -> Int4,
        exercise_id -> Int4,
        value -> Nullable<Float8>,
    }
}

table! {
    modifiers (id) {
        id -> Int4,
        name -> Varchar,
        prefix -> Bool,
        unit -> Nullable<Modifier_units>,
    }
}

table! {
    movements (id) {
        id -> Int4,
        uses_weight -> Bool,
        uses_time -> Bool,
        uses_duration -> Bool,
        name -> Nullable<Varchar>,
    }
}

table! {
    programs (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    sets (id) {
        id -> Int4,
        created_at -> Timestamptz,
        exercise_id -> Int4,
        reps -> Nullable<Int4>,
        weight -> Nullable<Float8>,
        rpe -> Nullable<Float8>,
        duration -> Nullable<Interval>,
        distance -> Nullable<Int4>,
        ordinal -> Int4,
    }
}

table! {
    workouts (id) {
        id -> Int4,
        created_at -> Timestamptz,
        name -> Varchar,
        session_rpe -> Nullable<Float8>,
        note -> Nullable<Varchar>,
        date -> Date,
        program_id -> Nullable<Int4>,
    }
}

joinable!(exercises -> movements (movement_id));
joinable!(exercises -> workouts (workout_id));
joinable!(modifier_maps -> exercises (exercise_id));
joinable!(modifier_maps -> modifiers (modifier_id));
joinable!(sets -> exercises (exercise_id));
joinable!(workouts -> programs (program_id));

allow_tables_to_appear_in_same_query!(
    exercises,
    modifier_maps,
    modifiers,
    movements,
    programs,
    sets,
    workouts,
);
