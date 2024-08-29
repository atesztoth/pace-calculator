// @generated automatically by Diesel CLI.

diesel::table! {
    calculations (id) {
        id -> Text,
        time -> Float,
        distance -> Float,
        pace -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
