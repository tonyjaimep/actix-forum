// @generated automatically by Diesel CLI.

diesel::table! {
    forums (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Nullable<Varchar>,
    }
}
