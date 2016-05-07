extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate logger;
extern crate mount;
extern crate persistent;
extern crate staticfile;

extern crate blog;
use blog::{PostMap, BlogResult};

use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use staticfile::Static;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use logger::Logger;

// Create our own filesystem backed database which implements the iron Key trait
// This allows the DataBase type to be re-used in all connection handlers
pub struct DataBase {
    pub posts: PostMap,
}
impl iron::typemap::Key for DataBase {
    type Value = DataBase;
}
pub fn get_database() -> BlogResult<DataBase> {
    Ok(DataBase { posts: try!(blog::load_posts()) })
}

use std::collections::BTreeMap;
use std::process;
use std::path::Path;

fn index(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let db = req.extensions.get::<persistent::Read<::DataBase>>().unwrap();

    let mut ctx = BTreeMap::new();
    ctx.insert("posts".to_string(), db.posts.clone());

    resp.set_mut(Template::new("index", ctx)).set_mut(status::Ok);
    Ok(resp)
}

fn entry(req: &mut Request) -> IronResult<Response> {
    let slug = req.extensions.get::<Router>().unwrap().find("slug").unwrap_or("/");

    let db = req.extensions.get::<persistent::Read<::DataBase>>().unwrap();

    if let Some(post) = db.posts.get(slug) {
        let mut resp = Response::new();
        resp.set_mut(Template::new("entry", post.clone())).set_mut(status::Ok);
        return Ok(resp);
    }
    Ok(Response::with((status::NotFound)))
}

fn main() {
    // Load posts
    let db = get_database()
        .map_err(|e| {
            println!("Failed to load posts: {}", e);
            process::exit(1);
        })
        .unwrap();
    if db.posts.len() == 0 {
        println!("No posts found in posts/ - clone posts repo first");
        process::exit(1);
    }
    println!("Loaded {} posts", db.posts.len());

    // Load templates
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));
    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    // Set up request pipeline
    let (logger_before, logger_after) = Logger::new(None);

    let mut router = Router::new();
    router.get("/", index);
    router.get("/:slug", entry);

    let mut mnt = Mount::new();
    mnt.mount("/", router);
    mnt.mount("/static/", Static::new(Path::new("posts/")));

    let mut chain = Chain::new(mnt);
    chain.link_before(logger_before);

    // allow posts to be read persitently across requests
    chain.link(persistent::Read::<DataBase>::both(db));

    chain.link_after(hbse);
    chain.link_after(logger_after);

    let addr = format!("{}:{}", "0.0.0.0", 8000);
    match Iron::new(chain).http(addr.as_str()) {
        Ok(_) => println!("Listening on {}", addr),
        Err(error) => println!("Unable to start: {}", error),
    }
}
