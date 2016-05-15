extern crate clap;

extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate mount;
extern crate persistent;
extern crate staticfile;

#[macro_use]
extern crate log;
extern crate loggerv;

extern crate blog;
use blog::{PostMap, BlogResult};

use clap::{Arg, App};

use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use staticfile::Static;
use hbs::{Template, HandlebarsEngine, DirectorySource};

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
    info!("GET /");

    let db = req.extensions.get::<persistent::Read<::DataBase>>().unwrap();

    let mut ctx = BTreeMap::new();
    ctx.insert("posts".to_string(), db.posts.clone());

    resp.set_mut(Template::new("index", ctx)).set_mut(status::Ok);
    Ok(resp)
}

fn entry(req: &mut Request) -> IronResult<Response> {
    let slug = req.extensions.get::<Router>().unwrap().find("slug").unwrap_or("/");
    info!("GET /{}", slug);

    let db = req.extensions.get::<persistent::Read<::DataBase>>().unwrap();

    if let Some(post) = db.posts.get(slug) {
        let mut resp = Response::new();
        resp.set_mut(Template::new("entry", post.clone())).set_mut(status::Ok);
        return Ok(resp);
    }
    Ok(Response::with((status::NotFound)))
}

fn main() {
    let args = App::new("blog")
        .about("stand alone blog engine written in rust\
                \nsource at https://github.com/clux/blog")
        .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("8000"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Use verbose output"))
        .get_matches();

    loggerv::init_with_verbosity(args.occurrences_of("verbose") + 1).unwrap();

    // Load posts
    let db = get_database()
        .map_err(|e| {
            warn!("Failed to load posts: {}", e);
            process::exit(1);
        })
        .unwrap();
    if db.posts.len() == 0 {
        warn!("No posts found in posts/ - clone posts repo first");
        process::exit(1);
    }
    info!("Loaded {} posts", db.posts.len());

    // Load templates
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));
    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    // Set up request pipeline
    let mut router = Router::new();
    router.get("/", index);
    router.get("/:slug", entry);

    let mut mnt = Mount::new();
    mnt.mount("/", router);
    mnt.mount("/static/", Static::new(Path::new("posts/")));

    let mut chain = Chain::new(mnt);


    // allow posts to be read persitently across requests
    chain.link(persistent::Read::<DataBase>::both(db));

    chain.link_after(hbse);

    let addr = format!("{}:{}", "0.0.0.0", args.value_of("port").unwrap());
    match Iron::new(chain).http(addr.as_str()) {
        Ok(_) => info!("Listening on {}", addr),
        Err(error) => error!("Unable to start: {}", error),
    }
}
