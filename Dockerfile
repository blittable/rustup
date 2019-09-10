FROM ekidd/rust-musl-builder AS builder

RUN cargo install mdbook --vers 0.2.1

EXPOSE 3000
EXPOSE 3001

FROM alpine:latest

COPY --from=builder \
    /home/rust/.cargo/bin/mdbook \
    /usr/local/bin/


WORKDIR /rustup
RUN ls -la

COPY . . 
RUN ls -la

ENTRYPOINT ["/usr/local/bin/mdbook"]
CMD ["--help"]
