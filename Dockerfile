FROM rust

ENV SOCKET_ADDRESS="0.0.0.0:8080"
ENV SUPABASE_ENDPOINT="https://quepskrrpovzwydvfezs.supabase.co/rest/v1"
ENV SUPABASE_AUTH_ENDPOINT="https://quepskrrpovzwydvfezs.supabase.co/auth/v1"

WORKDIR /budi
COPY . .
RUN git submodule init && git submodule update
RUN apt update && apt-get install -y cmake
RUN cargo build --release
CMD ["./target/release/server"]
EXPOSE 8080
