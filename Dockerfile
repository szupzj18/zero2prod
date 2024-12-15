# use the stable release
FROM rust:1.77.2 AS builder

# switch to app dir
WORKDIR /app

# install lld and clang
# lld is a linker from LLVM project
# clang is a C compiler from LLVM project
RUN apt update && apt install lld clang -y

# copy the source code
COPY . .
ENV SQLX_OFFLINE true 
# build  the release version
RUN cargo build --release

FROM rust:1.77.2 AS runtime

WORKDIR /app
# copy the binary from the build env
# to the runtime env
COPY --from=builder /app/target/release/zero2prod zero2prod
# configuration to the runtime env
COPY configuration configuration
# config the environment for production hosting
ENV APP_ENV production
# run the app
ENTRYPOINT [ "./target/release/zero2prod" ]
