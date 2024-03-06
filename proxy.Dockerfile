FROM rust:1.76.0 as build

# create a new empty shell project
RUN USER=root cargo new --bin proxy
WORKDIR /proxy

# install cmake
RUN apt update && apt install -y cmake

COPY ./proxy.conf.yml ./proxy.conf.yml

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src

# build for release
RUN rm -f ./target/release/deps/proxy*
RUN cargo build --release --bin proxy

# our final base
FROM rust:1.76.0

# copy the build artifact from the build stage
COPY --from=build /proxy/target/release/proxy .

COPY --from=build /proxy/proxy.conf.yml .

# set the startup command to run your binary
CMD ["./proxy", "-c", "proxy.conf.yml"]