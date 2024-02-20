use async_trait::async_trait;
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    dtos::person::{
        CreateAdminDto, CreateUserDto, SearchAdminQueryDto, SearchUserQueryDto,
        UpdateAdminPublicInfoDto, UpdateUserPublicInfoDto,
    },
    models::{Admin, User},
};

use super::DBClient;

#[async_trait]
pub trait PersonExt {
    async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn get_admin(&self, admin_id: Uuid) -> Result<Option<Admin>, sqlx::Error>;

    async fn get_users(&self, query: SearchUserQueryDto) -> Result<Vec<User>, sqlx::Error>;

    async fn get_admins(&self, query: SearchAdminQueryDto) -> Result<Vec<Admin>, sqlx::Error>;

    async fn save_user(&self, dto: CreateUserDto) -> Result<User, sqlx::Error>;

    async fn save_admin(&self, dto: CreateAdminDto) -> Result<Admin, sqlx::Error>;

    async fn update_user_public_info(
        &self,
        user_id: Uuid,
        dto: UpdateUserPublicInfoDto,
    ) -> Result<bool, sqlx::Error>;

    async fn update_admin_public_info(
        &self,
        admin_id: Uuid,
        dto: UpdateAdminPublicInfoDto,
    ) -> Result<bool, sqlx::Error>;

    async fn delete_user(&self, user_id: Uuid) -> Result<bool, sqlx::Error>;

    async fn delete_admin(&self, admin_id: Uuid) -> Result<bool, sqlx::Error>;
}

