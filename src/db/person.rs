use async_trait::async_trait;
use chrono::DateTime;
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    dtos::person::{
        CreateAdminDto, CreateUserDto, SearchAdminQueryDto, SearchUserQueryDto, UpdateAdminDto,
        UpdateUserDto,
    },
    models::{Admin, Email, Gender, Person, User},
};

use super::DBClient;

#[async_trait]
pub trait PersonExt {
    async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn get_admin(&self, admin_id: Uuid) -> Result<Option<Admin>, sqlx::Error>;

    async fn get_users(
        &self,
        query: SearchUserQueryDto,
        fetch_emails: bool,
    ) -> Result<Vec<User>, sqlx::Error>;

    async fn get_admins(
        &self,
        query: SearchAdminQueryDto,
        fetch_emails: bool,
    ) -> Result<Vec<Admin>, sqlx::Error>;

    async fn save_user(&self, dto: CreateUserDto) -> Result<User, sqlx::Error>;

    async fn save_admin(&self, dto: CreateAdminDto) -> Result<Admin, sqlx::Error>;

    async fn update_user(&self, user_id: Uuid, dto: UpdateUserDto) -> Result<bool, sqlx::Error>;

    async fn update_admin(&self, admin_id: Uuid, dto: UpdateAdminDto) -> Result<bool, sqlx::Error>;

    async fn delete_user(&self, user_id: Uuid) -> Result<bool, sqlx::Error>;

    async fn delete_admin(&self, admin_id: Uuid) -> Result<bool, sqlx::Error>;
}

#[async_trait]
impl PersonExt for DBClient {
    async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let person: Option<Person> =
            sqlx::query_as(r#"SELECT * FROM people WHERE id = $1 AND role = 'user'"#)
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await?;

        if let Some(mut person) = person {
            let emails = sqlx::query_as!(
                Email,
                r#"SELECT * FROM emails WHERE owner_id = $1"#,
                person.id
            )
            .fetch_all(&self.pool)
            .await?;

            person.emails = emails;

            Ok(Some(User::from(person)))
        } else {
            Ok(None)
        }
    }

