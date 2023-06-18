use actix_web::error;
use actix_web::web;

use super::models::*;
use crate::build_create_one;
use crate::db_pool::DbPool;

build_create_one!(create_post, posts, NewPost, Post);

pub async fn get_thread_posts(
    post_thread_id: i32,
    db_pool: web::Data<DbPool>,
) -> Result<Vec<Post>, error::BlockingError> {
    use crate::schema::posts::all_columns;
    use crate::schema::posts::dsl::*;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    web::block(move || {
        let mut connection = db_pool
            .get()
            .expect("Could not get database connection from pool");

        posts
            .select(all_columns)
            .filter(thread_id.eq(post_thread_id))
            .get_results::<Post>(&mut connection)
            .expect("Could not get posts from thread {post_thread_id}")
    })
    .await
}
