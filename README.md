# blog
[![build status](https://secure.travis-ci.org/clux/blog.svg)](http://travis-ci.org/clux/blog)
[![coverage status](http://img.shields.io/coveralls/clux/blog.svg)](https://coveralls.io/r/clux/blog)

Trying out making a blog in rust.

## Usage
Clone, build, run.

```sh
git clone git@github.com:clux/blog.git && cd blog
git clone git@github.com:clux/posts.git
cargo build --release
./target/release/blog
```

TODO: src change -> cargo build, posts change -> restart app

## Developing

```sh
ln -sf $PWD/target/debug/blog /usr/local/bin/blog
cargo build
blog # verify functionality
cargo fmt
git commit -a
```
