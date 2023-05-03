// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        profile_token -> Varchar,
        name -> Varchar,
        surname -> Varchar,
        email -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
