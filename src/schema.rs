// @generated automatically by Diesel CLI.

diesel::table! {
    calculations (id) {
        id -> Integer,
        time -> Integer,
        distance -> Integer,
        pace -> Integer,
    }
}
