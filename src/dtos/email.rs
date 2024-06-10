use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::Email;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateEmailDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub address: String,

    #[serde(rename = "isPrivate")]
    pub is_private: Option<bool>,

    #[serde(rename = "isPrimary")]
    pub is_primary: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateEmailDto {
    pub address: Option<String>,
    pub is_private: Option<bool>,
    pub is_primary: Option<bool>,
    pub is_verified: Option<bool>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateEmailAddressDto {
    #[validate(email(message = "Email is invalid"))]
    pub address: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateEmailPrivacyDto {
    #[serde(rename = "isPrivate")]
    pub is_private: Option<bool>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateEmailPrimaryStatusDto {
    #[serde(rename = "isPrimary")]
    pub is_primary: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailDto {
    pub id: String,
    pub address: String,

    #[serde(rename = "isPrimary")]
    pub is_primary: bool,

    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,
}

impl EmailDto {
    pub fn filter_email(email: &Email, remove_owner_id: bool) -> Self {
        Self {
            id: email.id.to_string(),
            address: email.address.to_owned(),
            is_primary: email.is_primary,
            owner_id: if remove_owner_id {
                None
            } else {
                Some(email.owner_id.to_string())
            },
        }
    }

    pub fn filter_emails(emails: &[Email], remove_owner_id: bool) -> Vec<Self> {
        emails
            .iter()
            .map(|e| Self::filter_email(e, remove_owner_id))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalEmailDto {
    pub id: String,
    pub address: String,

    #[serde(rename = "isPrimary")]
    pub is_primary: bool,

    #[serde(rename = "isPrivate")]
    pub is_private: bool,

    #[serde(rename = "isVerified")]
    pub is_verified: bool,
}

impl PersonalEmailDto {
    pub fn filter_email(email: &Email) -> Self {
        Self {
            id: email.id.to_string(),
            address: email.address.to_owned(),
            is_primary: email.is_primary,
            is_private: email.is_private,
            is_verified: email.is_verified,
        }
    }

    pub fn filter_emails(emails: &[Email]) -> Vec<Self> {
        emails.iter().map(|e| Self::filter_email(e)).collect()
    }
}

#[derive(Deserialize)]
pub struct GetEmailByIdParamsDto {
    pub id: String,
}

#[derive(Deserialize)]
pub struct GetEmailsByOwnerIdParamsDto {
    pub owner_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailResponseDto {
    pub status: u16,
    pub email: EmailDto,

    #[serde(rename = "ownerId")]
    pub owner_id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailListResponseDto {
    pub status: u16,
    pub emails: Vec<EmailDto>,
    pub results: usize,

    #[serde(rename = "ownerId")]
    pub owner_id: uuid::Uuid,
}
