use diesel::{r2d2, PgConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
