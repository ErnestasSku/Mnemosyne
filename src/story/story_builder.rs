use std::{sync::Arc, fs, path};

use crate::story::{story_parser::*, story2::StoryBlock2};

// use tracing::{error};


// fn find_paths(id: String, parsed: &Vec<StoryParse>) -> Vec<(String, String, String)> {
//     let mut res = Vec::new();
//     for x in parsed.iter() {
//         if x.id == id {
//             for y in x.children.iter() {
//                 res.push(
//                     (y.next_path.clone(),
//                     y.command.clone(),
//                     y.label.clone())
//                 );
//             }

//         }
//     }
//     res
// }

// fn find_blocks(ids: Vec<(String, String, String)>, blocks: &Vec<StoryBlock2>, ) -> Vec<(Arc<StoryBlock2>, String, String)>{
//     let mut res = Vec::new();
//     for x in blocks.iter() {

//         for t in ids.iter() {
//             if t.0 == x.id {
//                 let arc = Arc::new(x.clone());
//                 res.push(
//                     (arc,
//                     t.1.clone(),
//                     t.2.clone()
//                     ));
//             }
//         }
//     }
//     res
// }

// pub fn map_stories(file: &String) -> Result<StoryBlock2, String> {
//     let file = fs::read_to_string(file);
//     if let Err(error) = file {
//         return  Err("Error happened during file read: ".to_string() + &error.to_string());
//     }

//     let file_s = &file.unwrap()[..];

//     let parsed = story(file_s);
//     let (_, parsed) = parsed.unwrap();

//     let mut story_blocks: Vec<StoryBlock2> = parsed.
//         iter()
//         .map(|x| StoryBlock2::from_parse(x))
//         .collect();

//     // for x in parsed.iter() {
//     //     println!("{:#?}\n", x);
//     // }

//     let mut copied: Vec<StoryBlock2> = Vec::new();
//     for x in story_blocks.iter() {
//         copied.push(x.clone());
//     }

//     for current in story_blocks.iter_mut() {
//         let paths = find_paths(current.id.clone(), &parsed);
//         let paths = find_blocks(paths, &copied);
        
//         current.path = paths;
//     }
    
//     let mut head = Err("No story was created".to_string());
//     for x in story_blocks.iter() {
//         if x.id == "START" {
//             head = Ok(x.clone());
//         }
//     }
//     head
// }

pub fn map_stories(file: &String) -> Result<StoryBlock2, String> {
    Err("Temp".to_owned())
}

pub fn map_stories_p(file: &String) -> Result<Arc<StoryBlock2>, String> {

    let file = fs::read_to_string(file);
    if let Err(error) = file {
        return  Err("Error happened during file read: ".to_string() + &error.to_string());
    }

    let file_s = &file.unwrap()[..];

    let parsed = story(file_s);
    let (_, parsed) = parsed.unwrap();

    let mut story_blocks: Vec<Arc<StoryBlock2>> = parsed.
        iter()
        .map(|x| StoryBlock2::from_parse(x))
        .map(|x| Arc::new(x))
        .collect();

    // for current in story_blocks.iter_mut() {

    // }

    for i in 0..story_blocks.len() {
        let copied: Vec<Arc<StoryBlock2>> = story_blocks.iter().cloned().collect();
        let current = &mut story_blocks[i];
        
        let mut to_map: Vec<(String, String, String)> = Vec::new();
        let mut paths: Vec<(Arc<StoryBlock2>, String, String)> = Vec::new();
        
        //Find paths from parse
        for p in parsed.iter() {
            if p.id == current.id {
                for p_child in p.children.iter() {
                    to_map.push((
                        p_child.next_path.clone(),
                        p_child.command.clone(),
                        p_child.label.clone()
                    ));

                }
            }
        }


        //Find blocks   
        for item in copied.iter() {
            let elem = to_map.iter().find(|x| x.0 == item.id);

            if let Some(el) = elem {
                paths.push((
                    item.clone(),
                    el.1.clone(),
                    el.2.clone()
                ));
            }
        }
        *current.path.lock().unwrap() = paths;
        
    }

    let mut head = Err("No story was created".to_string());
    for x in story_blocks.iter() {
        if x.id == "START" {
            head = Ok(x.clone());
        }
    }
    head
}

// impl Deref for StoryBlock2 {
//     type Target = Vec<(Arc<StoryBlock2>, String, String)>;

//     fn deref(&self) -> &Self::Target {
//         &self.path
//     }
// }

// impl DerefMut for StoryBlock2 {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.path
//     }
// }