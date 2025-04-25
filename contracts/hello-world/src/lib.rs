#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, log, symbol_short, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Post {
    pub post_id: u64,
    pub content: String,
    pub sentiment_score: i32, // range: -100 to +100
    pub keywords: Vec<String>,
}

#[contracttype]
pub enum Postbook {
    Post(u64),
}

const COUNT_POST: Symbol = symbol_short!("C_POST");

#[contract]
pub struct SocialMediaAnalyzer;

#[contractimpl]
impl SocialMediaAnalyzer {
    // Create a new social media post
    pub fn add_post(env: Env, content: String, sentiment_score: i32, keywords: Vec<String>) -> u64 {
        let mut count = env.storage().instance().get(&COUNT_POST).unwrap_or(0u64);
        count += 1;

        let new_post = Post {
            post_id: count,
            content,
            sentiment_score,
            keywords,
        };

        env.storage()
            .instance()
            .set(&Postbook::Post(count), &new_post);
        env.storage().instance().set(&COUNT_POST, &count);
        log!(&env, "New post added with Post-ID: {}", count);

        count
    }

    // View a post by post ID
    pub fn view_post(env: Env, post_id: u64) -> Post {
        env.storage()
            .instance()
            .get(&Postbook::Post(post_id))
            .unwrap_or(Post {
                post_id: 0,
                content: String::from_str(&env, "Not Found"),
                sentiment_score: 0,
                keywords: Vec::new(&env),
            })
    }

    // Get the total number of posts
    pub fn total_posts(env: Env) -> u64 {
        env.storage().instance().get(&COUNT_POST).unwrap_or(0u64)
    }
}
