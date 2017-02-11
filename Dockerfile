FROM scratch
COPY ./target/x86_64-unknown-linux-musl/release/blog /blog
COPY ./Rocket.toml /Rocket.toml
COPY ./posts /posts
COPY ./templates /templates
ENV ROCKET_ENV production
ENTRYPOINT ["/blog"]
