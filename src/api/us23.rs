
use std::io::Read;
use scraper::Html;
use scraper::Selector;
use reqwest;
use std::str::FromStr;
use IStory;
use SearchResult;
use Chapter;

#[derive(Debug)]
struct StoryDir {
    pub name: String,
    pub link: String,
}

pub struct US23 {}
impl US23 {
    pub fn new() -> US23 {
        US23{}
    }
}
struct IterChapter {
    dir :Vec<StoryDir>,
    cur :i32,
}

impl Iterator for IterChapter {
    type Item = Chapter;
    fn next(&mut self) -> Option<Chapter> {
        if self.dir.len() <= self.cur as usize{
            return None;
        }
        let link = &self.dir[self.cur as usize].link;
        let content = get_story_content(&link);
        self.cur += 1;
        Some(Chapter{
            name: self.dir[self.cur as usize].name.clone(),
            content: content,
        })
    }
}

impl IStory for US23 {
    fn search(&self, name: &str) ->Vec<SearchResult> {
        let search_link = format!("http://zhannei.baidu.com/cse/search?q={}&click=1&entry=1&s=5513259216532962936&nsid=", name);
        let mut resp = reqwest::get(&search_link).unwrap();
        let mut content = String::new();
        resp.read_to_string(&mut content);
        let document = Html::parse_document(&content);
        let selector = Selector::parse("a[cpos=title]").unwrap();
        let mut v = Vec::new();
        for element in document.select(&selector) {
            let val = element.value();
            let sd = SearchResult{
                name: String::from_str(val.attr("title").unwrap_or_default()).unwrap(),
                link: String::from_str(val.attr("href").unwrap_or_default()).unwrap(),
            };
            v.push(sd);
        }
        v
    }
    fn download(&self, link: &str) -> Box<Iterator<Item = Chapter>>{
        Box::new(IterChapter{
            dir: get_story_dir(link),
            cur: 0,
        })
    }
}


pub const ORIGIN_LINK: &str = "http://www.23us.so/files/article/html/1/1247/index.html";

fn get_story_dir(link: &str) -> Vec<StoryDir> {
    let mut resp = reqwest::get(link).unwrap();
    let mut content = String::new();
    resp.read_to_string(&mut content);
    let document = Html::parse_document(&content);
    let selector = Selector::parse("td[class=L]>a").unwrap();
    let mut v = Vec::new();
    for element in document.select(&selector) {
        let val = element.value();
        let sd = StoryDir {
            name: element.inner_html(),
            link: String::from_str(val.attr("href").unwrap_or_default()).unwrap(),
        };
        v.push(sd)
    }
    v
}

pub fn get_story_content(link: &str) -> String {
    let mut resp = reqwest::get(link).unwrap();
    let mut content = String::new();
    resp.read_to_string(&mut content);
    let document = Html::parse_document(&content);
    let selector = Selector::parse("dd[id=contents]").unwrap();
    let element = document.select(&selector).next().unwrap();
    let s = element.inner_html();
    let s = s.replace("&nbsp;", "")
        .replace("\n", "")
        .replace("<br>", "\n");

    return s;
}

fn search_story(name: &str) -> Vec<StoryDir> {
    let search_link = format!("http://zhannei.baidu.com/cse/search?q={}&click=1&entry=1&s=5513259216532962936&nsid=", name);
    let mut resp = reqwest::get(&search_link).unwrap();
    let mut content = String::new();
    resp.read_to_string(&mut content);
    let document = Html::parse_document(&content);
    let selector = Selector::parse("a[cpos=title]").unwrap();
    let mut v = Vec::new();
    for element in document.select(&selector) {
        let val = element.value();
        let sd = StoryDir{
            name: String::from_str(val.attr("title").unwrap_or_default()).unwrap(),
            link: String::from_str(val.attr("href").unwrap_or_default()).unwrap(),
        };
        v.push(sd);
    }
    v
}
