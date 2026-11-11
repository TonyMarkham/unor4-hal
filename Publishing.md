# Publishing unor4-hal to crates.io

A run sheet for the owner. Written 2026-11-04, brought up to date
2026-11-11. Checked against cargo 1.99.0 (`cargo publish --help`,
`cargo help publish`), the Cargo book's
[Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html),
and docs.rs's [metadata page](https://docs.rs/about/metadata).

**A publish is permanent.** The Cargo book: "The version can never be
overwritten, and the code cannot be deleted." `cargo yank` only stops new
projects from depending on a version; existing users keep it. crates.io
holds 0.1.0 and 0.1.1 (both yanked) and 0.1.2 (current), read from the
crates.io API on 2026-11-11, so the next one is 0.2.0.

## 1. Check the contents

From the repo root:

```bash
cargo package --list -p unor4-hal --allow-dirty
```

On 2026-11-11 this listed 180 files: `.cargo_vcs_info.json`,
`Cargo.lock`, `Cargo.toml`, `Cargo.toml.orig`, `LICENSE.md`, `README.md`,
`build.rs` and 173 under `src/` (including `src/tests/pins.csv`). The
root README, `Validation.md` and `arduino/` aren't in the crate
(crates.io limit: 10 MB per `.crate`).

## 2. Dry run

The dry run does everything except upload. It checks the manifest, builds
the `.crate` file under `target/package/`, unpacks it and compiles it to
prove it builds from the packaged files alone.

Before committing (allowed to be dirty):

```bash
cargo publish --dry-run -p unor4-hal --allow-dirty
```

After committing (the same check the real publish makes):

```bash
cargo publish --dry-run -p unor4-hal
```

The verify step compiles for the host with default features. To also prove
the packaged crate builds with the optional features:

```bash
cargo publish --dry-run -p unor4-hal --features usb,uart-irq
```

**Pass:** no `error:` lines, and it ends with:

```text
    Packaged 180 files, 633.1KiB (186.4KiB compressed)
   Verifying unor4-hal v0.2.0 (D:\git\unor4-hal\crates\unor4-hal)
   Compiling unor4-hal v0.2.0 (D:\git\unor4-hal\target\package\unor4-hal-0.2.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in …
   Uploading unor4-hal v0.2.0 (D:\git\unor4-hal\crates\unor4-hal)
warning: aborting upload due to dry run
```

That is the 2026-11-11 run with `--allow-dirty`, by the agent under that
day's grant, with default features and with `usb,uart-irq`. The run
without `--allow-dirty` needs the commit first and hasn't been made. The
same day, the host matrix (`just check`, `just clippy`, `just test`), the
firmware builds and the bench (33/33 over three sessions, `Validation.md`)
passed on the same tree.

To check what docs.rs will build, from the repo root:

```bash
cargo doc -p unor4-hal --no-deps --features usb,uart-irq
```

It passed at `RUSTDOCFLAGS=-D warnings` on 2026-11-11, with `usb` and
`uart::UartIrq` in `target/doc/unor4_hal/`.

If a check fails, fix the cause and run it again. Never use `--no-verify`
to get past an error.

## 3. Log in (only if needed)

You published 0.1.2 from this machine, so a token may already be stored.
If the publish reports an authentication error:

1. Sign in at <https://crates.io> with GitHub. Your account email must be
   verified (Account Settings).
2. Create a token at <https://crates.io/settings/tokens> and copy it. It's
   shown only once.
3. Run the command below and paste the token. Cargo stores it in
   `~/.cargo/credentials.toml`. If the token ever leaks, revoke it on
   crates.io.

```bash
cargo login
```

## 4. Publish

Clean tree, version `0.2.0` in the root `Cargo.toml`, CHANGELOG `[0.2.0]`
dated with the publish day:

```bash
cargo publish -p unor4-hal
```

Then check:

- <https://crates.io/crates/unor4-hal> shows 0.2.0, with the crate README.
- <https://docs.rs/unor4-hal/0.2.0> finishes building (it can take a while)
  and shows the `usb` module and `uart::UartIrq`.
- Tag the release to match the CHANGELOG link
  (`releases/tag/v0.2.0`):

```bash
git tag v0.2.0
```

```bash
git push origin v0.2.0
```

## 5. If something is wrong after publishing

A published version can't be changed. Publish a fixed 0.2.1. Then, if
0.2.0 must not be used:

```bash
cargo yank unor4-hal --version 0.2.0
```

`cargo yank unor4-hal --version 0.2.0 --undo` reverses a yank. Yanking
never removes code. If a secret was published, revoke the secret itself.
