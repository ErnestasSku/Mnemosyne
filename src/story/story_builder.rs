use std::{sync::Arc, fs};

use crate::story::{story_parser::*, story2::StoryBlock2};

// use tracing::{error};


fn find_paths(id: String, parsed: &Vec<StoryParse>) -> Vec<(String, String, String)> {
    let mut res = Vec::new();
    for x in parsed.iter() {
        if x.id == id {


            for y in x.children.iter() {
                res.push(
                    (y.next_path.clone(),
                    y.command.clone(),
                    y.label.clone())
                );
            }

        }
    }
    res
}

fn find_blocks(ids: Vec<(String, String, String)>, blocks: &Vec<StoryBlock2>, ) -> Vec<(Arc<StoryBlock2>, String, String)>{
    let mut res = Vec::new();
    for x in blocks.iter() {

        for t in ids.iter() {
            if t.0 == x.id {
                let arc = Arc::new(x.clone());
                res.push(
                    (arc,
                    t.1.clone(),
                    t.2.clone()
                    ));
            }
        }
    }
    res
}

pub fn map_stories(file: &String) -> Result<StoryBlock2, String> {
    let file = fs::read_to_string(file);
    if let Err(error) = file {
        return  Err("Error happened during file read".to_string() + &error.to_string());
    }

    let file_s = &file.unwrap()[..];

    let parsed = story(file_s);
    let (_, parsed) = parsed.unwrap();

    let mut story_blocks: Vec<StoryBlock2> = parsed.
        iter()
        .map(|x| StoryBlock2::from_parse(x))
        .collect();

    for x in parsed.iter() {
        println!("{:?}\n", x);
    }

    let mut copied: Vec<StoryBlock2> = Vec::new();
    for x in story_blocks.iter() {
        copied.push(x.clone());
    }

    for current in story_blocks.iter_mut() {
        let paths = find_paths(current.id.clone(), &parsed);
        let paths = find_blocks(paths, &copied);
        
        current.path = paths;
    }
    
    let mut head = Err("No story was created".to_string());
    for x in story_blocks.iter() {
        if x.id == "START" {
            head = Ok(x.clone());
        }
    }
    head
}
