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

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchPostQueryDto {
    pub title: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDto {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl PostDto {
    pub fn filter_post(post: &Post) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title.to_owned(),
            description: post.description.to_owned(),
            created_at: post.created_at.unwrap(),
            updated_at: post.updated_at.unwrap(),
        }
    }

    pub fn filter_posts(posts: &[Post]) -> Vec<Self> {
        posts.iter().map(Self::filter_post).collect()
    }
}

#[derive(Deserialize)]
pub struct GetPostParamsDto {
    pub post_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponseDto {
    pub status: u16,
    pub post: PostDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostListResponseDto {
    pub status: u16,
    pub posts: Vec<PostDto>,
    pub results: usize,
}
