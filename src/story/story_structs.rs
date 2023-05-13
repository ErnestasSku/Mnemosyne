use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use serenity::builder::{CreateComponents, CreateMessage};

use super::story_parser::StoryParse;

#[derive(Debug, Clone)]
pub struct StoryBlock {
    pub id: String,
    pub text: String,

    pub path: Arc<Mutex<StoryPaths>>,
}

pub type StoryPath = (Arc<StoryBlock>, String, String);
pub type StoryPaths = Vec<StoryPath>;

impl StoryBlock {
    pub fn from_parse(parse: &StoryParse) -> StoryBlock {
        StoryBlock {
            id: String::from(&parse.id),
            text: String::from(&parse.content),
            path: Arc::new(Mutex::new(Vec::new())),
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

    pub fn present_interactive(&self) -> (String, CreateComponents) {
        // let ret = CreateMessage::default()
        let components = CreateComponents::default()
            // .content("test content")
            // .components(|c| {
            .create_action_row(|row| {
                for i in self.path.lock().unwrap().iter() {
                    row.create_button(|button| {
                        button.custom_id(i.1.clone());
                        button.label(i.2.clone());
                        button
                    });
                }
                row
            })
            .to_owned();

        // .to_owned();

        (self.text.clone(), components)
    }

    pub fn story_to_list_unique(
        story: &Arc<StoryBlock>,
        visited: &mut HashSet<String>,
        res: &mut Vec<Arc<StoryBlock>>,
    ) {
        let s = story.id.clone();

        visited.insert(s);
        res.push(story.clone());

        for i in story.path.lock().unwrap().iter() {
            if !visited.contains(&i.0.id) {
                StoryBlock::story_to_list_unique(&i.0, visited, res);
            }
        }
    }
}

pub struct StoryContainer;
