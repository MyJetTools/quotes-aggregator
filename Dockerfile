FROM rust
COPY . . 
ENTRYPOINT ["./target/release/quotes-mixer"]