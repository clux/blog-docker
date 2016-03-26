#[macro_use]
extern crate clap;
extern crate pencil;

extern crate blog;
use blog::*;

use pencil::{Pencil, Request, PencilResult};
use clap::{Arg, App};

use std::process;

fn main() {
    let args = App::new("blog")
        .version(crate_version!())
        .about("clux's blog engine")
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
