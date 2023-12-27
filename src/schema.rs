// @generated automatically by Diesel CLI.

diesel::table! {
    comics (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        status -> Varchar,
    }
}
