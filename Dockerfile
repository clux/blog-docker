FROM scratch
MAINTAINER Eirik Albrigtsen <analsandblaster@gmail.com>

COPY ./target/x86_64-unknown-linux-musl/release/blog /blog
COPY ./posts /posts
COPY ./templates /templates
ENTRYPOINT ["/blog"]
