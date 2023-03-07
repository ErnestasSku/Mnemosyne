use std::sync::{Arc, Mutex};

use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

use super::story_parser::StoryParse;

#[derive(Debug)]
pub struct StoryBlock {
    pub id: String,
    pub text: String,

    pub path: Mutex<Vec<(Arc<StoryBlock>, String, String)>>,
}

impl StoryBlock {
    pub fn from_parse(parse: &StoryParse) -> StoryBlock {
        StoryBlock {
            id: String::from(&parse.id),
            text: String::from(&parse.content),
            path: Mutex::new(Vec::new()),
        }
    }

    pub fn present(&self) -> String {
        let mut built_story;

        built_story = self.text.clone() + "\n";

        //Leaving unwrap for now here. Note: Come back here when I now more about rust.
        for i in self.path.lock().unwrap().iter() {
            // built_story
            let command = format!("{} - {}\n", i.1, i.2);
            built_story = built_story + &command;
        }

        built_story
    }
}

pub struct StoryContainer;

impl TypeMapKey for StoryContainer {
    type Value = Arc<RwLock<std::collections::HashMap<String, Arc<StoryBlock>>>>;
}
