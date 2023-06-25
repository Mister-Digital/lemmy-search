use std::collections::HashSet;

use crate::api::lemmy::models::{
    post::Post, 
    comment::Comment
};

pub struct Analyizer;

impl Analyizer {

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_distinct_words_in_post(
        &self,
        post : &Post
    ) -> HashSet<String> {
        let mut words = HashSet::<String>::new();
        let name_trimed = post.name.replace(|c : char| {
            !c.is_ascii_alphanumeric() && !c.is_whitespace()
        }, " ").to_lowercase();
        for word in name_trimed.split_whitespace() {
            words.insert(word.to_lowercase().trim().to_string());
        }
        match &post.body {
            Some(body) => {
                let body_trimed = body.replace(|c : char| {
                    !c.is_ascii_alphanumeric() && !c.is_whitespace()
                }, " ").to_lowercase();
                for word in body_trimed.split_whitespace() {
                    words.insert(word.to_lowercase().trim().to_string());
                }
            },
            None => {}
        }
        words
    }

    pub fn get_distinct_words_in_comment(
        &self,
        comment : &Comment
    ) -> HashSet<String> {
        HashSet::from_iter(comment.content.replace(|c : char| {
            !c.is_ascii_alphanumeric() && !c.is_whitespace()
        }, " ").to_lowercase().split_whitespace().map(|word|
            word.to_lowercase().trim().to_string()
        ))
    }
}
