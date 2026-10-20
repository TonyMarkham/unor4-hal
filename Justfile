[windows]
set shell := ["pwsh.exe", "-NoLogo", "-Command"]

[windows]
test:
    cargo check
    cargo check --features uart-irq
    cargo check --no-default-features --features minima
    cargo check --no-default-features --features minima,uart-irq
    cargo test
    cargo test --features uart-irq
    cargo test --no-default-features --features minima
    cargo test --no-default-features --features minima,uart-irq
