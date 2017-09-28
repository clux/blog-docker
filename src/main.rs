#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

extern crate blog;

use rocket::response::{NamedFile, Redirect, Failure};
use rocket::http::Status;
use rocket::State;
use rocket_contrib::Template;

use blog::DataBase;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// main /

#[get("/")]
fn root() -> Redirect {
    Redirect::to("/e")
}


// /e

#[get("/")]
fn index(db: State<DataBase>) -> Template {
    Template::render("index", &db.clone())
}

#[get("/<slug>")]
fn entry(db: State<DataBase>, slug: String) -> Result<Template, Failure> {
    if let Some(post) = db.posts.get(&slug) {
        Ok(Template::render("entry", &post))
    } else {
        Err(Failure(Status::NotFound))
    }
}

// /static

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("posts").join(file)).ok()
}

#[get("/about")]
fn about() -> Template {
    // apparently you have to feed Template something
    let noop : BTreeMap<String, String> = BTreeMap::new();
    Template::render("about", &noop)
}

#[get("/<file..>")]
fn resources(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("resources").join(file)).ok()
}

fn main() {
    use std::process;
    let db = blog::load_posts()
        .map_err(|e| {
            println!("Failed to load posts: {}", e);
            for e in e.iter().skip(1) {
                println!("Caused by: {}", e);
            }
            process::exit(1);
        })
        .unwrap();
    if db.posts.is_empty() {
        println!("No posts found in posts/ - clone posts repo first");
        process::exit(1);
    }
    println!("Loaded {} posts", db.posts.len());

    rocket::ignite()
        .manage(db)
        .mount("/static/", routes![files])
        .mount("/dist/", routes![resources])
        .mount("/", routes![root, about]) // misc resources under root
        .mount("/e/", routes![index, entry])
        .attach(Template::fairing())
        .launch();
}
