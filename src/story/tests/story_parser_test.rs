#[cfg(test)]
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

