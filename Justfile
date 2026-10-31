_default:
    just --list

test-watch args='':
    watchexec -e rs -- cargo nextest run {{args}}

check-watch:
    watchexec -e rs -- cargo check --workspace --all-targets

clippy:
    cargo clippy --workspace --all-targets -- -D warnings

clippy-fix:
    cargo clippy --workspace --all-targets --fix --allow-dirty --allow-staged

watch year day:
    watchexec -e rs -- cargo run -- -y {{year}} -d {{day}} -p ./aoc/src/aoc{{year}}/input/day{{day}}
