#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate reqwest;
extern crate scraper;
extern crate url;

#[macro_use]
extern crate lazy_static;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod api;
use api::us23;
use url::Url;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use rocket_contrib::Json;
use rocket::fairing::*;
use rocket::{Response, Request, Data};


lazy_static! {
    static ref GLOBAL_BOOK: Mutex<HashMap<&'static str, Arc<IStory+Sync+Send>>> = {
        let mut th: HashMap<&'static str, Arc<IStory+Sync+Send>>= HashMap::new();
        th.insert("us23", Arc::new(us23::US23::new()));
        Mutex::new(th)
    };
}


#[derive(Default)]
struct Cors {
}

impl Fairing for Cors {
    fn info(&self) -> Info{
        Info{
            name: "my cors",
            kind: Kind::Response | Kind::Request,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_raw_header("Access-Control-Allow-Origin", "*");
        response.set_raw_header("Access-Control-Allow-Credentials", "true");
        response.set_raw_header("Access-Control-Allow-Headers", "X-Requested-With,Content-Type,Ajax");
    }


    fn on_request(&self, request: &mut Request, data: &Data) {
        if request.method() != rocket::http::Method::Options {
            return
        }
        request.set_uri("/cors")
    }
}




#[derive(Serialize, Deserialize, Debug)]
pub struct SearchReq {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResp {
    books: Vec<SearchResult>
}

#[options("/cors")]
fn cors() {
}

#[post("/search", format = "application/json", data="<searchReq>")]
fn search(searchReq :Json<SearchReq>) -> Json<SearchResp>{
    println!("name: {}", searchReq.name);
    let re = build("http://23us.com").unwrap().search(&searchReq.name);
    Json(SearchResp{books: re})
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ListReq {
    link: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ListResp {
    chapters: Vec<ChapterDesc>
}


#[post("/list", format = "application/json", data="<listReq>")]
fn list(listReq: Json<ListReq>) -> Json<ListResp> {
    let chapters = build(&listReq.link).unwrap().content(&listReq.link);
    Json(ListResp{chapters: chapters})
}

#[derive(Serialize, Deserialize, Debug)]
struct GetReq {
    link: String
}

#[derive(Serialize, Deserialize, Debug)]
struct GetResp {
    chapter: Chapter
}

#[post("/get", format = "application/json", data="<req>")]
fn get(req: Json<GetReq>) -> Json<GetResp> {
    let chapter = build(&req.link).unwrap().get(&req.link);
    Json(GetResp{chapter: Chapter{content: chapter, name: String::from("第n章")}})
}



trait IStory {
    fn search(&self, name: &str) -> Vec<SearchResult>;
    fn download(&self, link: &str) -> Box<Iterator<Item = Chapter>>;
    fn content(&self, link: &str) -> Vec<ChapterDesc>;
    fn get(&self, link: &str) -> String;
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SearchResult {
    name: String,
    author: String,
    link: String,
    img: String,
    desc: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    name: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChapterDesc {
    pub name: String,
    pub link: String,
}



fn main() {
    rocket::ignite().mount("/", routes![search, list, get, cors]).attach(Cors::default()).launch();
}

fn build(link: &str) -> Option<Arc<IStory+Send+Sync>> {
    let url = Url::parse(link).unwrap();
    let host: &str = url.host_str().unwrap_or_default();
    if let Some(_) = host.find("23us") {
        println!("23us, find");
        return Some(GLOBAL_BOOK.lock().unwrap().get("us23").unwrap().clone())
    }
    None
}
