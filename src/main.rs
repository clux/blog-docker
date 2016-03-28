#[macro_use]
extern crate clap;
extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate logger;

extern crate blog;
use blog::*;
use clap::{Arg, App};

use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use logger::Logger;

use std::collections::BTreeMap;
use std::process;

fn index(_: &mut Request) -> IronResult<Response> {
    //println!("Got url {}", req.url);
    let mut resp = Response::new();

    let posts = data::load_post_vec().unwrap(); // TODO: iron state?

    let mut ctx = BTreeMap::new();
    ctx.insert("posts".to_string(), posts);

    resp.set_mut(Template::new("index", ctx)).set_mut(status::Ok);
    Ok(resp)
}

fn entry(req: &mut Request) -> IronResult<Response> {
    let slug = req.extensions.get::<Router>().unwrap().find("slug").unwrap_or("/");
    //println!("Got url {} {}", req.url, slug);

    let posts = data::load_posts().unwrap(); // TODO: iron state?
    if let Some(post) = posts.get(slug) {
        let mut resp = Response::new();
        resp.set_mut(Template::new("entry", post.clone())).set_mut(status::Ok);
        return Ok(resp);
    }
    Ok(Response::with((status::NotFound)))
}

fn main() {
    let args = App::new("blog")
        .version(crate_version!())
        .about("blog server")
        .arg(Arg::with_name("port").short("p").takes_value(true))
        .get_matches();

    let port = args.value_of("port").unwrap_or("8000");

    let posts = match data::load_posts() {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to load posts: {}", e);
            process::exit(1);
        }
    };

    if posts.len() == 0 {
        println!("No posts found in posts/ - clone posts repo first");
        process::exit(1);
    }
    println!("Loaded {} posts", posts.len());

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));
    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let (logger_before, logger_after) = Logger::new(None);

    let mut router = Router::new();
    router.get("/", index);
    router.get("/:slug", entry);

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(hbse);
    chain.link_after(logger_after);

    let addr = format!("{}:{}", "127.0.0.1", port);
    println!("Listening on {}", addr);
    Iron::new(chain).http(addr.as_str()).unwrap();
}

