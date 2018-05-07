
use std::io::Read;
use std::io;
use scraper::Html;
use scraper::Selector;
use reqwest;
use std::str::FromStr;
use IStory;
use SearchResult;
use Chapter;
use ChapterDesc;
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
        search_story2(name).unwrap_or_default()
    }
    fn download(&self, link: &str) -> Box<Iterator<Item = Chapter>>{
        Box::new(IterChapter{
            dir: get_story_dir(link),
            cur: 0,
        })
    }
    fn content(&self, link: &str) -> Vec<ChapterDesc> {
        get_story_dir(link).into_iter().map(|x|{
            ChapterDesc{
                name: x.name,
                link: x.link,
            }
        }).collect()
    }
    fn get(&self, link: &str) ->String{
        get_story_content(link)
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
        let auther = match book.select(&author_seletor).nth(1){
            Some(inner) => inner.inner_html(),
            None => "".to_owned(),
        };
        let selector = Selector::parse("a[cpos=title]")?;
        book.select(&selector).next().and_then(|inner|{
            let val = inner.value();
            let sd = SearchResult{
                name: String::from(val.attr("title").unwrap_or_default()),
                link: String::from(val.attr("href").unwrap_or_default()),
                author: String::from_str(auther.trim()).unwrap_or_default(),
                img: String::from(""),
                desc: String::from(""),
            };
            v.push(sd);
            Some(())
        });
    }
    return Ok(v)
}


fn search_story2(name: &str) -> Result<Vec<SearchResult>, StoryErr> {
    let mut v = Vec::new();
    let search_link = format!("http://zhannei.baidu.com/cse/search?q={}&click=1&entry=1&s=5513259216532962936&nsid=", name);
    let mut resp = reqwest::get(&search_link)?;
    let mut content = String::new();
    resp.read_to_string(&mut content)?;
    let document = Html::parse_document(&content);
    let desc = Selector::parse("div[class='result-item result-game-item']")?;
    for book in document.select(&desc) {
        let img = Selector::parse("img[class=result-game-item-pic-link-img]")?;
        let imgUrl = match book.select(&img).nth(0) {
            Some(inner) => {
                String::from(inner.value().attr("src").unwrap_or_default())
            },
            None => "".to_owned(),
        };

        let author_seletor = Selector::parse("p[class=result-game-item-info-tag]>span")?;
        let auther = match book.select(&author_seletor).nth(1){
            Some(inner) => inner.inner_html(),
            None => "".to_owned(),
        };
        let selector = Selector::parse("a[cpos=title]")?;
        book.select(&selector).next().and_then(|inner|{
            let val = inner.value();
            let sd = SearchResult{
                name: String::from(val.attr("title").unwrap_or_default()),
                link: String::from(val.attr("href").unwrap_or_default()),
                author: String::from(auther.trim()),
                img: imgUrl,
                desc: String::from("暂无描述"),
            };
            v.push(sd);
            Some(())
        });
    }
    println!("hello outer");
    return Ok(v)
}


