use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(
    AsChangeset, Clone, Deserialize, Insertable, PartialEq, Queryable, Selectable, Serialize,
)]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub thread_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub thread_id: i32,
    pub content: String,
}
