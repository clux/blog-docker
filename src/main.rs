#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

extern crate blog;

use rocket::response::NamedFile;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::Template;
use rocket::response::Failure;

use blog::PostMap;
use std::process;
use std::path::{Path, PathBuf};

#[get("/")]
fn index(db: State<PostMap>) -> Template {
    // TODO: render this properly
    Template::render("index", &db.clone())
}

#[get("/<slug>")]
fn entry(db: State<PostMap>, slug: &str) -> Result<Template, Failure> {
    // http://localhost:8000/e/2013-03-20-colemak
    if let Some(post) = db.get(slug) {
        Ok(Template::render("entry", &post))
    } else {
        Err(Failure(Status::NotFound))
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    // http://localhost:8000/static/2013-03-20-colemak/Colemak_fingers-600.png
    NamedFile::open(Path::new("posts/").join(file)).ok()
}

fn main() {
    // Load posts
    let db = blog::load_posts()
        .map_err(|e| {
            println!("Failed to load posts: {}", e);
            for e in e.iter().skip(1) {
                println!("Caused by: {}", e);
            }
            //if let Some(backtrace) = e.backtrace() {
            //    println!("backtrace: {:?}", backtrace);
            //}
            process::exit(1);
        })
        .unwrap();
    if db.len() == 0 {
        println!("No posts found in posts/ - clone posts repo first");
        process::exit(1);
    }
    println!("Loaded {} posts", db.len());
    // TODO: load templates in ./templates (currently .hbs - can be changed)

    rocket::ignite()
        .manage(db)
        .mount("/", routes![index, entry])
        .mount("/static/", routes![files])
        .launch();
}
