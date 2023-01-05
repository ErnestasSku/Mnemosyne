use std::sync::Arc;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::RwLock;


pub struct StoryListener<'a> {
    listener: UserId,
    current_story_path: &'a StoryBlock
}

#[derive(Debug)]
pub struct StoryBlock {
    id: String,
    key: String,
    key_label: String,
    text: String,
    paths: Vec<StoryBlock>
}

impl StoryBlock {
    fn new(text: &str) -> StoryBlock {
        StoryBlock { 
            text: text.to_string() , 
            paths: Vec::new(),
            id: String::new(),
            key: String::new(),
            key_label: String::new()
        }
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
            StoryBlock::new("The left path"),
            StoryBlock::new("The middle path"),
            StoryBlock::new("The right path"),
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