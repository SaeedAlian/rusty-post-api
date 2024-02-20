use sqlx::{Pool, Postgres};

pub mod person;
pub mod post;

#[derive(Clone, Debug)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}
