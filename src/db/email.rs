use async_trait::async_trait;
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    dtos::email::{CreateEmailDto, UpdateEmailDto},
    models::Email,
};

use super::DBClient;

#[async_trait]
pub trait EmailExt {
    async fn get_person_emails(&self, person_id: Uuid) -> Result<Vec<Email>, sqlx::Error>;

    async fn get_email_by_id(&self, email_id: Uuid) -> Result<Option<Email>, sqlx::Error>;

    async fn save_email(&self, owner_id: Uuid, dto: CreateEmailDto) -> Result<Email, sqlx::Error>;

    async fn update_email(
        &self,
        owner_id: Uuid,
        email_id: Uuid,
        dto: UpdateEmailDto,
    ) -> Result<bool, sqlx::Error>;

    async fn delete_email(&self, person_id: Uuid, email_id: Uuid) -> Result<bool, sqlx::Error>;
}

#[async_trait]
impl EmailExt for DBClient {
    async fn get_person_emails(&self, person_id: Uuid) -> Result<Vec<Email>, sqlx::Error> {
        let emails = sqlx::query_as!(
            Email,
            r#"SELECT * FROM emails WHERE owner_id = $1"#,
            person_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(emails)
    }

    async fn get_email_by_id(&self, email_id: Uuid) -> Result<Option<Email>, sqlx::Error> {
        let email = sqlx::query_as!(Email, r#"SELECT * FROM emails WHERE id = $1"#, email_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(email)
    }

    async fn save_email(&self, owner_id: Uuid, dto: CreateEmailDto) -> Result<Email, sqlx::Error> {
        let new_email = sqlx::query_as!(
            Email,
            r#"
                INSERT INTO emails (owner_id, address, is_private, is_primary) VALUES ($1, $2, $3, $4) RETURNING *
            "#,
            owner_id,
            dto.address, dto.is_private, dto.is_primary
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(new_email)
    }

    async fn update_email(
        &self,
        owner_id: Uuid,
        email_id: Uuid,
        dto: UpdateEmailDto,
    ) -> Result<bool, sqlx::Error> {
        let mut is_updated: bool = false;

        let mut query_builder = QueryBuilder::new("UPDATE emails");

        let mut is_using_dto = false;

        if let Some(address) = dto.address {
            is_using_dto = true;

            query_builder.push(" SET ");
            query_builder.push("address = ");
            query_builder.push_bind(address);
        }

        if let Some(is_private) = dto.is_private {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            } else {
                query_builder.push(", ");
            }

            query_builder.push("is_private = ");
            query_builder.push_bind(is_private);
        }

        if let Some(is_primary) = dto.is_primary {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            } else {
                query_builder.push(", ");
            }

            query_builder.push("is_primary = ");
            query_builder.push_bind(is_primary);
        }

        if let Some(is_verified) = dto.is_verified {
            if !is_using_dto {
                query_builder.push(" SET ");
            } else {
                query_builder.push(", ");
            }

            query_builder.push("is_verified = ");
            query_builder.push_bind(is_verified);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(email_id);
        query_builder.push(" AND owner_id = ");
        query_builder.push_bind(owner_id);

        let result = query_builder.build().execute(&self.pool).await?;

        if result.rows_affected() > 0 {
            is_updated = true;
        }

        Ok(is_updated)
    }

    async fn delete_email(&self, owner_id: Uuid, email_id: Uuid) -> Result<bool, sqlx::Error> {
        let mut is_deleted = false;

        let result = sqlx::query!(
            "DELETE FROM emails WHERE id = $1 AND owner_id = $2",
            email_id,
            owner_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            is_deleted = true;
        }

        Ok(is_deleted)
    }
}
