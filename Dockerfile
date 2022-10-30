FROM rust

ENV SOCKET_ADDRESS="0.0.0.0:8080"
ENV SUPABASE_ENDPOINT="https://quepskrrpovzwydvfezs.supabase.co/rest/v1"
ENV SUPABASE_API_KEY="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InF1ZXBza3JycG92end5ZHZmZXpzIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTY1Nzg1MjUxMCwiZXhwIjoxOTczNDI4NTEwfQ.uB7pqybhZSKUufL10F2fMbHaSOdBIDkvL6W-TXKEMRo"

WORKDIR /budi
COPY . .
RUN git submodule init && apt update && apt-get install -y cmake && cargo build --release
CMD ["./target/release/server"]
EXPOSE 8080
