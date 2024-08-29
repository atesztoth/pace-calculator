// @generated automatically by Diesel CLI.

diesel::table! {
    calculations (id) {
        id -> Text,
        time -> Integer,
        distance -> Integer,
        pace -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
