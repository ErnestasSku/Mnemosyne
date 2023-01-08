extern crate nom;
use nom::{
    IResult,
    bytes::complete::{tag, take_while},
    sequence::{tuple, delimited}, character::complete::{alphanumeric1, multispace0, alphanumeric0}, multi::many0
};


#[derive(Debug, PartialEq)]
pub struct StoryParse {
    pub id: String,
    pub content: String,
    pub children: Vec<Path>,

}
#[derive(Debug, PartialEq)]
pub struct Path {
    pub next_path: String,
    pub command: String,
    pub label: String,
}

impl StoryParse {


}


fn story_id(input: &str) -> IResult<&str, String> {
    tuple((multispace0, delimited(tag("["), alphanumeric1, tag("]"))))
        (input)
        .map(|(next_input, (_, id))| (next_input, id.into()))
}

fn story_text(input: &str) -> IResult<&str, String> {
    take_while(is_not_command)
    (input)
    .map(|(next_input, parsed)| (next_input, parsed.into()))
}

fn story_command(input: &str) -> IResult<&str, Path> {
    tuple((
        tag("\\"), alphanumeric0, multispace0,
        delimited(tag("("), alphanumeric0, tag(")")), multispace0,
        delimited(tag("{"), take_while(is_not_bracket)  , tag("}")), multispace0,
    ))
    (input)
    .map(|(left_input, res)| {
        let (_, id, _, command, _, label, _) = res;
        
        (left_input, Path {
            next_path: id.into(),
            command: command.into(),
            label: label.into(),
        })
    } 
    )
}

fn story_block(input: &str) -> IResult<&str, StoryParse> {
    tuple((
        story_id,
        story_text,
        many0(story_command)
    ))
    (input)
    .map(|(left_input, (id, text, command))| (left_input,
        StoryParse {
            id: id,
            content: text, 
            children: command
        }
    ))
}

pub fn story(input: &str) -> IResult<&str, Vec<StoryParse>> {
    many0(story_block)(input)
}

fn is_not_command(chr: char) -> bool {
    chr != '\\'
}

fn is_not_bracket(chr: char) -> bool {
    chr != '}'
}


#[cfg(test)]
mod test {    
    use crate::story::story_parser::*;

    #[test]
    fn test_id() {
        let parse = story_id("[asd]");
    
        let ok = parse.is_ok();
        assert_eq!(ok, true);
        if ok {
            let (s, _) = parse.unwrap();
            assert_eq!(s, "".to_owned(), "All input should have been consumed");
        } 
    }
    
    #[test]
    fn test_story_text() {
        let parse = story_text("sadbnk");
    
        let ok = parse.is_ok();
        assert_eq!(ok, true);
        if ok {
            let (s, _) = parse.unwrap();
            assert_eq!(s, "".to_owned(), "All input should have been consumed");
        } 
    }
    
    #[test]
    fn test_story_command() {
        let parse = story_command("\\WEST1 (west){You think of going west}");

        let ok = parse.is_ok();
        assert_eq!(ok, true);
        if ok {
            let (s, _) = parse.unwrap();
            assert_eq!(s, "".to_owned(), "All input should have been consumed");
        } 

    }
    
    #[test]
    fn test_block() {
        let str = "[START]
        You enter a forest.
        \\WEST1 (west){You think of going west} 
        \\EAST1 (east){You think of going east}
        \\Lay (lay){You decide to take a rest}";
    
        let parse = story_block(str);

        let ok = parse.is_ok();
        assert_eq!(ok, true);
        if ok {
            let (s, _) = parse.unwrap();
            assert_eq!(s, "".to_owned(), "All input should have been consumed");
        } 
    
    }
    
    #[test]
    fn test_final() {
        let str = "[START]
        You enter a forest.
        \\WEST1 (west){You think of going west} 
        \\EAST1 (east){You think of going east}
        \\Lay (lay){You decide to take a rest}
        
        [WEST1]
        You go west
        \\East (east){east}
        ";
    
        let parse = story(str);
    
        let ok = parse.is_ok();
        assert_eq!(ok, true);
        if ok {
            let (s, _) = parse.unwrap();
            assert_eq!(s, "".to_owned(), "All input should have been consumed");
        } 
    }

}