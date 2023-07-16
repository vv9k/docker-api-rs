#Build convenience

lint:
    cargo clippy --all-targets --all-features -- -D clippy::all
    cargo fmt --all -- --check

test:
    docker pull ubuntu:latest
    cargo test --all-features --all-targets
    cargo test --doc
