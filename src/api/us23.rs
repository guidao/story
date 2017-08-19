
use std::io::Read;
use std::io;
use scraper::Html;
use scraper::Selector;
use reqwest;
use std::str::FromStr;
use IStory;
use SearchResult;
use Chapter;

// pub const ORIGIN_LINK: &str = "http://www.23us.so/files/article/html/1/1247/index.html";

#[derive(Debug)]
struct StoryDir {
    pub name: String,
    pub link: String,
}

#[derive(Clone)]
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
        search_story(name).unwrap_or_default()
    }
    fn download(&self, link: &str) -> Box<Iterator<Item = Chapter>>{
        Box::new(IterChapter{
            dir: get_story_dir(link),
            cur: 0,
        })
    }
}

enum StoryErr {
    ERR1,
}

impl From<()> for StoryErr {
    fn from(_ :()) ->StoryErr {
        return StoryErr::ERR1;
    }
}

impl From<io::Error> for StoryErr {
    fn from(_: io::Error) ->StoryErr{
        return StoryErr::ERR1;
    }
}

impl From<reqwest::Error> for StoryErr {
    fn from(_: reqwest::Error) ->StoryErr{
        return StoryErr::ERR1;
    }
}


fn get_story_dir(link: &str) -> Vec<StoryDir> {
    get_story_dir_result(link).unwrap_or_default()
}

fn get_story_dir_result(link:&str) -> Result<Vec<StoryDir>, StoryErr> {
    let mut resp = reqwest::get(link)?;
    let mut content = String::new();
    resp.read_to_string(&mut content)?;
    let document = Html::parse_document(&content);
    let selector = Selector::parse("td[class=L]>a")?;
    let mut v = Vec::new();
    for element in document.select(&selector) {
        let val = element.value();
        let sd = StoryDir {
            name: element.inner_html(),
            link: String::from_str(val.attr("href").unwrap_or_default()).unwrap(),
        };
        v.push(sd)
    }
    Ok(v)
}

pub fn get_story_content(link: &str) -> String {
    get_story_content_result(link).unwrap_or_default()
}

fn get_story_content_result(link :&str) ->Result<String, StoryErr> {
    let mut resp = reqwest::get(link)?;
    let mut content = String::new();
    resp.read_to_string(&mut content)?;
    let document = Html::parse_document(&content);
    let selector = Selector::parse("dd[id=contents]")?;
    let element = document.select(&selector).next().unwrap();
    let s = element.inner_html();
    let s = s.replace("&nbsp;", "")
        .replace("\n", "")
        .replace("<br>", "\n");

    Ok(s)
}




fn search_story(name: &str) -> Result<Vec<SearchResult>, StoryErr> {
    let mut v = Vec::new();
    let search_link = format!("http://zhannei.baidu.com/cse/search?q={}&click=1&entry=1&s=5513259216532962936&nsid=", name);
    let mut resp = reqwest::get(&search_link)?;
    let mut content = String::new();
    resp.read_to_string(&mut content)?;
    let document = Html::parse_document(&content);
    let desc = Selector::parse("div[class=result-game-item-detail]")?;
    for book in document.select(&desc) {
        let author_seletor = Selector::parse("p[class=result-game-item-info-tag]>span")?;
        let mut auth = book.select(&author_seletor);
        auth.next();
        let auther = auth.next().unwrap().inner_html();
        let selector = Selector::parse("a[cpos=title]")?;
        for element in book.select(&selector) {
            let val = element.value();
            let sd = SearchResult{
                name: String::from_str(val.attr("title").unwrap_or_default()).unwrap(),
                link: String::from_str(val.attr("href").unwrap_or_default()).unwrap(),
                author: String::from_str(auther.trim()).unwrap_or_default(),
            };
            v.push(sd);
            break;
        }
    }

    return Ok(v)
}




