use sqlx::{Pool, Postgres};

use crate::{
    db::person::PersonExt,
    db::post::PostExt,
    db::DBClient,
    dtos::person::CreateUserDto,
    dtos::post::CreatePostDto,
    models::{Post, User},
};

#[allow(dead_code)]
pub struct TestPost {
    title: &'static str,
    description: &'static str,
}

#[allow(dead_code)]
pub struct TestUser {
    pub username: &'static str,
    pub firstname: &'static str,
    pub lastname: &'static str,
    pub password: &'static str,
    pub email: &'static str,
}

#[allow(dead_code)]
pub async fn init_test_posts(pool: &Pool<Postgres>) -> (Post, Post, Post, Post, Post) {
    let db_client = DBClient::new(pool.clone());

    let posts: Vec<TestPost> = vec![
            TestPost {
                title: "Mastering Data Structures and Algorithms: A Comprehensive Guide for Programmers",
                description: "Dive into the world of data structures and algorithms with this comprehensive guide aimed at programmers looking to enhance their problem-solving skills. Explore essential data structures like arrays, linked lists, trees, and advanced algorithms such as sorting, searching, and graph algorithms. Mastering these concepts is crucial for writing efficient and optimized code in various programming languages.",
            },
            TestPost {
                title: "Effective Debugging Techniques: Strategies to Improve Code Quality and Productivity" ,
                description: "Enhance your programming skills with a deep dive into effective debugging techniques to streamline your development process. This post covers essential strategies for identifying and fixing bugs efficiently, optimizing code performance, and improving overall code quality. Learn valuable tips and tools that will boost your productivity and make you a more proficient programmer.",
            },
            TestPost {
                title : "Demystifying Machine Learning: A Beginner's Journey into AI Programming",
                description: "Embark on a beginner-friendly journey into the exciting world of machine learning and artificial intelligence programming. Unravel the mysteries behind key machine learning concepts, such as supervised and unsupervised learning, neural networks, and deep learning. Gain insights into practical applications of machine learning algorithms and how they are revolutionizing various industries.",
            },
            TestPost {
                title: "Building Scalable Web Applications with Microservices Architecture",
                description : "Learn how to design and implement scalable web applications using microservices architecture. This post explores the advantages of microservices, guiding developers through the process of breaking down monolithic applications into smaller, independent services. Discover best practices for building resilient, highly scalable systems that can adapt to growing user demands and evolving business requirements.",
            },
            TestPost {
                title: "Web Accessibility: Creating Inclusive User Experiences for All",
                description : "Delve into the crucial topic of web accessibility and learn how to design and develop websites that are inclusive and usable by all individuals. This post explores the importance of accessibility in web design, addressing the needs of users with disabilities and diverse abilities. Discover techniques and best practices for creating accessible web content, including proper HTML semantics, keyboard navigation, color contrast, and assistive technologies compatibility. Empower yourself to make the web a more inclusive and welcoming space for everyone, ensuring that all users can access and interact with digital content seamlessly.",
            },
        ];

    let mut created_posts: Vec<Post> = vec![];

    for post_data in posts {
        let post = db_client
            .save_post(CreatePostDto {
                title: post_data.title.to_string(),
                description: post_data.description.to_string(),
            })
            .await
            .unwrap();

        created_posts.push(post);
    }

    (
        created_posts[0].clone(),
        created_posts[1].clone(),
        created_posts[2].clone(),
        created_posts[3].clone(),
        created_posts[4].clone(),
    )
}

#[allow(dead_code)]
pub async fn init_test_users(pool: &Pool<Postgres>) -> (User, User, User, User) {
    let db_client = DBClient::new(pool.clone());

    let users: Vec<TestUser> = vec![
        TestUser {
            firstname: "Alice",
            lastname: "Smith",
            username: "alice_smith",
            password: "password123",
            email: "alice@example.com",
        },
        TestUser {
            firstname: "John",
            lastname: "Doe",
            username: "john_doe123",
            password: "doe1234",
            email: "john.doe@example.com",
        },
        TestUser {
            firstname: "Sarah",
            lastname: "Johnson",
            username: "sarah_j",
            password: "sarahpw",
            email: "sarah.j@example.com",
        },
        TestUser {
            firstname: "Michael",
            lastname: "Brown",
            username: "mbrown123",
            password: "brownie456",
            email: "michael.b@example.com",
        },
    ];

    let mut created_users: Vec<User> = vec![];

    for user_data in users {
        let user = db_client
            .save_user(CreateUserDto {
                firstname: user_data.firstname.to_string(),
                lastname: user_data.lastname.to_string(),
                username: user_data.username.to_string(),
                password: user_data.password.to_string(),
                email: user_data.email.to_string(),
            })
            .await
            .unwrap();

        created_users.push(user);
    }

    (
        created_users[0].clone(),
        created_users[1].clone(),
        created_users[2].clone(),
        created_users[3].clone(),
    )
}
