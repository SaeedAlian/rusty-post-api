use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::Post;

#[derive(Clone, Debug)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}

#[async_trait]
pub trait PostExt {
    async fn get_post(&self, post_id: Option<Uuid>) -> Result<Option<Post>, sqlx::Error>;

    async fn get_posts(&self) -> Result<Vec<Post>, sqlx::Error>;

    async fn save_post<T: Into<String> + Send>(
        &self,
        title: T,
        description: T,
    ) -> Result<Post, sqlx::Error>;
}

#[async_trait]
impl PostExt for DBClient {
    async fn get_post(&self, post_id: Option<Uuid>) -> Result<Option<Post>, sqlx::Error> {
        let mut post: Option<Post> = None;

        if let Some(post_id) = post_id {
            post = sqlx::query_as!(
                Post,
                r#"SELECT id, title, description, created_at, updated_at FROM posts WHERE id = $1"#,
                post_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(post)
    }

    async fn get_posts(&self) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"SELECT id, title, description, created_at, updated_at FROM posts"#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    async fn save_post<T: Into<String> + Send>(
        &self,
        title: T,
        description: T,
    ) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"INSERT INTO posts (title, description) VALUES ($1, $2) RETURNING id, title, description, created_at, updated_at"#,
            title.into(),
            description.into(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }
}
