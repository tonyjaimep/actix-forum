use actix_web::error;
use actix_web::{web, HttpResponse, Responder, Scope};

use super::models::Forum;
use super::utils::*;
use crate::db_pool::DbPool;

async fn find_all(db_pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let all_forums = get_all_forums(db_pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(all_forums))
}

async fn create(
    create_forum_dto: web::Json<CreateForumDTO>,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let forum_input = create_forum_dto.into_inner();
    let forum_to_create = Forum {
        id: forum_input.id,
        title: forum_input.title,
        description: Some(forum_input.description),
    };

    let forum_output = forum_to_create.clone();

    create_forum(forum_to_create, db_pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Created().json(forum_output))
}

async fn find_one(path: web::Path<String>, db_pool: web::Data<DbPool>) -> impl Responder {
    let forum_id = path.into_inner();
    let forum_id_copy = forum_id.clone();

    let forum = find_one_forum(forum_id, db_pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    match forum {
        Some(forum) => Ok::<HttpResponse, actix_web::Error>(HttpResponse::Ok().json(forum)),
        None => {
            Ok(HttpResponse::NotFound()
                .body(format!("Could not find forum with id {forum_id_copy}")))
        }
    }
}

async fn update(
    path: web::Path<String>,
    updated_forum: web::Json<UpdateForumDTO>,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let forum_id = path.into_inner();

    let updated_forum = updated_forum.into_inner();

    let forum = update_forum(forum_id, updated_forum, db_pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Ok().json(forum))
}

pub fn setup() -> Scope {
    web::scope("/forums")
        .service(
            web::resource("")
                .route(web::get().to(find_all))
                .route(web::post().to(create)),
        )
        .service(
            web::resource("/{forum_id}")
                .route(web::get().to(find_one))
                .route(web::patch().to(update)),
        )
}
