use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Database {
    pub db_pool: PgPool,
}
