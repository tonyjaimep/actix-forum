#[macro_export]
macro_rules! build_get_all {
    ($name: tt, $table: ident, $T: ty) => {
        pub async fn $name(
            db_pool: actix_web::web::Data<crate::db_pool::DbPool>,
        ) -> Result<Vec<$T>, actix_web::error::BlockingError> {
            web::block(move || {
                use crate::schema::$table::dsl::*;
                use diesel::RunQueryDsl;

                let mut connection = db_pool
                    .get()
                    .expect("Could not get database connection from pool");

                $table
                    .load::<$T>(&mut connection)
                    .expect("Could not load forums")
            })
            .await
        }
    };
}

#[macro_export]
macro_rules! build_create_one {
    ($name: tt, $table: ident, $T: ty) => {
        pub async fn $name(
            to_create: $T,
            db_pool: actix_web::web::Data<DbPool>,
        ) -> Result<usize, actix_web::error::BlockingError> {
            web::block(move || {
                use crate::schema::$table::dsl::*;

                let mut connection = db_pool
                    .get()
                    .expect("Could not get database connection from pool");

                diesel::insert_into($table)
                    .values(vec![to_create])
                    .execute(&mut connection)
                    .expect("Unable to insert into table")
            })
            .await
        }
    };
}

#[macro_export]
macro_rules! build_find_one {
    ($name: tt, $table: ident, $T: ty) => {
        pub async fn $name(
            lookup_id: String,
            db_pool: actix_web::web::Data<DbPool>,
        ) -> Result<Option<$T>, actix_web::error::BlockingError> {
            actix_web::web::block(move || {
                use crate::schema::$table::dsl::*;
                use diesel::OptionalExtension;

                let mut connection = db_pool
                    .get()
                    .expect("Could not get database connection from pool");

                $table
                    .filter(id.eq(lookup_id))
                    .first(&mut connection)
                    .optional()
                    .unwrap()
            })
            .await
        }
    };
}

#[macro_export]
macro_rules! build_update {
    ($name: tt, $table: ident, $T: ty) => {
        pub async fn $name(
            lookup_id: String,
            updated_item: $T,
            db_pool: actix_web::web::Data<DbPool>,
        ) -> Result<$T, error::BlockingError> {
            actix_web::web::block(move || {
                use crate::schema::$table::dsl::*;

                let mut connection = db_pool
                    .get()
                    .expect("Could not get database connection from pool");

                diesel::update(forums.find(lookup_id))
                    .set(updated_item)
                    .get_result::<Forum>(&mut connection)
                    .expect("Could not update forum")
            })
            .await
        }
    };
}
