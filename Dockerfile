FROM scratch

COPY ./target/x86_64-unknown-linux-musl/debug/blog /blog
COPY ./posts /posts
COPY ./templates /templates
EXPOSE 80
WORKDIR /
ENTRYPOINT ["/blog"]
