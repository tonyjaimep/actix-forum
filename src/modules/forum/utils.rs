use actix_web::error;
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use super::models::Forum;
use crate::db_pool::DbPool;

#[derive(Deserialize)]
pub struct CreateForumDTO {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateForumDTO {
    pub title: String,
    pub description: Option<String>,
}

pub async fn get_all_forums(
    db_pool: web::Data<DbPool>,
) -> Result<Vec<Forum>, error::BlockingError> {
    web::block(move || {
        use crate::schema::forums::dsl::*;

        let mut connection = db_pool
            .get()
            .expect("Could not get database connection from pool");

        forums
            .load::<Forum>(&mut connection)
            .expect("Could not load forums")
    })
    .await
}

pub async fn create_forum(
    forum_to_create: Forum,
    db_pool: web::Data<DbPool>,
) -> Result<usize, error::BlockingError> {
    web::block(move || {
        use crate::schema::forums::dsl::*;

        let mut connection = db_pool
            .get()
            .expect("Could not get database connection from pool");

        diesel::insert_into(forums)
            .values(vec![forum_to_create])
            .execute(&mut connection)
            .expect("Unable to insert into forums")
    })
    .await
}

pub async fn find_one_forum(
    forum_id: String,
    db_pool: web::Data<DbPool>,
) -> Result<Option<Forum>, error::BlockingError> {
    web::block(move || {
        use crate::schema::forums::dsl::*;
        use diesel::OptionalExtension;

        let mut connection = db_pool
            .get()
            .expect("Could not get database connection from pool");

        forums
            .filter(id.eq(forum_id))
            .first(&mut connection)
            .optional()
            .unwrap()
    })
    .await
}

pub async fn update_forum(
    forum_id: String,
    updated_forum: UpdateForumDTO,
    db_pool: web::Data<DbPool>,
) -> Result<Forum, error::BlockingError> {
    web::block(move || {
        use crate::schema::forums::dsl::*;

        let mut connection = db_pool
            .get()
            .expect("Could not get database connection from pool");

        diesel::update(forums.find(forum_id))
            .set((
                title.eq(updated_forum.title),
                description.eq(updated_forum.description),
            ))
            .get_result::<Forum>(&mut connection)
            .expect("Could not update forum")
    })
    .await
}
