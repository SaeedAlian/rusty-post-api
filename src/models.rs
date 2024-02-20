use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Type, PartialEq)]
#[sqlx(type_name = "person_role", rename_all = "lowercase")]
pub enum PersonRole {
    Admin,
    User,
}

impl PersonRole {
    pub fn to_str(&self) -> &str {
        match self {
            PersonRole::Admin => "admin",
            PersonRole::User => "user",
        }
    }
}

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

#[derive(Debug, Deserialize, FromRow, Type, Serialize, Clone)]
pub struct Person {
    pub id: uuid::Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub role: PersonRole,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, FromRow, Type, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, FromRow, Type, Serialize, Clone)]
pub struct Admin {
    pub id: uuid::Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Person> for Admin {
    fn from(person: Person) -> Self {
        Self {
            id: person.id,
            username: person.username,
            firstname: person.firstname,
            lastname: person.lastname,
            password: person.password,
            created_at: person.created_at,
            updated_at: person.updated_at,
        }
    }
}

impl From<Person> for User {
    fn from(person: Person) -> Self {
        Self {
            id: person.id,
            username: person.username,
            firstname: person.firstname,
            lastname: person.lastname,
            password: person.password,
            created_at: person.created_at,
            updated_at: person.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, FromRow, Type, Serialize, Clone)]
pub struct Email {
    pub id: uuid::Uuid,
    pub person_id: uuid::Uuid,
    pub address: String,
}
