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
        .map(|x| (fs::read(x.to_owned()), x.to_owned()))
        .map(|(x, y)| (Result::unwrap(x), y))
        .map(|(x, y)| (String::from_utf8(x), y))
        .map(|(x, y)| (Result::unwrap(x), y))
        .map(|(x, y)| {
            (
                COMMENT_RE
                    .captures_iter(&x)
                    .map(|x| x[1].trim().to_owned())
                    .filter(|x| !x.is_empty())
                    .fold(String::new(), |acc, x| acc + &x + "\n"),
                y,
            )
        })
        .map(|(x, mut y)| {
            y.set_file_name(
                y.file_stem().unwrap().to_str().unwrap().to_owned()
                    + "_strip"
                    + "."
                    + y.extension().unwrap().to_str().unwrap(),
            );
            fs::write(y.to_owned(), x.to_owned()).unwrap();
            (x.lines().count(), y)
        })
        .for_each(|(x, y)| {
            println!("{}: {} lines", y.file_name().unwrap().to_str().unwrap(), x);
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
