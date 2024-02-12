#![cfg(test)]

use sqlx::{Pool, Postgres};

use super::*;
use crate::{
    db::PostExt,
    dtos::{CreatePostDto, SearchPostQueryDto, UpdatePostDto},
    utils::test::init_test_posts,
};

#[sqlx::test]
async fn test_get_post_by_id(pool: Pool<Postgres>) {
    let (post_one, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let post = db_client
        .get_post(post_one)
        .await
        .unwrap_or_else(|err| panic!("Failed to get post by id: {}", err))
        .expect("Post not found");

    assert_eq!(post.id, post_one)
}

#[sqlx::test]
async fn test_get_all_posts(pool: Pool<Postgres>) {
    init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let posts = db_client
        .get_posts(SearchPostQueryDto {
            limit: Some(6),
            page: Some(1),
            title: None,
        })
        .await
        .unwrap();

    assert_eq!(posts.len(), 5)
}

#[sqlx::test]
async fn test_get_posts_with_title_search(pool: Pool<Postgres>) {
    init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let posts = db_client
        .get_posts(SearchPostQueryDto {
            limit: Some(6),
            page: Some(1),
            title: Some("web".to_string()),
        })
        .await
        .unwrap();

    assert_eq!(posts.len(), 2)
}

#[sqlx::test]
async fn test_get_posts_with_title_search_2(pool: Pool<Postgres>) {
    init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let posts = db_client
        .get_posts(SearchPostQueryDto {
            limit: Some(6),
            page: Some(1),
            title: Some("ai".to_string()),
        })
        .await
        .unwrap();

    assert_eq!(posts.len(), 1)
}

#[sqlx::test]
async fn test_save_post(pool: Pool<Postgres>) {
    let db_client = DBClient::new(pool);

    let dto = CreatePostDto {
        title: "New Post".to_string(),
        description: "New Post Description".to_string(),
    };

    let new_post = db_client.save_post(dto.clone()).await.unwrap();

    assert_eq!(new_post.title, dto.title);
    assert_eq!(new_post.description, dto.description);
}

#[sqlx::test]
async fn test_delete_post(pool: Pool<Postgres>) {
    let (post_one, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let is_deleted = db_client.delete_post(post_one).await.unwrap();

    assert_eq!(is_deleted, true)
}

#[sqlx::test]
async fn test_delete_post_nonexistent(pool: Pool<Postgres>) {
    let (post_one, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let _ = db_client.delete_post(post_one).await.unwrap();
    let is_deleted = db_client.delete_post(post_one).await.unwrap();

    assert_eq!(is_deleted, false)
}

#[sqlx::test]
async fn test_update_post_title_only(pool: Pool<Postgres>) {
    let (post_one, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdatePostDto {
        title: Some("New Title".to_string()),
        description: None,
    };

    let is_updated = db_client.update_post(post_one, dto.clone()).await.unwrap();

    assert_eq!(is_updated, true);

    let updated_post = db_client
        .get_post(post_one)
        .await
        .unwrap_or_else(|err| panic!("Failed to get post by id: {}", err))
        .expect("Post not found");

    assert_eq!(updated_post.title, dto.title.unwrap())
}

#[sqlx::test]
async fn test_update_post_description_only(pool: Pool<Postgres>) {
    let (post_one, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdatePostDto {
        title: None,
        description: Some("New Description".to_string()),
    };

    let is_updated = db_client.update_post(post_one, dto.clone()).await.unwrap();

    assert_eq!(is_updated, true);

    let updated_post = db_client
        .get_post(post_one)
        .await
        .unwrap_or_else(|err| panic!("Failed to get post by id: {}", err))
        .expect("Post not found");

    assert_eq!(updated_post.description, dto.description.unwrap())
}

#[sqlx::test]
async fn test_update_post_title_and_desc(pool: Pool<Postgres>) {
    let (post_one, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdatePostDto {
        title: Some("New Title".to_string()),
        description: Some("New Description".to_string()),
    };

    let is_updated = db_client.update_post(post_one, dto.clone()).await.unwrap();

    assert_eq!(is_updated, true);

    let updated_post = db_client
        .get_post(post_one)
        .await
        .unwrap_or_else(|err| panic!("Failed to get post by id: {}", err))
        .expect("Post not found");

    assert_eq!(updated_post.title, dto.title.unwrap());
    assert_eq!(updated_post.description, dto.description.unwrap());
}
