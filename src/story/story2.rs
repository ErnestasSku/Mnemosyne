use std::sync::{Arc, Mutex};

use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

use super::story_parser::StoryParse;

#[derive(Debug)]
pub struct StoryBlock2 {
    pub id: String,
    pub text: String,
    
    pub path: Mutex<Vec<(Arc<StoryBlock2>, String, String)>>,
}

impl StoryBlock2 {

    pub fn new(text: &str) -> StoryBlock2 {
        StoryBlock2 { 
            text: text.to_string() , 
            id: String::new(),
            path: Mutex::new(Vec::new()),
        }
    }

    pub fn from_parse(parse: &StoryParse) -> StoryBlock2 {
        StoryBlock2 { 
            id: String::from(&parse.id), 
            text: String::from(&parse.content), 
            path: Mutex::new(Vec::new()),
        }
    }

    pub fn present(&self) -> String {
        let mut built_story;

        built_story = self.text.clone() + "\n";
        for i in self.path.lock().unwrap().iter() {
            // built_story
            let command = format!("{} - {}\n", i.1, i.2);
            built_story = built_story + &command;
        }

        built_story
    }

    

}

pub struct StoryContainer2;

impl TypeMapKey for StoryContainer2 {
    type Value = Arc<RwLock<std::collections::HashMap<String, Arc<StoryBlock2>>>>;
}

