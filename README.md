# blog
[![build status](https://secure.travis-ci.org/clux/blog.svg)](http://travis-ci.org/clux/blog)
[![coverage status](http://img.shields.io/coveralls/clux/blog.svg)](https://coveralls.io/r/clux/blog)
[![image size](https://imagelayers.io/badge/clux/blog:latest.svg)](https://imagelayers.io/?images=clux/blog:latest)

Dockerised rust blog serving content from a [directory of markdown posts](https://github.com/clux/posts).

## [documentation](http://clux.github.io/blog)

## Deploying
Pull docker image and run:

```sh
docker pull clux/blog
docker run -p 80:8000 -t clux/blog
```

Run on CoreOS remotely with `fleetctl start cluxblog.service`.

## Developing
Clone this repo, the dependent post repo, then build and link.

```sh
git clone git@github.com:clux/blog.git && cd blog
git clone git@github.com:clux/posts.git
cargo build
ln -sf $PWD/target/debug/blog /usr/local/bin/blog
```

Iterate and verify:

```sh
blog
cargo fmt
cargo test
cargo doc
```