    async fn get_admin(&self, admin_id: Uuid) -> Result<Option<Admin>, sqlx::Error> {
        let person: Option<Person> =
            sqlx::query_as(r#"SELECT * FROM people WHERE id = $1 AND role = 'admin'"#)
                .bind(admin_id)
                .fetch_optional(&self.pool)
                .await?;

        if let Some(mut person) = person {
            let emails = sqlx::query_as!(
                Email,
                r#"SELECT * FROM emails WHERE owner_id = $1"#,
                person.id
            )
            .fetch_all(&self.pool)
            .await?;

            person.emails = emails;

            Ok(Some(Admin::from(person)))
        } else {
            Ok(None)
        }
    }

    async fn get_users(
        &self,
        query: SearchUserQueryDto,
        fetch_emails: bool,
    ) -> Result<Vec<User>, sqlx::Error> {
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

        let people: Vec<Person> = query_builder.build_query_as().fetch_all(&self.pool).await?;

        let mut result: Vec<User> = vec![];

        for mut person in people.into_iter() {
            if fetch_emails {
                let emails = sqlx::query_as!(
                    Email,
                    r#"SELECT * FROM emails WHERE owner_id = $1"#,
                    person.id
                )
                .fetch_all(&self.pool)
                .await?;

                person.emails = emails;
            }

            result.push(User::from(person))
        }

        Ok(result)
    }

    async fn get_admins(
        &self,
        query: SearchAdminQueryDto,
        fetch_emails: bool,
    ) -> Result<Vec<Admin>, sqlx::Error> {
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

        let people: Vec<Person> = query_builder.build_query_as().fetch_all(&self.pool).await?;

        let mut result: Vec<Admin> = vec![];

        for mut person in people.into_iter() {
            if fetch_emails {
                let emails = sqlx::query_as!(
                    Email,
                    r#"SELECT * FROM emails WHERE owner_id = $1"#,
                    person.id
                )
                .fetch_all(&self.pool)
                .await?;

                person.emails = emails;
            }

            result.push(Admin::from(person))
        }

        Ok(result)
    }

    async fn save_user(&self, dto: CreateUserDto) -> Result<User, sqlx::Error> {
        let mut new_person: Person = sqlx::query_as(r#"
            INSERT INTO people 
                (firstname, lastname, username, gender, is_profile_private, birthdate, password, role)
                VALUES ($1, $2, $3, $4, $5, $6, $7, 'user')
                RETURNING *
        "#)
            .bind(dto.firstname)
            .bind(dto.lastname)
            .bind(dto.username)
            .bind(Gender::from(dto.gender))
            .bind(dto.is_profile_private)
            .bind(DateTime::parse_from_str(&dto.birthdate, "%Y-%m-%d").unwrap())
            .bind(dto.password)
            .fetch_one(&self.pool).await?;

        let email = sqlx::query_as!(
        Email,
        r#"INSERT INTO emails (address, owner_id, is_primary, is_private) VALUES ($1, $2, true, true) RETURNING *"#,
        dto.email, new_person.id)
            .fetch_one(&self.pool)
            .await?;

        new_person.emails = vec![email];

        Ok(User::from(new_person))
    }

    async fn save_admin(&self, dto: CreateAdminDto) -> Result<Admin, sqlx::Error> {
        let mut new_person: Person = sqlx::query_as(
            r#"
            INSERT INTO people 
                (firstname, lastname, username, gender, birthdate, password, role)
                VALUES ($1, $2, $3, $4, $5, $6, 'user')
                RETURNING *
        "#,
        )
        .bind(dto.firstname)
        .bind(dto.lastname)
        .bind(dto.username)
        .bind(Gender::from(dto.gender))
        .bind(DateTime::parse_from_str(&dto.birthdate, "%Y-%m-%d").unwrap())
        .bind(dto.password)
        .fetch_one(&self.pool)
        .await?;

        let email = sqlx::query_as!(
        Email,
        r#"INSERT INTO emails (address, owner_id, is_primary, is_private) VALUES ($1, $2, true, true) RETURNING *"#,
        dto.email, new_person.id)
            .fetch_one(&self.pool)
            .await?;

        new_person.emails = vec![email];

        Ok(Admin::from(new_person))
    }

    async fn update_user(&self, user_id: Uuid, dto: UpdateUserDto) -> Result<bool, sqlx::Error> {
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

        if let Some(birthdate) = dto.birthdate {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            } else {
                query_builder.push(",");
            }

            query_builder.push("birthdate = ");
            query_builder.push_bind(DateTime::parse_from_str(&birthdate, "%Y-%m-%d").unwrap());
        }

        if let Some(gender) = dto.gender {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            } else {
                query_builder.push(",");
            }

            query_builder.push("gender = ");
            query_builder.push_bind(Gender::from(gender));
        }

        if let Some(is_profile_private) = dto.is_profile_private {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            } else {
                query_builder.push(",");
            }

            query_builder.push("is_profile_private = ");
            query_builder.push_bind(is_profile_private);
        }

        if let Some(biography) = dto.biography {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            } else {
                query_builder.push(",");
            }

            query_builder.push("biography = ");
            query_builder.push_bind(biography);
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

    async fn update_admin(&self, admin_id: Uuid, dto: UpdateAdminDto) -> Result<bool, sqlx::Error> {
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

        if let Some(birthdate) = dto.birthdate {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            } else {
                query_builder.push(",");
            }

            query_builder.push("birthdate = ");
            query_builder.push_bind(DateTime::parse_from_str(&birthdate, "%Y-%m-%d").unwrap());
        }

        if let Some(gender) = dto.gender {
            if !is_using_dto {
                query_builder.push(" SET ");
                is_using_dto = true;
            } else {
                query_builder.push(",");
            }

            query_builder.push("gender = ");
            query_builder.push_bind(Gender::from(gender));
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
