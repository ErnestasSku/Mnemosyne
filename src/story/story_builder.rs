use crate::story::{story::{StoryBlock}, story_parser};

use tracing::{error};

pub fn build_story(file: &str) -> Result<Box<StoryBlock>, String> {
    let parsed = story_parser::story(file);

    println!("{:?}", &parsed);

    if let Err(err) = parsed {
        error!("{}", err);
        return Err("Could not parse the file correctly".to_owned());
    }

    let (rem, parsed) = parsed.unwrap();

    if rem != "" {
        let s = "Unparsed fully".to_owned() + &rem;
        return Err(s);
    }

    let story_blocks: Vec<StoryBlock> = parsed
        .iter()
        .map(|x| StoryBlock::from_parse(x))
        .collect();

    // let new_story: Vec<StoryBlock> = story_blocks
    //     .iter()
    //     .map(|x| x.map_story(story_blocks, &parsed))
    //     .collect();

    // for x in story_blocks.iter() {
        // *x = *x.map_story(&story_blocks, &parsed);
    // }

    let story_blocks: Vec<StoryBlock> = story_blocks
        .iter_mut()
        .map(|x| x.map_story(&story_blocks, &parsed))
        .collect();


    // let head;
    // for x in story_blocks.iter() {
    //     if x.id == "START" {
    //         head = x;
    //         break;
    //     }
    // }

    // Ok(Box::)
    Ok(Box::new(StoryBlock::new("a")))
}