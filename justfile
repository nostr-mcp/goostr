run *args:
    RUST_LOG=debug cargo run {{args}}
start:
    ./target/release/goostr