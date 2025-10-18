run *args:
    RUST_LOG=debug cargo run {{args}}

start:
    ./target/release/goostr

install:
    just run install

uninstall:
    just run uninstall