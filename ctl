#!/bin/bash

stop_blog() {
  docker kill blog
  docker rm blog
}

if [[ "$1" == "build" ]]; then
  docker run --rm -v $PWD:/volume -w /volume -t clux/muslrust cargo build --release
  docker build -t clux/blog .
elif [[ "$1" == "run" ]]; then
  trap stop_blog SIGINT
  docker run -p 8000:8000 --name=blog -t clux/blog
fi
