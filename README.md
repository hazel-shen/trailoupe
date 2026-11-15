# Trailoupe

English | [繁體中文](README.zh-TW.md)

Trailoupe is a desktop mobile-forensics tool. It analyses encrypted iPhone backups and
sysdiagnose archives, compares them against public indicators of compromise (STIX2), and
looks for traces that spyware may have left behind.

Trailoupe is intended only for checking **your own** devices.

> **Important:** Trailoupe can only tell you that *no known indicators were found*. It cannot
> prove that a device is safe. Indicator lists only cover publicly documented threats; unknown
> or new spyware will not be detected.

## Status

The project is currently a skeleton; no real parsing logic is implemented yet. Every
subcommand prints "not yet implemented" (尚未實作).

## Layout

| Crate | Kind | Description |
| --- | --- | --- |
| `crates/trailoupe-core` | library | Backup / sysdiagnose parsing, indicator loading, timeline and reports |
| `crates/trailoupe-cli` | binary (`trailoupe`) | Command-line interface |

There is no GUI yet.

## Usage

```sh
cargo run -p trailoupe-cli -- scan-backup <backup-dir>
cargo run -p trailoupe-cli -- scan-sysdiagnose <sysdiagnose.tar.gz>
cargo run -p trailoupe-cli -- update-iocs
```

## Indicator files

This repository does not contain any indicator files (STIX2 or otherwise). Indicator files are
always downloaded at runtime from their original publishers and are subject to each source's
own license terms.

## Development

The Rust version is pinned in `rust-toolchain.toml` (picked up automatically by rustup).

```sh
cargo build --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo audit
cargo deny check
```

`cargo audit` and `cargo deny` are installed with
`cargo install --locked cargo-audit cargo-deny`. `deny.toml` allows only permissive licenses
(plus GPL-3.0-or-later for `trailoupe-cli` itself) and only the crates.io registry.

Rules:

- All input (backups, archives, indicator files) is untrusted: parsing code must not use
  `unwrap` / `expect` / `panic!`, direct indexing, or `unsafe`. The workspace lints set
  `unsafe_code = "forbid"` and deny `unwrap_used` / `expect_used` / `panic` /
  `indexing_slicing` outside of tests (exempted via `clippy.toml`).
- Do not copy or translate line by line any code from MVT (Mobile Verification Toolkit).
- No output may state that a device is safe.

## License

The crates are licensed differently:

- **`trailoupe-core`**: dual-licensed under MIT or Apache-2.0 (`MIT OR Apache-2.0`), at your
  option. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE). The permissive
  license lets other projects (including a future GUI or third-party tools) use the library
  freely.
- **`trailoupe-cli`**: GNU General Public License v3.0 or later (`GPL-3.0-or-later`). See
  [LICENSE-GPL](LICENSE-GPL).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in this project is licensed under the terms of the crate it belongs to, without any additional
terms or conditions.
