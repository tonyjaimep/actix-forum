use actix_web::error;
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;

use super::models::NewThread;
use super::utils::*;
use crate::db_pool::DbPool;
use crate::modules::post::utils::{create_post, get_thread_posts};

#[derive(Deserialize, Clone)]
struct CreateThreadDTO {
    pub title: String,
    pub forum_id: String,
    pub post_content: String,
}

async fn create(
    create_thread_dto: web::Json<CreateThreadDTO>,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let thread_input = create_thread_dto.into_inner();
    let thread_to_create = NewThread {
        title: thread_input.title,
        forum_id: thread_input.forum_id,
    };

    let db_pool_clone = db_pool.clone();

    let thread = create_thread(thread_to_create, db_pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let original_post = thread.new_post(thread_input.post_content.clone());
    create_post(original_post, db_pool_clone).await.unwrap();

    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Created().json(thread))
}

async fn find_one(path: web::Path<i32>, db_pool: web::Data<DbPool>) -> impl Responder {
    let thread_id: i32 = path.into_inner();

    let thread = find_one_thread(thread_id, db_pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Ok().json(thread))
}

async fn get_posts(path: web::Path<i32>, db_pool: web::Data<DbPool>) -> impl Responder {
    let thread_id: i32 = path.into_inner();

    let posts = get_thread_posts(thread_id, db_pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Ok().json(posts))
}

pub fn setup() -> Scope {
    web::scope("/threads")
        .service(web::resource("").route(web::post().to(create)))
        .service(web::resource("/{thread_id}").route(web::get().to(find_one)))
        .service(web::resource("/{thread_id}/posts").route(web::get().to(get_posts)))
}
