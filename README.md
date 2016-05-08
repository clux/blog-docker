# blog
[![build status](https://secure.travis-ci.org/clux/blog.svg)](http://travis-ci.org/clux/blog)
[![coverage status](http://img.shields.io/coveralls/clux/blog.svg)](https://coveralls.io/r/clux/blog)
[![image size](https://img.shields.io/imagelayers/image-size/clux/blog/latest.svg)](https://imagelayers.io/?images=clux%2Fblog:latest)

Dockerized rust blog serving content from a mounted posts directory.

## Deploying
Pull docker image and posts directory:

```sh
git clone git@github.com:clux/posts.git
docker pull clux/blog
docker run -p 8000:8000 -t clux/blog
```

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
