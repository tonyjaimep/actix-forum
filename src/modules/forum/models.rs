use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::forums;

#[derive(
    AsChangeset, Clone, Deserialize, Insertable, PartialEq, Queryable, Selectable, Serialize,
)]
pub struct Forum {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}
