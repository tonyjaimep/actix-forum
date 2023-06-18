#[macro_export]
macro_rules! build_get_all {
    ($name: tt, $table: ident, $T: ty) => {
        pub async fn $name(
            db_pool: actix_web::web::Data<crate::db_pool::DbPool>,
        ) -> Result<Vec<$T>, actix_web::error::BlockingError> {
            actix_web::web::block(move || {
                use crate::schema::$table::dsl::*;
                use diesel::RunQueryDsl;

                let mut connection = db_pool
                    .get()
                    .expect("Could not get database connection from pool");

                $table
                    .load::<$T>(&mut connection)
                    .expect("Could not load {$table}")
            })
            .await
        }
    };
}

#[macro_export]
macro_rules! build_create_one {
    ($name: tt, $table: ident, $InputType: ty, $OutputType: ty) => {
        pub async fn $name(
            to_create: $InputType,
            db_pool: actix_web::web::Data<crate::db_pool::DbPool>,
        ) -> Result<$OutputType, actix_web::error::BlockingError> {
            actix_web::web::block(move || {
                use crate::schema::$table::dsl::*;
                use diesel::RunQueryDsl;

                let mut connection = db_pool
                    .get()
                    .expect("Could not get database connection from pool");

                diesel::insert_into($table)
                    .values(to_create)
                    .get_result::<$OutputType>(&mut connection)
                    .expect("Unable to insert into table")
            })
            .await
        }
    };
}

#[macro_export]
macro_rules! build_find_one {
    ($name: tt, $table: ident, $T: ty, $IdType: ty) => {
        pub async fn $name(
            lookup_id: $IdType,
            db_pool: actix_web::web::Data<crate::db_pool::DbPool>,
        ) -> Result<$T, actix_web::error::BlockingError> {
            actix_web::web::block(move || {
                use crate::schema::$table::dsl::*;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;

                let mut connection = db_pool
                    .get()
                    .expect("Could not get database connection from pool");

                $table
                    .find(lookup_id)
                    .first::<$T>(&mut connection)
                    .expect("Error loading entity")
            })
            .await
        }
    };
}

#[macro_export]
macro_rules! build_update {
    ($name: tt, $table: ident, $T: ty, $IdType: ty) => {
        pub async fn $name(
            lookup_id: $IdType,
            updated_item: $T,
            db_pool: actix_web::web::Data<crate::db_pool::DbPool>,
        ) -> Result<$T, actix_web::error::BlockingError> {
            actix_web::web::block(move || {
                use crate::schema::$table::dsl::*;
                use diesel::QueryDsl;
                use diesel::RunQueryDsl;

                let mut connection = db_pool
                    .get()
                    .expect("Could not get database connection from pool");

                diesel::update($table.find(lookup_id))
                    .set(updated_item)
                    .get_result::<$T>(&mut connection)
                    .expect("Could not update {$table}")
            })
            .await
        }
    };
}
