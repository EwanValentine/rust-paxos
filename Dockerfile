FROM rust:alpine as build

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp ./target/release/paxos /build-out/


FROM alpine

COPY --from=build /build-out/paxos ./

ENTRYPOINT [ "./paxos", "master" ]