extern crate reqwest;
extern crate scraper;
extern crate clap;
extern crate url;
#[macro_use]
extern crate lazy_static;

mod api;
use api::us23;
use clap::{App, SubCommand, Arg};
use url::Url;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

trait IStory {
    fn search(&self, name: &str) -> Vec<SearchResult>;
    fn download(&self, link: &str) -> Box<Iterator<Item = Chapter>>;
}

struct SearchResult {
    name: String,
    link: String,
}

struct Chapter {
    name: String,
    content: String,
}

lazy_static! {
    static ref GLOBAL_BOOK: Mutex<HashMap<&'static str, Arc<IStory+Sync+Send>>> = {
        let mut th: HashMap<&'static str, Arc<IStory+Sync+Send>>= HashMap::new();
        th.insert("us23", Arc::new(us23::US23::new()));
        Mutex::new(th)
    };
}

fn main() {
    let matches = App::new("story")
        .subcommand(SubCommand::with_name("search")
                        .arg(Arg::with_name("name").help("search story")))
        .subcommand(SubCommand::with_name("download").arg(Arg::with_name("link").help("download link")))
        .get_matches();
    match matches.subcommand() {
        ("search", Some(search_command)) => {
            let name = search_command.value_of("name").unwrap();
            let result = GLOBAL_BOOK.lock().unwrap().get("us23").unwrap().search(name);
            for story in result {
                println!("Name: {}  Link: {}", story.name, story.link);
            }
        }
        ("download", Some(get_command)) => {
            let link = get_command.value_of("link").unwrap();
            let content = build(link).unwrap().download(link);
            for chapter in content {
                println!("{}", chapter.name);
                println!("{}", chapter.content);
            }
        }
        _ => unreachable!(),
    }
}

fn build(link: &str) -> Option<Arc<IStory+Send+Sync>> {
    let url = Url::parse(link).unwrap();
    let host: &str = url.host_str().unwrap_or_default();
    if let Some(_) = host.find("23us") {
        return Some(GLOBAL_BOOK.lock().unwrap().get("us23").unwrap().clone())
    }
    None
}
