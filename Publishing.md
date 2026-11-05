# Publishing unor4-hal to crates.io

A run sheet for the owner. You run every command. Checked 2026-11-04
against cargo 1.99.0 (`cargo publish --help`, `cargo help publish`), the
Cargo book's
[Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html),
and docs.rs's [metadata page](https://docs.rs/about/metadata).

**A publish is permanent.** The Cargo book: "The version can never be
overwritten, and the code cannot be deleted." `cargo yank` only stops new
projects from depending on a version; existing users keep it. crates.io
already holds 0.1.0 and 0.1.1 (both yanked) and 0.1.2 (current), so the
next one is 0.2.0.

## 1. Fix these before 0.2.0

Found on 2026-11-04 by running `cargo package --list -p unor4-hal`:

| # | Problem | Status |
| --- | --- | --- |
| 1 | **No licence file in the package.** Cargo packages only the crate's folder (`crates/unor4-hal/`), so the root `LICENSE.md` doesn't ship. `license = "MIT"` in the manifest names the licence but doesn't include its text. | **Open, owner's choice:** copy `LICENSE.md` into `crates/unor4-hal/`, or set `license-file = "../../LICENSE.md"` in its `Cargo.toml`. For the second, whether a path outside the crate folder gets packaged is unverified: check it with step 2's list. The crate README's licence link is fixed either way (it now points at the GitHub file). |
| 2 | **Crate README out of date:** `unor4-hal = "0.1"`, no ELC or step generator. | **Fixed 2026-11-04** (agent): `"0.2"`, `elc` and `step` added. |
| 3 | **docs.rs shows only the default features.** With no `[package.metadata.docs.rs]` table, the `usb` and `uart-irq` APIs never appear. docs.rs also builds five targets by default (x86-64 Linux, macOS ARM, Windows x86-64, ARM64 Linux, Windows x86). | **Open:** add the table below to `crates/unor4-hal/Cargo.toml`. The docs build without warnings for the host with these features (`cargo doc -p unor4-hal --no-deps --features usb,uart-irq`, 2026-11-04). |
| 4 | **Slice 11 isn't closed.** The README record, CHANGELOG entry and cart-pole sign-off are still to do (`HANDOFF.md`). | **Open, agent's work.** Publish 0.2.0 after it lands. |
| 5 | **The working tree is dirty.** Without `--allow-dirty`, cargo refuses to package uncommitted changes. | **Open, owner commits** (AGENTS.md rule 6). Commit before the real publish; `--allow-dirty` is for dry runs only. |

The table for item 3:

```toml
[package.metadata.docs.rs]
features = ["usb", "uart-irq"]
targets  = []   # build only the default target (x86_64-unknown-linux-gnu)
```

docs.rs builds with `--no-default-features` only if you ask, so `wifi` (the
default) stays on and the docs show the WiFi pin map. `all-features = true`
won't work, because `wifi` and `minima` together are a `compile_error!`.

The Cargo book also lists `homepage` among the fields to fill in. The
manifest has `repository` but no `homepage`; crates.io links the
repository either way.

## 2. Check the contents

From the repo root:

```bash
cargo package --list -p unor4-hal --allow-dirty
```

On 2026-11-04 this listed 99 files: `.cargo_vcs_info.json`, `Cargo.lock`,
`Cargo.toml`, `Cargo.toml.orig`, `README.md`, `build.rs` and `src/**`
(including `src/tests/pins.csv`). After fix 1 you should also see
`LICENSE.md`. The root README, `Research/` and `arduino/` aren't in the
crate (crates.io limit: 10 MB per `.crate`).

## 3. Dry run

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

**Pass:** no `error:` lines, and it stops without uploading. (I haven't
run the dry run myself. Running builds is your step under AGENTS.md
rule 2, so I'm not quoting its exact output.) If it fails, fix the cause
and run it again. Never use `--no-verify` to get past an error.

## 4. Log in (only if needed)

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

## 5. Publish

Clean tree, version `0.2.0` in the root `Cargo.toml`, CHANGELOG `[0.2.0]`
dated (not "Unreleased"):

```bash
cargo publish -p unor4-hal
```

Then check:

- <https://crates.io/crates/unor4-hal> shows 0.2.0, with the crate README.
- <https://docs.rs/unor4-hal/0.2.0> finishes building (it can take a while)
  and shows the `usb` / `uart-irq` items if you added the item 3 table.
- Tag the release to match the CHANGELOG link
  (`releases/tag/v0.2.0`):

```bash
git tag v0.2.0
```

```bash
git push origin v0.2.0
```

## 6. If something is wrong after publishing

A published version can't be changed. Publish a fixed 0.2.1. Then, if
0.2.0 must not be used:

```bash
cargo yank unor4-hal --version 0.2.0
```

`cargo yank unor4-hal --version 0.2.0 --undo` reverses a yank. Yanking
never removes code. If a secret was published, revoke the secret itself.
