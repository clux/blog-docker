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
use blog::DataBase;

use std::path::{Path, PathBuf};

#[get("/")]
fn index(db: State<DataBase>) -> Template {
    Template::render("index", &db.clone())
}

#[get("/<slug>")]
fn entry(db: State<DataBase>, slug: &str) -> Result<Template, Failure> {
    if let Some(post) = db.posts.get(slug) {
        Ok(Template::render("entry", &post))
    } else {
        Err(Failure(Status::NotFound))
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("posts/").join(file)).ok()
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
        .mount("/", routes![index, entry])
        .mount("/static/", routes![files])
        .launch();
}
