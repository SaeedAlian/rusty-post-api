use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Deserialize, FromRow, Type, Serialize, Clone)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
