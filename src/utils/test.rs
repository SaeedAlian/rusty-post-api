use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    db::{DBClient, PostExt},
    dtos::CreatePostDto,
};

#[allow(dead_code)]
pub struct TestPost {
    title: &'static str,
    description: &'static str,
}

#[allow(dead_code)]
pub async fn init_test_posts(pool: &Pool<Postgres>) -> (Uuid, Uuid, Uuid, Uuid) {
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

    let mut post_ids = vec![];

    for post_data in posts {
        let post = db_client
            .save_post(CreatePostDto {
                title: post_data.title.to_string(),
                description: post_data.description.to_string(),
            })
            .await
            .unwrap();
        post_ids.push(post.id);
    }

    (
        post_ids[0].clone(),
        post_ids[1].clone(),
        post_ids[2].clone(),
        post_ids[3].clone(),
    )
}
