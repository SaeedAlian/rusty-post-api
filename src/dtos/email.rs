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
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateEmailDto {
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailDto {
    pub id: String,
    pub address: String,
    pub person_id: Option<String>,
}

impl EmailDto {
    pub fn filter_email(email: &Email, remove_person_id: Option<bool>) -> Self {
        let can_remove_person_id = remove_person_id.unwrap_or(false);

        Self {
            id: email.id.to_string(),
            address: email.address.to_owned(),
            person_id: if can_remove_person_id {
                None
            } else {
                Some(email.person_id.to_string())
            },
        }
    }

    pub fn filter_emails(emails: &[Email], remove_person_id: Option<bool>) -> Vec<Self> {
        emails
            .iter()
            .map(|e| Self::filter_email(e, remove_person_id))
            .collect()
    }
}

#[derive(Deserialize)]
pub struct GetEmailByIdParamsDto {
    pub id: String,
}

#[derive(Deserialize)]
pub struct GetEmailsByPersonIdParamsDto {
    pub person_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailResponseDto {
    pub status: u16,
    pub email: EmailDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailListResponseDto {
    pub status: u16,
    pub emails: Vec<EmailDto>,
    pub results: usize,
    pub person_id: uuid::Uuid,
}
