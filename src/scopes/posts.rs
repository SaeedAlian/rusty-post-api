use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    db::PostExt,
    dtos::{CreatePostDto, FilterPostDto, PostData, PostListResponseDto, PostResponseDto},
    error::HttpError,
    AppState,
};

pub fn posts_scope() -> Scope {
    web::scope("/api/posts")
        .route("", web::get().to(get_posts))
        .route("", web::post().to(save_post))
}

pub async fn get_posts(app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {
    let posts = app_state
        .db_client
        .get_posts()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(PostListResponseDto {
        status: "success".to_string(),
        posts: FilterPostDto::filter_posts(&posts),
        results: posts.len(),
    }))
}

pub async fn save_post(
    app_state: web::Data<AppState>,
    body: web::Json<CreatePostDto>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .save_post(&body.title, &body.description)
        .await;

    match result {
        Ok(post) => Ok(HttpResponse::Created().json(PostResponseDto {
            status: "success".to_string(),
            data: PostData {
                post: FilterPostDto::filter_post(&post),
            },
        })),
        Err(sqlx::Error::Database(db_err)) => Err(HttpError::server_error(db_err.to_string())),
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}
