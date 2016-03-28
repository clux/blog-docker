use glob::glob;
use rustc_serialize::json::{self, ToJson, Json};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use errors::BlogResult;


#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct MetaData {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub latex: bool,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Post {
    pub info: MetaData,
    pub html: String,
}

// Manual ToJson implementations (disappears with serde)
impl ToJson for MetaData {
    fn to_json(&self) -> Json {
        let mut obj = BTreeMap::new();
        obj.insert("date".to_string(), self.date.to_json());
        obj.insert("slug".to_string(), self.slug.to_json());
        obj.insert("title".to_string(), self.title.to_json());
        obj.insert("latex".to_string(), self.latex.to_json());
        Json::Object(obj)
    }
}
impl ToJson for Post {
    fn to_json(&self) -> Json {
        let mut obj = BTreeMap::new();
        obj.insert("info".to_string(), self.info.to_json());
        obj.insert("html".to_string(), self.html.to_json());
        Json::Object(obj)
    }
}

pub type PostMap = BTreeMap<String, Post>;
pub type Posts = Vec<Post>;

fn load_post(slug: &str) -> BlogResult<String> {
    use hoedown::{Markdown, Render};
    use hoedown::renderer::html::{Flags, Html};

    let mut f = try!(File::open(format!("posts/{}/README.md", slug)));
    let mut data = String::new();
    try!(f.read_to_string(&mut data));
    let md = Markdown::new(data.as_str());
    // TODO: markdown need to change relative links to point at slug when converting
    let mut html = Html::new(Flags::empty(), 0);
    let output = html.render(&md);
    let html = try!(output.to_str());
    Ok(html.to_string())
}

pub fn load_posts() -> BlogResult<PostMap> {
    let mut map = PostMap::new();
    let entries = try!(glob("posts/*/data.json"));
    // TODO: parallelize these reads
    for entry in entries {
        let pth = try!(entry);
        // println!("opening path {:?}", pth);
        let mut f = try!(File::open(pth));
        let mut data = String::new();
        try!(f.read_to_string(&mut data));
        let meta: MetaData = try!(json::decode(&data));
        // println!("got metadata {}", json::as_pretty_json(&meta));
        let slug = meta.slug.clone();
        let html = try!(load_post(&slug));
        // println!("got html: {}", html);
        let post = Post {
            info: meta,
            html: html,
        };
        map.insert(slug, post);
    }
    Ok(map)
}

pub fn load_post_vec() -> BlogResult<Posts> {
    let pm = try!(load_posts());

    // TODO: can use collect here with correct derive?
    let mut xs = Vec::new();
    for (_, v) in pm {
        xs.push(v);
    }

    Ok(xs)
}

#[cfg(test)]
mod tests {
    use data;

    #[test]
    fn check_load() {
        let pm = data::load_posts();
        assert!(pm.is_ok(), "could load posts");
    }
}
