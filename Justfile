[windows]
set shell := ["pwsh.exe", "-NoLogo", "-Command"]

[windows]
check:
    cargo check
    cargo check --features uart-irq
    cargo check --features usb,uart-irq
    cargo check --no-default-features --features minima
    cargo check --no-default-features --features minima,uart-irq
    cargo check --no-default-features --features minima,usb,uart-irq
    cargo check -p validation

[windows]
clippy:
    cargo clippy -- -D warnings
    cargo clippy --features uart-irq -- -D warnings
    cargo clippy --features usb,uart-irq -- -D warnings
    cargo clippy --no-default-features --features minima -- -D warnings
    cargo clippy --no-default-features --features minima,uart-irq -- -D warnings
    cargo clippy --no-default-features --features minima,usb,uart-irq -- -D warnings
    cargo clippy -p validation -- -D warnings

[windows]
test: check
    cargo test
    cargo test --features uart-irq
    cargo test --features usb,uart-irq
    cargo test --no-default-features --features minima
    cargo test --no-default-features --features minima,uart-irq
    cargo test --no-default-features --features minima,usb,uart-irq
    cargo test -p validation
