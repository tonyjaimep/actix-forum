use actix_web::error;
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;

use super::models::NewPost;
use super::utils::*;
use crate::db_pool::DbPool;

#[derive(Deserialize, Clone)]
struct CreatePostDTO {
    pub content: String,
    pub thread_id: i32,
}

async fn create(
    create_post_dto: web::Json<CreatePostDTO>,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let post_input = create_post_dto.into_inner();

    let new_post = NewPost {
        content: post_input.content,
        thread_id: post_input.thread_id,
    };

    let created_post = create_post(new_post, db_pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Created().json(created_post))
}

pub fn setup() -> Scope {
    web::scope("/posts").service(web::resource("").route(web::post().to(create)))
}
