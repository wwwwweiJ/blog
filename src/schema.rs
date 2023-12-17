// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        password -> Text,
        email -> Varchar,
    }
}
