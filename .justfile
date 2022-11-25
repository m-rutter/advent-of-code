_default:
    just --list

install-tools:
    cargo install cargo-nextest --locked
    cargo install cargo-watch

test-watch:
    cargo watch -x 'nextest run'

check-watch:
    cargo watch -x check 