table! {
    workouts (id) {
        id -> Int4,
        created_at -> Timestamptz,
        name -> Varchar,
        session_rpe -> Nullable<Float8>,
        note -> Nullable<Varchar>,
    }
}
