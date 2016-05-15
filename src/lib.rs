//! This is the rust doc for the `blog` *library* - what the `blog` *binary*
//! depends on to get posts, and convert markdown to html properly.
//!
//! For documentation on using the blog binary, see
//! [the main readme](https://github.com/clux/blog/blob/rust/README.md)
//!
//! ## Strategy
//! All markdown files are read at startup by globbing for README.md in
//! the posts/ directory. These files are converted to HTML and stored
//! in a map. It's also linked up with a `data.json` and all links are
//! converted from relative links to something with a /static prefix.
//!
//! Summaries are created by somewhat naively trying to find the first
//! paragraph that is not a raw heading or an image.
//!
//! ## Serving
//! The main blog engine would be in the binary, and that's the only
//! thing that is depending on `iron` and its ecosystem. It has thus far
//! seemed the most stable, and it has the most feature support of
//! normal web app stuff like templates, databases, routers and loggers.
//!
//! ## Docker
//! End goal is to have a 5 MB docker image that contains everything.
//! This is achieved almost entirely by compiling it with musl, and by
//! building a docker image 'FROM scratch' with the static binary copied.
//!

extern crate glob;
extern crate hoedown;
extern crate rustc_serialize;
extern crate regex;
#[macro_use]
extern crate log;

pub use errors::BlogResult;
pub use data::{Post, MetaData, PostMap, load_posts};

mod errors;
mod data;
