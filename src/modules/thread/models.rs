use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::modules::post::models::NewPost;
use crate::schema::threads;

#[derive(
    AsChangeset, Clone, Deserialize, Insertable, PartialEq, Queryable, Selectable, Serialize,
)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub forum_id: String,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = threads)]
pub struct NewThread {
    pub title: String,
    pub forum_id: String,
}

impl Thread {
    pub fn new_post(&self, post_content: String) -> NewPost {
        NewPost {
            thread_id: self.id,
            content: post_content,
        }
    }
}
