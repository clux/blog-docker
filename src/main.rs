#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate blog;

#[macro_use]
extern crate log;

use rocket::response::NamedFile;
use rocket::State;

use blog::PostMap;
//use std::collections::BTreeMap;
use std::process;
use std::path::{Path, PathBuf};


#[get("/")]
fn index(db: State<PostMap>) -> String {
    // TODO: use index template with the whole db
    db.get("2006-08-09-vault-of-therayne").unwrap().clone().html
}

#[get("/<slug>")]
fn entry(db: State<PostMap>, slug: &str) -> String {
    // http://localhost:8000/e/2013-03-20-colemak
    if let Some(post) = db.get(slug) {
        // TODO: load template from post.clone() instead
        post.clone().html
    } else {
        "Not found".into() // TODO: 404
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
            warn!("Failed to load posts: {}", e);
            process::exit(1);
        })
        .unwrap();
    if db.len() == 0 {
        warn!("No posts found in posts/ - clone posts repo first");
        process::exit(1);
    }
    println!("Loaded {} posts", db.len());
    // TODO: load templates in ./templates (currently .hbs - can be changed)

    rocket::ignite()
        .manage(db)
        .mount("/", routes![index])
        .mount("/e", routes![entry])
        .mount("/static", routes![files])
        .launch();
}
