extern crate reqwest;
extern crate scraper;
extern crate clap;


mod api;
use std::str::FromStr;
use api::us23;
use clap::{App, SubCommand, Arg};

fn main() {
    let matches = App::new("story").subcommand(SubCommand::with_name("search")
                                               .arg(Arg::with_name("name").help("search story")))
        .subcommand(SubCommand::with_name("get")
                    .arg(Arg::with_name("link").help("download link")))
        .get_matches();
    match matches.subcommand() {
        ("search", Some(search_command)) => {
            let name = search_command.value_of("name").unwrap();
            println!("search arg:{}", search_command.value_of("name").unwrap());
            let result = us23::search_story(name);
            for story in result {
                println!("Name: {}  Link: {}", story.name, story.link);
            }
        },
        ("get", Some(get_command)) => {
            let link = get_command.value_of("link").unwrap();
            let book = us23::get_story_dir(String::from_str(link).unwrap());
            for chapter in book {
                println!("{}", chapter.name);
                let s = us23::get_story_content(&chapter.link);
                println!("{}", s);
            }
        }
        _ => unreachable!(),
    }
}
