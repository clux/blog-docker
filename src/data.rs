use glob::glob;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use rustc_serialize::json;

use errors::{BlogResult};

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct MetaData {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub latex: bool,
}

#[derive(Clone)]
pub struct Post {
    pub info: MetaData,
    pub html: String,
}

pub type PostMap = HashMap<String, Post>;



fn load_post(slug: &str) -> BlogResult<String> {
    use hoedown::{Markdown, Render};
    use hoedown::renderer::html::{Flags, Html};

    let mut f = try!(File::open(format!("posts/{}/README.md", slug)));
    let mut data = String::new();
    try!(f.read_to_string(&mut data));
    let md = Markdown::new(data.as_str());
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
        //println!("opening path {:?}", pth);
        let mut f = try!(File::open(pth));
        let mut data = String::new();
        try!(f.read_to_string(&mut data));
        let meta : MetaData = try!(json::decode(&data));
        //println!("got metadata {}", json::as_pretty_json(&meta));
        let slug = meta.slug.clone();
        let html = try!(load_post(&slug));
        //println!("got html: {}", html);
        let post = Post {
            info: meta,
            html: html
        };
        map.insert(slug, post);
    }
    Ok(map)
}
