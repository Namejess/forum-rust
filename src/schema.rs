// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Unsigned<Integer>,
        content -> Text,
        author -> Varchar,
    }
}
