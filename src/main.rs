use encoding::all::WINDOWS_949;
use encoding::{DecoderTrap, Encoding};
use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;
use std::io::BufRead;
use std::path::Path;

lazy_static! {
    static ref COMMENT_RE: Regex = Regex::new(r#"(?m)^(.*?(?:".*")?)(?:\s*;.*)?$"#).unwrap();
}

fn main() -> Result<(), Error> {
    env::args()
        .skip(1)
        .map(|x| Path::new(x.as_str()).to_owned())
        .filter(|x| x.extension().unwrap() == "asm")
        .filter(|x| !x.file_stem().unwrap().to_str().unwrap().ends_with("_strip"))
        .map(fs::canonicalize)
        .map(Result::unwrap)
        .for_each(|mut x| {
            let content = fs::read(x.to_owned()).unwrap();
            let content_str = String::from_utf8(content).unwrap_or_else(|err| {
                WINDOWS_949
                    .decode(err.as_bytes(), DecoderTrap::Strict)
                    .unwrap()
            });
            let filtered = COMMENT_RE
                .captures_iter(&content_str)
                .map(|x| x[1].trim().to_owned())
                .filter(|x| !x.is_empty())
                .fold(String::new(), |acc, x| acc + &x + "\n");
            x.set_file_name(x.file_stem().unwrap().to_str().unwrap().to_owned() + "_strip.asm");
            fs::write(x.to_owned(), filtered.to_owned()).unwrap();
            println!(
                "{}: {} lines",
                x.file_name().unwrap().to_str().unwrap(),
                filtered.lines().count()
            );
        });
    /*
    RE.captures_iter(
        r#"hello!
    world
    asm ; COMMENT
    oh yay
    .STRINGZ ";"
    .STRINGZ ";" ;COMMENT
    good"#,
    )
    .filter(|x| !x[1].is_empty())
    .fold(String::new(), |acc, x| { acc + &x[1] + "\n" })
    */
    println!("Press <enter> to continue...");
    std::io::stdin().lock().lines().next();
    Ok(())
}

/*
fn trim_comment(line: &str) -> &str {}

#[test]
fn trim_comment_test() {
    assert_eq!(trim_comment("hello"), "hello");
}

*/
