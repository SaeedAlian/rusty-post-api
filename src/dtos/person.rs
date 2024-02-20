use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::User;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    #[validate(length(min = 1, message = "Firstname is required"))]
    pub firstname: String,

    #[validate(length(min = 1, message = "Lastname is required"))]
    pub lastname: String,

    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,

    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateAdminDto {
    #[validate(length(min = 1, message = "Firstname is required"))]
    pub firstname: String,

    #[validate(length(min = 1, message = "Lastname is required"))]
    pub lastname: String,

    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,

    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateUserPublicInfoDto {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateAdminPublicInfoDto {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchUserQueryDto {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<usize>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchAdminQueryDto {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl UserDto {
    pub fn filter_user(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            firstname: user.firstname.to_owned(),
            lastname: user.lastname.to_owned(),
            username: user.username.to_owned(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn filter_users(users: &[User]) -> Vec<Self> {
        users.iter().map(Self::filter_user).collect()
    }
}

#[derive(Deserialize)]
pub struct GetUserParamsDto {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: u16,
    pub user: UserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: u16,
    pub users: Vec<UserDto>,
    pub results: usize,
}
