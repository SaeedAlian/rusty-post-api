#![cfg(test)]

use sqlx::{Pool, Postgres};

use super::*;
use crate::{
    db::person::PersonExt,
    db::post::PostExt,
    dtos::person::{CreateUserDto, UpdateUserDto},
    dtos::{
        person::SearchUserQueryDto,
        post::{CreatePostDto, SearchPostQueryDto, UpdatePostDto},
    },
    utils::test::{init_test_posts, init_test_users},
};

#[sqlx::test]
async fn test_get_post_by_id(pool: Pool<Postgres>) {
    let (post_one, _, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let post = db_client
        .get_post(post_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get post by id: {}", err))
        .expect("Post not found");

    assert_eq!(post.id, post_one.id)
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
    let (post_one, _, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let is_deleted = db_client.delete_post(post_one.id).await.unwrap();

    assert_eq!(is_deleted, true)
}

#[sqlx::test]
async fn test_delete_post_nonexistent(pool: Pool<Postgres>) {
    let (post_one, _, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let _ = db_client.delete_post(post_one.id).await.unwrap();
    let is_deleted = db_client.delete_post(post_one.id).await.unwrap();

    assert_eq!(is_deleted, false)
}

#[sqlx::test]
async fn test_update_post_title_only(pool: Pool<Postgres>) {
    let (post_one, _, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdatePostDto {
        title: Some("New Title".to_string()),
        description: None,
    };

    let is_updated = db_client
        .update_post(post_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_post = db_client
        .get_post(post_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get post by id: {}", err))
        .expect("Post not found");

    assert_eq!(updated_post.title, dto.title.unwrap());
    assert_eq!(updated_post.description, post_one.description);
}

#[sqlx::test]
async fn test_update_post_description_only(pool: Pool<Postgres>) {
    let (post_one, _, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdatePostDto {
        title: None,
        description: Some("New Description".to_string()),
    };

    let is_updated = db_client
        .update_post(post_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_post = db_client
        .get_post(post_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get post by id: {}", err))
        .expect("Post not found");

    assert_eq!(updated_post.description, dto.description.unwrap());
    assert_eq!(updated_post.title, post_one.title);
}

#[sqlx::test]
async fn test_update_post_title_and_desc(pool: Pool<Postgres>) {
    let (post_one, _, _, _, _) = init_test_posts(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdatePostDto {
        title: Some("New Title".to_string()),
        description: Some("New Description".to_string()),
    };

    let is_updated = db_client
        .update_post(post_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_post = db_client
        .get_post(post_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get post by id: {}", err))
        .expect("Post not found");

    assert_eq!(updated_post.title, dto.title.unwrap());
    assert_eq!(updated_post.description, dto.description.unwrap());
}

#[sqlx::test]
async fn test_get_user_by_id(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let user = db_client
        .get_user(user_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get user by id: {}", err))
        .expect("User not found");

    assert_eq!(user.id, user_one.id)
}

#[sqlx::test]
async fn test_get_all_users_without_emails(pool: Pool<Postgres>) {
    init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let users = db_client
        .get_users(
            SearchUserQueryDto {
                limit: Some(2),
                page: Some(1),
                username: None,
                firstname: None,
                lastname: None,
            },
            false,
        )
        .await
        .unwrap();

    assert_eq!(users.len(), 2)
}

#[sqlx::test]
async fn test_get_all_users_with_emails(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let users = db_client
        .get_users(
            SearchUserQueryDto {
                limit: Some(2),
                page: Some(1),
                username: None,
                firstname: None,
                lastname: None,
            },
            true,
        )
        .await
        .unwrap();

    assert_eq!(user_one.emails.len(), users[0].emails.len());
    assert_eq!(users.len(), 2);
}

#[sqlx::test]
async fn test_get_users_with_firstname_only(pool: Pool<Postgres>) {
    init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let users = db_client
        .get_users(
            SearchUserQueryDto {
                limit: Some(4),
                page: Some(1),
                firstname: Some("Alic".to_string()),
                lastname: None,
                username: None,
            },
            false,
        )
        .await
        .unwrap();

    assert_eq!(users.len(), 1)
}

#[sqlx::test]
async fn test_get_users_with_lastname_only(pool: Pool<Postgres>) {
    init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let users = db_client
        .get_users(
            SearchUserQueryDto {
                limit: Some(4),
                page: Some(1),
                lastname: Some("Doe".to_string()),
                firstname: None,
                username: None,
            },
            false,
        )
        .await
        .unwrap();

    assert_eq!(users.len(), 1)
}

#[sqlx::test]
async fn test_get_users_with_username_only(pool: Pool<Postgres>) {
    init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let users = db_client
        .get_users(
            SearchUserQueryDto {
                limit: Some(4),
                page: Some(1),
                username: Some("123".to_string()),
                firstname: None,
                lastname: None,
            },
            false,
        )
        .await
        .unwrap();

    assert_eq!(users.len(), 2)
}

#[sqlx::test]
async fn test_get_users_with_full_search(pool: Pool<Postgres>) {
    init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let users = db_client
        .get_users(
            SearchUserQueryDto {
                limit: Some(4),
                page: Some(1),
                username: Some("john".to_string()),
                firstname: Some("Jo".to_string()),
                lastname: Some("D".to_string()),
            },
            false,
        )
        .await
        .unwrap();

    assert_eq!(users.len(), 1)
}

#[sqlx::test]
async fn test_save_user(pool: Pool<Postgres>) {
    init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = CreateUserDto {
        username: "new_user123".to_string(),
        firstname: "New".to_string(),
        lastname: "User".to_string(),
        email: "new_user@example.com".to_string(),
        password: "abc12345".to_string(),
        birthdate: "1999-10-10".to_string(),
        gender: "male".to_string(),
        is_profile_private: Some(false),
    };

    let new_user = db_client.save_user(dto.clone()).await.unwrap();

    assert_eq!(new_user.username, dto.username);
    assert_eq!(new_user.firstname, dto.firstname);
    assert_eq!(new_user.lastname, dto.lastname);
    assert_eq!(new_user.gender.to_string(), dto.gender);
    assert_eq!(new_user.birthdate.to_string(), dto.birthdate);
    assert_eq!(new_user.is_profile_private, dto.is_profile_private.unwrap());
}

#[sqlx::test]
async fn test_save_user_with_existent_username(pool: Pool<Postgres>) {
    init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = CreateUserDto {
        username: "alice_smith".to_string(),
        firstname: "New".to_string(),
        lastname: "User".to_string(),
        email: "new_user@example.com".to_string(),
        password: "abc12345".to_string(),
        birthdate: "1999-10-10".to_string(),
        gender: "male".to_string(),
        is_profile_private: Some(false),
    };

    let res = db_client.save_user(dto.clone()).await;

    match res {
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            assert!(true);
        }
        _ => {
            assert!(false, "Expected unique constraint violation error");
        }
    }
}

#[sqlx::test]
async fn test_save_user_with_existent_email(pool: Pool<Postgres>) {
    init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = CreateUserDto {
        username: "alice_smith2".to_string(),
        firstname: "New".to_string(),
        lastname: "User".to_string(),
        email: "alice@example.com".to_string(),
        password: "abc12345".to_string(),
        birthdate: "1999-10-10".to_string(),
        gender: "male".to_string(),
        is_profile_private: Some(false),
    };

    let res = db_client.save_user(dto.clone()).await;

    match res {
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            assert!(true);
        }
        _ => {
            assert!(false, "Expected unique constraint violation error");
        }
    }
}

#[sqlx::test]
async fn test_save_user_with_wrong_gender(pool: Pool<Postgres>) {
    init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = CreateUserDto {
        username: "new_username".to_string(),
        firstname: "New".to_string(),
        lastname: "User".to_string(),
        email: "new_user@example.com".to_string(),
        password: "abc12345".to_string(),
        birthdate: "1999-10-10".to_string(),
        gender: "maleee".to_string(),
        is_profile_private: Some(false),
    };

    let res = db_client.save_user(dto.clone()).await;

    match res {
        Err(sqlx::Error::Database(db_err)) if db_err.is_check_violation() => {
            assert!(true);
        }
        _ => {
            assert!(false, "Expected an error for gender enum");
        }
    }
}

#[sqlx::test]
async fn test_delete_user(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let is_deleted = db_client.delete_user(user_one.id).await.unwrap();

    assert_eq!(is_deleted, true)
}

#[sqlx::test]
async fn test_delete_user_nonexistent(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let _ = db_client.delete_user(user_one.id).await.unwrap();
    let is_deleted = db_client.delete_user(user_one.id).await.unwrap();

    assert_eq!(is_deleted, false)
}

#[sqlx::test]
async fn test_update_user_username_only(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdateUserDto {
        username: Some("new_username".to_string()),
        firstname: None,
        lastname: None,
        gender: None,
        biography: None,
        birthdate: None,
        is_profile_private: None,
    };

    let is_updated = db_client
        .update_user(user_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_user = db_client
        .get_user(user_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get user by id: {}", err))
        .expect("User not found");

    assert_eq!(updated_user.username, dto.username.unwrap());
    assert_eq!(updated_user.firstname, user_one.firstname);
    assert_eq!(updated_user.lastname, user_one.lastname);
}

#[sqlx::test]
async fn test_update_user_firstname_only(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdateUserDto {
        firstname: Some("New Firstname".to_string()),
        username: None,
        lastname: None,
        gender: None,
        biography: None,
        birthdate: None,
        is_profile_private: None,
    };

    let is_updated = db_client
        .update_user(user_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_user = db_client
        .get_user(user_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get user by id: {}", err))
        .expect("User not found");

    assert_eq!(updated_user.firstname, dto.firstname.unwrap());
    assert_eq!(updated_user.username, user_one.username);
    assert_eq!(updated_user.lastname, user_one.lastname);
}

#[sqlx::test]
async fn test_update_user_lastname_only(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdateUserDto {
        lastname: Some("New Lastname".to_string()),
        username: None,
        firstname: None,
        gender: None,
        biography: None,
        birthdate: None,
        is_profile_private: None,
    };

    let is_updated = db_client
        .update_user(user_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_user = db_client
        .get_user(user_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get user by id: {}", err))
        .expect("User not found");

    assert_eq!(updated_user.lastname, dto.lastname.unwrap());
    assert_eq!(updated_user.username, user_one.username);
    assert_eq!(updated_user.firstname, user_one.firstname);
}

#[sqlx::test]
async fn test_update_user_username_and_firstname(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdateUserDto {
        firstname: Some("New Firstname".to_string()),
        username: Some("new_username".to_string()),
        lastname: None,
        gender: None,
        biography: None,
        birthdate: None,
        is_profile_private: None,
    };

    let is_updated = db_client
        .update_user(user_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_user = db_client
        .get_user(user_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get user by id: {}", err))
        .expect("User not found");

    assert_eq!(updated_user.username, dto.username.unwrap());
    assert_eq!(updated_user.firstname, dto.firstname.unwrap());
    assert_eq!(updated_user.lastname, user_one.lastname);
}

#[sqlx::test]
async fn test_update_user_username_and_lastname(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdateUserDto {
        lastname: Some("New Lastname".to_string()),
        username: Some("new_username".to_string()),
        firstname: None,
        gender: None,
        biography: None,
        birthdate: None,
        is_profile_private: None,
    };

    let is_updated = db_client
        .update_user(user_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_user = db_client
        .get_user(user_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get user by id: {}", err))
        .expect("User not found");

    assert_eq!(updated_user.username, dto.username.unwrap());
    assert_eq!(updated_user.lastname, dto.lastname.unwrap());
    assert_eq!(updated_user.firstname, user_one.firstname);
}

#[sqlx::test]
async fn test_update_user_firstname_and_lastname(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdateUserDto {
        lastname: Some("New Lastname".to_string()),
        firstname: Some("New Firstname".to_string()),
        username: None,
        gender: None,
        biography: None,
        birthdate: None,
        is_profile_private: None,
    };

    let is_updated = db_client
        .update_user(user_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_user = db_client
        .get_user(user_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get user by id: {}", err))
        .expect("User not found");

    assert_eq!(updated_user.firstname, dto.firstname.unwrap());
    assert_eq!(updated_user.lastname, dto.lastname.unwrap());
    assert_eq!(updated_user.username, user_one.username);
}

#[sqlx::test]
async fn test_update_user_firstname_and_lastname_and_username(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdateUserDto {
        lastname: Some("New Lastname".to_string()),
        firstname: Some("New Firstname".to_string()),
        username: Some("new_username".to_string()),
        gender: None,
        biography: None,
        birthdate: None,
        is_profile_private: None,
    };

    let is_updated = db_client
        .update_user(user_one.id, dto.clone())
        .await
        .unwrap();

    assert_eq!(is_updated, true);

    let updated_user = db_client
        .get_user(user_one.id)
        .await
        .unwrap_or_else(|err| panic!("Failed to get user by id: {}", err))
        .expect("User not found");

    assert_eq!(updated_user.firstname, dto.firstname.unwrap());
    assert_eq!(updated_user.lastname, dto.lastname.unwrap());
    assert_eq!(updated_user.username, dto.username.unwrap());
}

#[sqlx::test]
async fn test_update_user_with_existent_username(pool: Pool<Postgres>) {
    let (user_one, _, _, _) = init_test_users(&pool).await;
    let db_client = DBClient::new(pool);

    let dto = UpdateUserDto {
        username: Some("john_doe123".to_string()),
        firstname: None,
        lastname: None,
        gender: None,
        biography: None,
        birthdate: None,
        is_profile_private: None,
    };

    let res = db_client.update_user(user_one.id, dto.clone()).await;

    match res {
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            assert!(true);
        }
        _ => {
            assert!(false, "Expected unique constraint violation error");
        }
    }
}
