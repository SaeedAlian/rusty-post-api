use actix_web::{web, HttpResponse as ActixHttpResponse, Scope};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::PostExt,
    dtos::{
        CreatePostDto, GetPostParamsDto, PostDto, PostListResponseDto, PostResponseDto,
        SearchPostQueryDto, UpdatePostDto,
    },
    response::{DefaultHttpError, DefaultHttpResponse, HttpResponse},
    AppState,
};

pub fn posts_scope() -> Scope {
    web::scope("/api/posts")
        // GET methods
        .route("", web::get().to(get_posts))
        .route("{post_id}", web::get().to(get_post))
        // POST methods
        .route("", web::post().to(save_post))
        // PATCH methods
        .route("{post_id}", web::patch().to(update_post))
        // DELETE methods
        .route("{post_id}", web::delete().to(delete_post))
}

pub async fn get_posts(
    query: web::Query<SearchPostQueryDto>,
    app_state: web::Data<AppState>,
) -> Result<ActixHttpResponse, DefaultHttpError> {
    let query_params: SearchPostQueryDto = query.into_inner();

    query_params
        .validate()
        .map_err(|e| DefaultHttpError::bad_request(e.to_string()))?;

    let posts = app_state
        .db_client
        .get_posts(query_params)
        .await
        .map_err(|e| DefaultHttpError::server_error(e.to_string()))?;

    Ok(ActixHttpResponse::Ok().json(PostListResponseDto {
        status: 200,
        posts: PostDto::filter_posts(&posts),
        results: posts.len(),
    }))
}

pub async fn get_post(
    app_state: web::Data<AppState>,
    path: web::Path<GetPostParamsDto>,
) -> Result<ActixHttpResponse, DefaultHttpError> {
    let post_id = Uuid::parse_str(&path.post_id);

    if let Ok(id) = post_id {
        let result = app_state.db_client.get_post(id).await;

        return match result {
            Ok(post) => {
                if let Some(post) = post {
                    return Ok(ActixHttpResponse::Ok().json(PostResponseDto {
                        status: 200,
                        post: PostDto::filter_post(&post),
                    }));
                }

                Err(DefaultHttpError::not_found("Post not found".to_string()))
            }

            Err(sqlx::Error::Database(db_err)) => {
                Err(DefaultHttpError::server_error(db_err.to_string()))
            }
            Err(e) => Err(DefaultHttpError::server_error(e.to_string())),
        };
    }

    Err(DefaultHttpError::server_error(
        "The provided id is not valid".to_string(),
    ))
}

pub async fn save_post(
    app_state: web::Data<AppState>,
    body: web::Json<CreatePostDto>,
) -> Result<ActixHttpResponse, DefaultHttpError> {
    body.validate()
        .map_err(|e| DefaultHttpError::bad_request(e.to_string()))?;

    let result = app_state.db_client.save_post(body.into_inner()).await;

    match result {
        Ok(post) => Ok(ActixHttpResponse::Created().json(PostResponseDto {
            status: 200,
            post: PostDto::filter_post(&post),
        })),
        Err(sqlx::Error::Database(db_err)) => {
            Err(DefaultHttpError::server_error(db_err.to_string()))
        }
        Err(e) => Err(DefaultHttpError::server_error(e.to_string())),
    }
}

pub async fn update_post(
    app_state: web::Data<AppState>,
    path: web::Path<GetPostParamsDto>,
    body: web::Json<UpdatePostDto>,
) -> Result<ActixHttpResponse, DefaultHttpError> {
    let post_id = Uuid::parse_str(&path.post_id);

    body.validate()
        .map_err(|e| DefaultHttpError::bad_request(e.to_string()))?;

    if let Ok(id) = post_id {
        let result = app_state.db_client.update_post(id, body.into_inner()).await;

        return match result {
            Ok(is_updated) => {
                if is_updated {
                    return Ok(
                        DefaultHttpResponse::ok("Post has been updated").into_http_response()
                    );
                }

                Err(DefaultHttpError::not_found("Post not found".to_string()))
            }
            Err(sqlx::Error::Database(db_err)) => {
                Err(DefaultHttpError::server_error(db_err.to_string()))
            }
            Err(e) => Err(DefaultHttpError::server_error(e.to_string())),
        };
    }

    Err(DefaultHttpError::server_error(
        "The provided id is not valid".to_string(),
    ))
}

pub async fn delete_post(
    app_state: web::Data<AppState>,
    path: web::Path<GetPostParamsDto>,
) -> Result<ActixHttpResponse, DefaultHttpError> {
    let post_id = Uuid::parse_str(&path.post_id);

    if let Ok(id) = post_id {
        let result = app_state.db_client.delete_post(id).await;

        return match result {
            Ok(is_deleted) => {
                if is_deleted {
                    return Ok(
                        DefaultHttpResponse::ok("Post has been deleted").into_http_response()
                    );
                }

                Err(DefaultHttpError::not_found("Post not found".to_string()))
            }
            Err(sqlx::Error::Database(db_err)) => {
                Err(DefaultHttpError::server_error(db_err.to_string()))
            }
            Err(e) => Err(DefaultHttpError::server_error(e.to_string())),
        };
    }

    Err(DefaultHttpError::server_error(
        "The provided id is not valid".to_string(),
    ))
}
