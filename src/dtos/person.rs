use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{Admin, User};

use super::email::EmailDto;

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

    #[validate(length(min = 1, message = "Birthdate is required"))]
    pub birthdate: String,

    #[validate(length(min = 1, message = "Gender is required"))]
    pub gender: String,

    pub is_profile_private: Option<bool>,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters"),
        length(max = 64, message = "Password cannot be more than 64 characters")
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

    #[validate(length(min = 1, message = "Birthdate is required"))]
    pub birthdate: String,

    #[validate(length(min = 1, message = "Gender is required"))]
    pub gender: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters"),
        length(max = 64, message = "Password cannot be more than 64 characters")
    )]
    pub password: String,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateUserDto {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
    pub birthdate: Option<String>,
    pub gender: Option<String>,
    pub biography: Option<String>,
    pub is_profile_private: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateAdminDto {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
    pub birthdate: Option<String>,
    pub gender: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateUserPublicInfoDto {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
    pub gender: Option<String>,
    pub birthdate: Option<String>,
    pub biography: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateUserProfileStatusDto {
    pub is_profile_private: bool,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateAdminPublicInfoDto {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub username: Option<String>,
    pub birthdate: Option<String>,
    pub gender: Option<String>,
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
    pub gender: String,
    pub biography: String,
    pub birthdate: DateTime<Utc>,
    pub emails: Vec<EmailDto>,
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
            birthdate: user.birthdate,
            gender: user.gender.to_string(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
            emails: EmailDto::filter_emails(&user.emails, true),
            biography: if let Some(bio) = &user.biography {
                bio.to_owned()
            } else {
                String::from("")
            },
        }
    }

    pub fn filter_users(users: &[User]) -> Vec<Self> {
        users.iter().map(Self::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminDto {
    pub id: String,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub gender: String,
    pub birthdate: DateTime<Utc>,
    pub emails: Vec<EmailDto>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl AdminDto {
    pub fn filter_admin(admin: &Admin) -> Self {
        Self {
            id: admin.id.to_string(),
            firstname: admin.firstname.to_owned(),
            lastname: admin.lastname.to_owned(),
            username: admin.username.to_owned(),
            birthdate: admin.birthdate,
            gender: admin.gender.to_string(),
            emails: EmailDto::filter_emails(&admin.emails, true),
            created_at: admin.created_at.unwrap(),
            updated_at: admin.updated_at.unwrap(),
        }
    }

    pub fn filter_admins(admins: &[Admin]) -> Vec<Self> {
        admins.iter().map(Self::filter_admin).collect()
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
