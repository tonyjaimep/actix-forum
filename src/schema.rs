// @generated automatically by Diesel CLI.

diesel::table! {
    forums (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        content -> Text,
        thread_id -> Int4,
    }
}

diesel::table! {
    threads (id) {
        id -> Int4,
        title -> Varchar,
        forum_id -> Varchar,
    }
}

diesel::joinable!(posts -> threads (thread_id));
diesel::joinable!(threads -> forums (forum_id));

diesel::allow_tables_to_appear_in_same_query!(
    forums,
    posts,
    threads,
);
