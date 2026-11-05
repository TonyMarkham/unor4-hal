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
    just doc

# Broken doc links are rustdoc lints: `cargo check` never reports them.
[windows]
doc:
    $env:RUSTDOCFLAGS='-D warnings'; cargo doc -p unor4-hal --no-deps
    $env:RUSTDOCFLAGS='-D warnings'; cargo doc -p unor4-hal --no-deps --features uart-irq
    $env:RUSTDOCFLAGS='-D warnings'; cargo doc -p unor4-hal --no-deps --features usb,uart-irq
    $env:RUSTDOCFLAGS='-D warnings'; cargo doc -p unor4-hal --no-deps --no-default-features --features minima
    $env:RUSTDOCFLAGS='-D warnings'; cargo doc -p unor4-hal --no-deps --no-default-features --features minima,uart-irq
    $env:RUSTDOCFLAGS='-D warnings'; cargo doc -p unor4-hal --no-deps --no-default-features --features minima,usb,uart-irq
    $env:RUSTDOCFLAGS='-D warnings'; cargo doc -p validation --no-deps

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
