use std::rc::Rc;
use std::sync::Arc;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::RwLock;

use super::story_parser::StoryParse;


pub struct StoryListener<'a> {
    listener: UserId,
    current_story_path: &'a StoryBlock
}

#[derive(Debug, Clone)]
pub struct StoryBlock {
    pub id: String,
    pub text: String,
    
    pub key: String,
    pub key_label: String,
    pub paths: Vec<Arc<StoryBlock>>
}

impl StoryBlock {
    pub fn new(text: &str) -> StoryBlock {
        StoryBlock { 
            text: text.to_string() , 
            paths: Vec::new(),
            id: String::new(),
            key: String::new(),
            key_label: String::new()
        }
    }

    pub fn from_parse(parse: &StoryParse) -> StoryBlock {
        StoryBlock { 
            id: String::from(&parse.id), 
            text: String::from(&parse.content), 
            key: "".into(), 
            key_label: "".into(), 
            paths: vec!() 
        }
    }

    pub fn map_story(mut self, stories: &Vec<StoryBlock>, parses: &Vec<StoryParse>) -> StoryBlock {

        let ids :Vec<String> = parses
            .iter()
            .filter(|x| x.id == self.id)
            .map(|x| String::from(&x.id))
            .collect();

        let mut filtered_stories = Vec::new();
        for x in stories {
            if ids.contains(&x.id) {
                let a = x.to_owned();
                let b = Arc::new(a);
                filtered_stories.push(b);
            }
        }

        self.paths = filtered_stories;
        self
    }
}


pub struct StoryContainer;

impl TypeMapKey for StoryContainer {
    type Value = Arc<RwLock<std::collections::HashMap<String, StoryBlock>>>;
} 


#[command]
async fn start_story(_ctx: &Context, _msg: &Message) -> CommandResult {

    Ok(())
}

#[command]
async fn load(ctx: &Context, _msg: &Message) -> CommandResult {
    let head = StoryBlock {
        id: String::new(),
        key: String::new(),
        key_label: String::new(),
        text: "The beginning of the story".to_string(),
        paths: vec![
            Arc::new(StoryBlock::new("The left path")),
            Arc::new(StoryBlock::new("The middle path")),
            Arc::new(StoryBlock::new("The right path")),
        ]
    };

    let story_lock = {
        let data_read = ctx.data.read().await;

        data_read.get::<StoryContainer>().expect("Expected StoryBlock in TypeMap.").clone()
    };
    
    println!("load - outter");

    {
        println!("load");
        let mut stories = story_lock.write().await;
        
        stories.insert("head".to_string(), head);
        println!("{:?}", &stories);
    }



    Ok(())
}

#[command]
async fn read(ctx: &Context, _msg: &Message) -> CommandResult {

    let story_lock = {
        let data_read = ctx.data.read().await;

        data_read.get::<StoryContainer>().expect("Expected StoryBlock in TypeMap.").clone()
    };

    {
        let stories = story_lock.read().await;
        
        println!("{:?}", stories);
        println!("{:?}", stories.get("head"));
    }

    Ok(())
}