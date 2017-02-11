# blog
[![build status](https://secure.travis-ci.org/clux/blog.svg)](http://travis-ci.org/clux/blog)
[![docker pulls](https://img.shields.io/docker/pulls/clux/blog.svg)](
https://hub.docker.com/r/clux/blog/)
[![docker image info](https://images.microbadger.com/badges/image/clux/blog.svg)](http://microbadger.com/images/clux/blog)
[![docker tag](https://images.microbadger.com/badges/version/clux/blog.svg)](https://hub.docker.com/r/clux/blog/tags/)

Dockerised rust blog serving content from a [directory of markdown posts](https://github.com/clux/posts).

## [documentation](http://clux.github.io/blog)

## Deploying
Pull docker image and run:

```sh
docker pull clux/blog
docker run -p 80:8000 -t clux/blog
```

The production build of this blog is entirely self-contained (`FROM scratch` - statically linked using [muslrust](https://github.com/clux/muslrust)), and uses no database.

## Developing
Clone this repo, the dependent post repo, then build and link.

```sh
git clone git@github.com:clux/blog.git && cd blog
git clone git@github.com:clux/posts.git
rustup override set nightly
rustup update
cargo build
cargo run
```

Iterate and verify:

```sh
cargo run
cargo fmt
cargo test
cargo doc
```
