use glob::glob;
use regex::Regex;
use serde_json;

use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use std::vec::Vec;
use std::io::Read;

use errors::*;

/// The metadata representation of the `data.json` files
#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    /// Information from the `data.json`
    pub info: MetaData,
    /// `README.md` passed through hoedown
    pub html: String,
    /// The first paragraph, but with a sanity cutoff of 300 characters
    pub summary: String,
}

/// All the data we have in posts/
#[derive(Serialize, Deserialize, Clone)]
pub struct DataBase {
    /// All the posts indexed by slug
    pub posts: BTreeMap<String, Post>,
    /// All the posts in a tera-iterable format
    pub post_list: Vec<Post>,
}

fn parse_markdown(data: &String) -> Result<String> {
    use hoedown::{self, Markdown, Render, Extension};
    use hoedown::renderer::html::{Flags, Html};

    let mut ext = Extension::empty();
    ext.insert(hoedown::FENCED_CODE); // ```code blocks
    ext.insert(hoedown::MATH); // allows using $inline=math$ escaped to \(inline=math\)
    ext.insert(hoedown::MATH_EXPLICIT); // preserve $$displaystyle=math$$
    let md = Markdown::new(data.as_str()).extensions(ext);

    let mut html = Html::new(Flags::empty(), 0);
    let output = html.render(&md);
    let outputstr = output.to_str()
        .chain_err(|| "Failed to convert html to string")?;
    Ok(outputstr.into())
}

// Helper to extract a suitable first paragraph for index
fn generate_summary(md: &String) -> String {
    // find first paragraph from md that does not start with an image
    for l in md.lines() {
        // println!("Found line {}", l);
        if l.starts_with("# ") && l.chars().count() < 80 {
            continue; // Skip headings
        }
        if l.starts_with("![") && l.ends_with(")") {
            continue; // Skip image tags
        }
        if l.chars().count() < 5 {
            continue; // empty lines and weird shit
        }
        // println!("  -> using: {}", l);
        return l.into();
    }
    "<p>No summary</p>".into()
}

/// Replace relative image paths with the mounted /static prefix
pub fn rmap_relative_paths(htmlpost: &str, slug: &str) -> String {
    let re = Regex::new("<img src=\"(./)").unwrap();
    let replacer = format!("<img src=\"/static/{}/", slug);
    re.replace_all(&htmlpost, &replacer as &str).into()
}

// Helper to load parse `README.md` and convert it to `HTML`.
fn load_post(slug: &str) -> Result<(String, String)> {
    let pth = Path::new("./posts").join(slug).join("README.md");
    let pthstr = pth.display().to_string();
    let mut f = File::open(pth)
        .chain_err(|| format!("Failed to open {}", pthstr))?;
    let mut data = String::new();
    f.read_to_string(&mut data)
        .chain_err(|| format!("Failed to read {}", pthstr))?;
    let htmlpost = parse_markdown(&data)
        .chain_err(|| "Failed to parse {} as markdown")?;

    // create markdown summary
    let htmlintro = parse_markdown(&generate_summary(&data))?;

    // remove images from summary
    let image_reg = Regex::new("(<img src=\"[^\"]*\">)").unwrap();
    let htmlintro_safe = image_reg.replace_all(&htmlintro, "");

    Ok((rmap_relative_paths(&htmlpost, slug), htmlintro_safe.into()))
}

/// A one time sequential loader of all posts from the posts folder
///
/// By globbing for `data.json` in subdirectories, we can find the all they keys
/// and metadata. By reading the `README.md` and converting it to HTML via `hoedown`
/// we can build up the values of `DataBase`.
pub fn load_posts() -> Result<DataBase> {
    let mut map = BTreeMap::new();
    // TODO: warn on subdirectories of posts/ not containing data.json
    let entries = glob("posts/*/data.json")
        .chain_err(|| "Failed to glob for posts")?;

    for entry in entries { // iterate over Result objects from Glob
        let pth = entry.chain_err(|| "Failed to glob path")?;
        let resource = pth.display().to_string();
        let mut f = File::open(pth)
            .chain_err(|| format!("Failed to open {}", resource))?;
        let mut data = String::new();
        f.read_to_string(&mut data)
            .chain_err(|| format!("Failed to read {}", resource))?;
        let meta: MetaData = serde_json::from_str(&data)
            .chain_err(|| format!("Failed to deserialize {}", resource))?;
        let slug = meta.slug.clone();
        let (html, summary) = load_post(&slug)?;
        let post = Post {
            info: meta,
            summary: summary,
            html: html,
        };
        map.insert(slug, post);
    }
    let mut vec : Vec<Post> = map.values().cloned().collect();
    vec.reverse();
    Ok(DataBase { posts: map, post_list: vec })
}

#[cfg(test)]
mod tests {
    use data;

    #[test]
    fn check_load() {
        let pm = data::load_posts();
        assert!(pm.is_ok(), "could load posts");
    }

    #[test]
    fn remap_paths() {
        let html = "bah <p><img src=\"./Blah_aw-z2.png\" alt=\"Wz8\"></p>";
        assert_eq!(data::rmap_relative_paths(html, "slug"),
            "bah <p><img src=\"/static/slug/Blah_aw-z2.png\" alt=\"Wz8\"></p>");

    }
}
