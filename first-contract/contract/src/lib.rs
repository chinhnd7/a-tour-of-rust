/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId, env};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the Blog structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Blog {
    message: String,
    posts: UnorderedMap<usize, Post>,
    owner: AccountId,
}

// Define the default, which automatically initializes the contract
impl Default for Blog{
    fn default() -> Self{
        Self {
            message: DEFAULT_MESSAGE.to_string(),
            posts: UnorderedMap::new(b"posts".to_vec()),
            owner: env::signer_account_id(),
        }        
    }
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Post {
    pub title: String,
    pub body: String,
    pub author: AccountId,
    pub id: usize,
}


// Implement the Blog structure
#[near_bindgen]
impl Blog {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    // create_post
    pub fn create(&mut self, title: String, body: String) -> usize{
        let author = env::signer_account_id();
        let post = Post {
            title,
            body,
            author,
            id: self.posts.len() as usize
        };

        self.posts.insert(&post.id, &post);
        post.id
    }

    // delete_post
    pub fn delete_post(&mut self, id: usize) {
        // check if current user is not owner
        let user = env:: signer_account_id();
        assert_eq!(self.owner, user, "Only owner can delete post");

        self.posts.remove(&id);
    }

    // edit_post
    pub fn edit_post(&mut self, id: usize, title: String, body: String) {
        // check if current user is not owner
        let user = env:: signer_account_id();
        assert_eq!(self.owner, user, "Only owner can edit post");

        let mut post = self.posts.get(&id).unwrap();
        assert_eq!(post.author, user, "Only author can edit post");

        post.title = title;
        post.body = body;
    }

    // get_post
    pub fn get_post(&self, id: usize) -> Post {
        self.posts.get(&id).unwrap().clone()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let blog = Blog::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            blog.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut blog = Blog::default();
        blog.set_greeting("howdy".to_string());
        assert_eq!(
            blog.get_greeting(),
            "howdy".to_string()
        );
    }

    #[test]
    fn test_create_post() {
        let mut blog = Blog::default();
        let post_id = blog.create("title".to_string(), "blog 0".to_string());
        assert_eq!( post_id, 0);
    }
}
