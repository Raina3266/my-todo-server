// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Uuid,
        message -> Text,
    }
}
