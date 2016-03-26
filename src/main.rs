#[macro_use]
extern crate clap;
extern crate glob;
extern crate hoedown;
extern crate pencil;
extern crate rustc_serialize;
//extern crate walkdir;

//use walkdir::WalkDir;
use glob::glob;
use pencil::{Pencil, Request, PencilResult};
use rustc_serialize::json;
use clap::{Arg, App};

use std::collections::HashMap;
use std::fs::File;
//use std::path::Path;
use std::io::{self, Read};
use std::process;
use std::fmt;

#[derive(RustcDecodable, RustcEncodable, Clone)]
struct MetaData {
    title: String,
    date: String,
    slug: String,
    latex: bool,
}

#[derive(Clone)]
struct Post {
    info: MetaData,
    html: String,
}

type PostMap = HashMap<String, Post>;

#[derive(Debug)]
pub enum BlogError {
    Io(io::Error),
    Parse(json::DecoderError),
    Pattern(glob::PatternError),
    Glob(glob::GlobError),
    Unicode(std::str::Utf8Error)
}
pub type BlogResult<T> = Result<T, BlogError>;

// Format implementation used when printing an error
impl fmt::Display for BlogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlogError::Io(ref err) => err.fmt(f),
            BlogError::Parse(ref err) => err.fmt(f),
            BlogError::Pattern(ref err) => err.fmt(f),
            BlogError::Glob(ref err) => err.fmt(f),
            BlogError::Unicode(ref err) => err.fmt(f),
        }
    }
}

// Absorb error types
impl From<io::Error> for BlogError {
    fn from(err: io::Error) -> BlogError { BlogError::Io(err) }
}
impl From<json::DecoderError> for BlogError {
    fn from(err: json::DecoderError) -> BlogError { BlogError::Parse(err) }
}
impl From<glob::PatternError> for BlogError {
    fn from(err: glob::PatternError) -> BlogError { BlogError::Pattern(err) }
}
impl From<glob::GlobError> for BlogError {
    fn from(err: glob::GlobError) -> BlogError { BlogError::Glob(err) }
}
impl From<std::str::Utf8Error> for BlogError {
    fn from(err: std::str::Utf8Error) -> BlogError { BlogError::Unicode(err) }
}

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
    //Ok(html.to_string())
    Ok(html.to_string())
}


fn load_posts() -> BlogResult<PostMap> {
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

fn main() {
    let args = App::new("blog")
        .version(crate_version!())
        .about("clux's blog engine")
        .arg(Arg::with_name("port").short("p").takes_value(true))
        .get_matches();

    let port = args.value_of("port").unwrap_or("8000");

    let posts = load_posts().unwrap(); // TODO: message forgot to clone

    if posts.len() == 0 {
        println!("No posts found in posts/ - clone posts repo first");
        process::exit(1);
    }

    let mut app = Pencil::new("./");
    app.set_debug(true);
    app.set_log_level();
    app.enable_static_file_handling();

    //app.register_template("index.html.hb");
    //app.register_template("entry.html.hb");

    //app.get("/", "index", index);
    //app.get("/<slug>/", "slug", slug);

    let listen = format!("{}:{}", "127.0.0.1", port);
    let listen_str = listen.as_str();
    println!("Listening on {}", listen);
    app.run(listen_str)
}
