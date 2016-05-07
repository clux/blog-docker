FROM scratch

COPY ./target/x86_64-unknown-linux-musl/release/blog /blog

EXPOSE 8000
WORKDIR /

CMD ["/blog"]
