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

    pub fn to_string(&self) -> String {
        match self {
            PersonRole::Admin => "Admin".to_string(),
            PersonRole::User => "User".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Type, PartialEq)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn to_str(&self) -> &str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Gender::Male => "Male".to_string(),
            Gender::Female => "Female".to_string(),
        }
    }
}

impl From<String> for Gender {
    fn from(string: String) -> Gender {
        match string.to_lowercase().as_str() {
            "male" => Gender::Male,
            "female" => Gender::Female,
            _ => {
                eprintln!("Gender is not correct");
                Gender::Male
            }
        }
    }
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct Person {
    pub id: uuid::Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub role: PersonRole,
    pub birthdate: DateTime<Utc>,
    pub gender: Gender,
    pub biography: Option<String>,
    pub is_profile_private: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    #[sqlx(skip)]
    pub emails: Vec<Email>,
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub birthdate: DateTime<Utc>,
    pub gender: Gender,
    pub biography: Option<String>,
    pub is_profile_private: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    #[sqlx(skip)]
    pub emails: Vec<Email>,
}

impl From<Person> for User {
    fn from(person: Person) -> Self {
        Self {
            id: person.id,
            username: person.username,
            firstname: person.firstname,
            lastname: person.lastname,
            password: person.password,
            birthdate: person.birthdate,
            gender: person.gender,
            biography: person.biography,
            is_profile_private: person.is_profile_private,
            emails: person.emails,
            created_at: person.created_at,
            updated_at: person.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct Admin {
    pub id: uuid::Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub birthdate: DateTime<Utc>,
    pub gender: Gender,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    #[sqlx(skip)]
    pub emails: Vec<Email>,
}

impl From<Person> for Admin {
    fn from(person: Person) -> Self {
        Self {
            id: person.id,
            username: person.username,
            firstname: person.firstname,
            lastname: person.lastname,
            password: person.password,
            birthdate: person.birthdate,
            gender: person.gender,
            emails: person.emails,
            created_at: person.created_at,
            updated_at: person.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct Email {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub address: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub is_private: bool,
    pub updated_at: Option<DateTime<Utc>>,
}