#[async_trait]
impl PersonExt for DBClient {
    async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as(r#"SELECT * FROM people WHERE id = $1 AND role = 'user'"#)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_admin(&self, admin_id: Uuid) -> Result<Option<Admin>, sqlx::Error> {
        let admin = sqlx::query_as(r#"SELECT * FROM people WHERE id = $1 AND role = 'admin'"#)
            .bind(admin_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(admin)
    }

    async fn get_users(&self, query: SearchUserQueryDto) -> Result<Vec<User>, sqlx::Error> {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(6);
        let offset: u32 = (page - 1) * limit as u32;

        let mut query_builder = QueryBuilder::new(r#"SELECT * FROM people WHERE (role = 'user')"#);

        let mut is_using_query = false;

        if let Some(firstname) = query.firstname {
            is_using_query = true;

            query_builder.push(" AND ( ");
            query_builder.push(" firstname LIKE ");
            query_builder.push_bind(format!("%{}%", firstname));
        }

        if let Some(lastname) = query.lastname {
            if is_using_query {
                query_builder.push(" AND ");
            } else {
                is_using_query = true;
                query_builder.push(" AND ( ");
            }

            query_builder.push(" lastname LIKE ");
            query_builder.push_bind(format!("%{}%", lastname));
        }

        if let Some(username) = query.username {
            if is_using_query {
                query_builder.push(" AND ");
            } else {
                is_using_query = true;
                query_builder.push(" AND ( ");
            }

            query_builder.push(" LOWER(username) LIKE ");
            query_builder.push_bind(format!("%{}%", username.to_lowercase()));
        }

        if is_using_query {
            query_builder.push(" ) ");
        }

        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);

        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit as i64);

        let users = query_builder.build_query_as().fetch_all(&self.pool).await?;

        Ok(users)
    }

    async fn get_admins(&self, query: SearchAdminQueryDto) -> Result<Vec<Admin>, sqlx::Error> {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(6);
        let offset: u32 = (page - 1) * limit as u32;

        let mut query_builder = QueryBuilder::new(r#"SELECT * FROM people WHERE (role = 'admin')"#);

        let mut is_using_query = false;

        if let Some(firstname) = query.firstname {
            is_using_query = true;

            query_builder.push(" AND ( ");
            query_builder.push(" firstname LIKE ");
            query_builder.push_bind(format!("%{}%", firstname));
        }

        if let Some(lastname) = query.lastname {
            if is_using_query {
                query_builder.push(" OR ");
            } else {
                is_using_query = true;
                query_builder.push(" AND ( ");
            }

            query_builder.push(" lastname LIKE ");
            query_builder.push_bind(format!("%{}%", lastname));
        }

        if let Some(username) = query.username {
            if is_using_query {
                query_builder.push(" OR ");
            } else {
                is_using_query = true;
                query_builder.push(" AND ( ");
            }

            query_builder.push(" LOWER(username) LIKE ");
            query_builder.push_bind(format!("%{}%", username.to_lowercase()));
        }

        if is_using_query {
            query_builder.push(" ) ");
        }

        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);

        query_builder.push(" LIMIT ");
        query_builder.push_bind(limit as i64);

        let admins = query_builder.build_query_as().fetch_all(&self.pool).await?;

        Ok(admins)
    }

    async fn save_user(&self, dto: CreateUserDto) -> Result<User, sqlx::Error> {
        let new_user: User = sqlx::query_as(r#"
            INSERT INTO people (firstname, lastname, username, password, role) VALUES ($1, $2, $3, $4, 'user') RETURNING *
        "#)
            .bind(dto.firstname)
            .bind(dto.lastname)
            .bind(dto.username)
            .bind(dto.password)
            .fetch_one(&self.pool).await?;

        sqlx::query_as(
            r#"
            INSERT INTO emails (address, person_id) VALUES ($1, $2) RETURNING *
        "#,
        )
        .bind(dto.email)
        .bind(new_user.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(new_user)
    }

    async fn save_admin(&self, dto: CreateAdminDto) -> Result<Admin, sqlx::Error> {
        let new_admin: Admin = sqlx::query_as(r#"
            INSERT INTO people (firstname, lastname, username, password, role) VALUES ($1, $2, $3, $4, 'admin') RETURNING *
        "#)
            .bind(dto.firstname)
            .bind(dto.lastname)
            .bind(dto.username)
            .bind(dto.password)
            .fetch_one(&self.pool).await?;

        sqlx::query_as(
            r#"
            INSERT INTO emails (address, person_id) VALUES ($1, $2) RETURNING *
        "#,
        )
        .bind(dto.email)
        .bind(new_admin.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(new_admin)
    }

    async fn update_user_public_info(
        &self,
        user_id: Uuid,
        dto: UpdateUserPublicInfoDto,
    ) -> Result<bool, sqlx::Error> {
        let mut is_updated: bool = false;

        let mut query_builder = QueryBuilder::new(r#"UPDATE people"#);

        let mut is_using_dto = false;

        if let Some(firstname) = dto.firstname {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            }

            query_builder.push("firstname = ");
            query_builder.push_bind(firstname);
        }

        if let Some(lastname) = dto.lastname {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            } else {
                query_builder.push(",");
            }

            query_builder.push("lastname = ");
            query_builder.push_bind(lastname);
        }

        if let Some(username) = dto.username {
            if !is_using_dto {
                query_builder.push(" SET ");
            } else {
                query_builder.push(",");
            }

            query_builder.push("username = ");
            query_builder.push_bind(username);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(user_id);

        query_builder.push(" AND role = 'user' ");

        let result = query_builder.build().execute(&self.pool).await?;

        if result.rows_affected() > 0 {
            is_updated = true;
        }

        Ok(is_updated)
    }

    async fn update_admin_public_info(
        &self,
        admin_id: Uuid,
        dto: UpdateAdminPublicInfoDto,
    ) -> Result<bool, sqlx::Error> {
        let mut is_updated: bool = false;

        let mut query_builder = QueryBuilder::new(r#"UPDATE people"#);

        let mut is_using_dto = false;

        if let Some(firstname) = dto.firstname {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            }

            query_builder.push("firstname = ");
            query_builder.push_bind(firstname);
        }

        if let Some(lastname) = dto.lastname {
            if !is_using_dto {
                query_builder.push(" SET ");
            } else {
                query_builder.push(",");
            }

            query_builder.push("lastname = ");
            query_builder.push_bind(lastname);
        }

        if let Some(username) = dto.username {
            if !is_using_dto {
                query_builder.push(" SET ");
            } else {
                query_builder.push(",");
            }

            query_builder.push("username = ");
            query_builder.push_bind(username);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(admin_id);

        query_builder.push(" AND role = 'admin' ");

        let result = query_builder.build().execute(&self.pool).await?;

        if result.rows_affected() > 0 {
            is_updated = true;
        }

        Ok(is_updated)
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let mut is_deleted = false;

        let result = sqlx::query!(
            "DELETE FROM people WHERE id = $1 AND role = 'user'",
            user_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            is_deleted = true;
        }

        Ok(is_deleted)
    }

    async fn delete_admin(&self, admin_id: Uuid) -> Result<bool, sqlx::Error> {
        let mut is_deleted = false;

        let result = sqlx::query!(
            "DELETE FROM people WHERE id = $1 AND role = 'admin'",
            admin_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            is_deleted = true;
        }

        Ok(is_deleted)
    }
}
