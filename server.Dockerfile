FROM rust:1.76.0 as build

# create a new empty shell project
RUN USER=root cargo new --bin server
WORKDIR /server

# install cmake
RUN apt update && apt install -y cmake

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src

# build for release
RUN rm -f ./target/release/deps/server*
RUN cargo build --release --bin server

# our final base
FROM rust:1.76.0

# copy the build artifact from the build stage
COPY --from=build /server/target/release/server .

# set the startup command to run your binary
CMD ["./server"]