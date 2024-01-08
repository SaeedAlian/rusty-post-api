use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::Post;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreatePostDto {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,

    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
}

#[derive(Deserialize)]
pub struct PostPathParamDto {
    pub post_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterPostDto {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterPostDto {
    pub fn filter_post(post: &Post) -> Self {
        FilterPostDto {
            id: post.id.to_string(),
            title: post.title.to_owned(),
            description: post.description.to_owned(),
            created_at: post.created_at.unwrap(),
            updated_at: post.updated_at.unwrap(),
        }
    }

    pub fn filter_posts(posts: &[Post]) -> Vec<FilterPostDto> {
        posts.iter().map(FilterPostDto::filter_post).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostData {
    pub post: FilterPostDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponseDto {
    pub status: String,
    pub data: PostData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostListResponseDto {
    pub status: String,
    pub posts: Vec<FilterPostDto>,
    pub results: usize,
}
