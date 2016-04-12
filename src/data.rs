use glob::glob;
use rustc_serialize::json::{self, ToJson, Json};
use regex::Regex;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use errors::BlogResult;

/// The metadata representation of the `data.json` files
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct MetaData {
    /// Post title proper
    pub title: String,
    /// Slugified date + title string corresponding to the folder name
    pub slug: String,
    /// Date ISO string
    pub date: String,
    /// Whether or not LaTeX formatting is used in the markdown
    pub latex: bool,
}

/// The full internal representation of a post subfolder
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Post {
    /// Information from the `data.json`
    pub info: MetaData,
    /// `README.md` passed through hoedown
    pub html: String,
    /// The first paragraph, but with a sanity cutoff of 300 characters
    pub summary: String,
}

/// Manual ToJson implementation (disappears with serde)
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
/// Manual ToJson implementation (disappears with serde)
impl ToJson for Post {
    fn to_json(&self) -> Json {
        let mut obj = BTreeMap::new();
        obj.insert("info".to_string(), self.info.to_json());
        obj.insert("html".to_string(), self.html.to_json());
        obj.insert("summary".to_string(), self.summary.to_json());
        Json::Object(obj)
    }
}

/// Convenience alias
pub type PostMap = BTreeMap<String, Post>;

fn parse_markdown(data: &String) -> BlogResult<String> {
    use hoedown::{self, Markdown, Render, Extension};
    use hoedown::renderer::html::{Flags, Html};

    let mut ext = Extension::empty();
    ext.insert(hoedown::FENCED_CODE); // ```code blocks
    ext.insert(hoedown::MATH); // allows using $inline=math$ escaped to \(inline=math\)
    ext.insert(hoedown::MATH_EXPLICIT); // preserve $$displaystyle=math$$
    let md = Markdown::new(data.as_str()).extensions(ext);

    let mut html = Html::new(Flags::empty(), 0);
    let output = html.render(&md);
    let outputstr = try!(output.to_str()).to_string();
    Ok(outputstr)
}

// Helper to extract a suitable first paragraph for index
fn generate_summary(md: &String) -> String {
    // find first paragraph from md that does not start with an image
    for l in md.lines() {
        //println!("Found line {}", l);
        if l.starts_with("# ") && l.chars().count() < 80 {
            continue // Skip headings
        }
        if l.starts_with("![") && l.ends_with(")") {
            continue // Skip image tags
        }
        if l.chars().count() < 5 {
            continue // empty lines and weird shit
        }
        //println!("  -> using: {}", l);
        return l.to_string();
    }
    "<p>No summary</p>".to_string()
}

// Helper to load parse `README.md` and convert it to `HTML`.
fn load_post(slug: &str) -> BlogResult<(String, String)> {
    let mut f = try!(File::open(format!("posts/{}/README.md", slug)));
    let mut data = String::new();
    try!(f.read_to_string(&mut data));
    let mut htmlpost = try!(parse_markdown(&data));

    // replace relative image paths with their correct path
    let image_path_reg = Regex::new("<img src=\"(./)").unwrap();
    let replacer = format!("<img src=\"static/{}/", slug);
    htmlpost = image_path_reg.replace_all(&htmlpost, &replacer as &str);

    // summary markdown (subset of data)
    let mut htmlintro = try!(parse_markdown(&generate_summary(&data)));

    // remove images from summary
    let image_reg = Regex::new("(<img src=\"[^\"]*\">)").unwrap();
    htmlintro = image_reg.replace_all(&htmlintro, "");

    Ok((htmlpost.to_string(), htmlintro.to_string()))
}

/// A one time sequential loader of all posts from the posts folder
///
/// By globbing for `data.json` in subdirectories, we can find the all they keys
/// and metadata. By reading the `README.md` and converting it to HTML via `hoedown`
/// we can build up the values of `PostMap`.
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
        let (html, summary) = try!(load_post(&slug));
        //println!("got html: {}\n\n and summary: {}\n", html, summary);
        let post = Post {
            info: meta,
            summary: summary,
            html: html,
        };
        map.insert(slug, post);
    }
    Ok(map)
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
