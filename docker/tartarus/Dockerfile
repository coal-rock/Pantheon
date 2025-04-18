FROM clux/muslrust:latest AS builder 
WORKDIR /app

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl -p tartarus && \
strip /app/target/x86_64-unknown-linux-musl/release/tartarus

FROM scratch AS runner
WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/tartarus .

EXPOSE 8000

CMD ["./tartarus"]
