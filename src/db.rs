use async_trait::async_trait;
use sqlx::{Pool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    dtos::{CreatePostDto, SearchPostQueryDto, UpdatePostDto},
    models::Post,
};

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
    async fn get_post(&self, post_id: Uuid) -> Result<Option<Post>, sqlx::Error>;

    async fn get_posts(&self, query: SearchPostQueryDto) -> Result<Vec<Post>, sqlx::Error>;

    async fn save_post(&self, dto: CreatePostDto) -> Result<Post, sqlx::Error>;

    async fn update_post(&self, post_id: Uuid, dto: UpdatePostDto) -> Result<bool, sqlx::Error>;

    async fn delete_post(&self, post_id: Uuid) -> Result<bool, sqlx::Error>;
}

#[async_trait]
impl PostExt for DBClient {
    async fn get_post(&self, post_id: Uuid) -> Result<Option<Post>, sqlx::Error> {
        let post = sqlx::query_as!(Post, r#"SELECT * FROM posts WHERE id = $1"#, post_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(post)
    }

    async fn get_posts(&self, query: SearchPostQueryDto) -> Result<Vec<Post>, sqlx::Error> {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(6);
        let offset: u32 = (page - 1) * limit as u32;

        let mut query_builder = QueryBuilder::new(r#"SELECT * FROM posts"#);

        if let Some(title) = query.title {
            query_builder.push(" WHERE ");

            query_builder.push(" title like ");
            query_builder.push_bind(format!("%{}%", title));
        }

        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);

        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit as i64);

        let posts = query_builder.build_query_as().fetch_all(&self.pool).await?;

        Ok(posts)
    }

    async fn save_post(&self, dto: CreatePostDto) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"INSERT INTO posts (title, description) VALUES ($1, $2) RETURNING *"#,
            dto.title,
            dto.description,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    async fn update_post(&self, post_id: Uuid, dto: UpdatePostDto) -> Result<bool, sqlx::Error> {
        let mut is_updated: bool = false;

        let mut query_builder = QueryBuilder::new(r#"UPDATE posts"#);

        let mut is_using_dto = false;

        if let Some(title) = dto.title {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            }

            query_builder.push("title = ");
            query_builder.push_bind(title);
        }

        if let Some(description) = dto.description {
            if !is_using_dto {
                query_builder.push(" SET ");
            } else {
                query_builder.push(",");
            }

            query_builder.push("description = ");
            query_builder.push_bind(description);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(post_id);

        let result = query_builder.build().execute(&self.pool).await?;

        if result.rows_affected() > 0 {
            is_updated = true;
        }

        Ok(is_updated)
    }

    async fn delete_post(&self, post_id: Uuid) -> Result<bool, sqlx::Error> {
        let mut is_deleted = false;

        let result = sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 {
            is_deleted = true;
        }

        Ok(is_deleted)
    }
}
