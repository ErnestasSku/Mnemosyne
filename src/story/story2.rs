use std::sync::Arc;

use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

use super::story_parser::StoryParse;

#[derive(Debug, Clone)]
pub struct StoryBlock2 {
    pub id: String,
    pub text: String,
    
    pub path: Vec<(Arc<StoryBlock2>, String, String)>,
}

impl StoryBlock2 {
    pub fn new(text: &str) -> StoryBlock2 {
        StoryBlock2 { 
            text: text.to_string() , 
            id: String::new(),
            path: Vec::new(),
        }
    }

    pub fn from_parse(parse: &StoryParse) -> StoryBlock2 {
        StoryBlock2 { 
            id: String::from(&parse.id), 
            text: String::from(&parse.content), 
            path: Vec::new(),
        }
    }

    pub fn set_paths(mut self, paths: Vec<(Arc<StoryBlock2>, String, String)>) -> Self {
        self.path = paths;
        self
    }
}

pub struct StoryContainer2;

impl TypeMapKey for StoryContainer2 {
    type Value = Arc<RwLock<std::collections::HashMap<String, StoryBlock2>>>;
}

