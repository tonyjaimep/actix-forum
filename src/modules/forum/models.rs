use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::forums;

#[derive(Clone, Serialize, Deserialize, Queryable, PartialEq, Insertable, Selectable)]
pub struct Forum {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}
