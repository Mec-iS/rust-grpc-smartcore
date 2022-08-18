FROM rust:1.61-slim
RUN apt-get update && apt-get install -y libprotobuf-dev protobuf-compiler
WORKDIR /usr/src/app
COPY . .
RUN cargo build --path .
CMD ["grpc-smartcore-server"]

EXPOSE 5005