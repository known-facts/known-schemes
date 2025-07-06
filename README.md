# Known Schemes

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.81%2B-blue)](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0/)
[![Package](https://img.shields.io/crates/v/known-schemes)](https://crates.io/crates/known-schemes)
[![Documentation](https://docs.rs/known-schemes/badge.svg)](https://docs.rs/known-schemes/)

Well-known URI/IRI schemes for Rust.

## ✨ Features

- Exports an enum for all well-known URI/IRI schemes (protocols).
- Integrates seamlessly with popular crates like [Serde] and [Clap].
- Integrates seamlessly with all URI/IRI crates.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.81+

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add known-schemes
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
known-schemes = "0.2"
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
known-schemes = { version = "0.2", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the library

```rust
use known_schemes;
```

## 📚 Reference

https://docs.rs/known-schemes/

## 👨‍💻 Development

```bash
git clone https://github.com/known-facts/known-schemes.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/known-facts/known-schemes&text=Known%20Schemes)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/known-facts/known-schemes&title=Known%20Schemes)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/known-facts/known-schemes&t=Known%20Schemes)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/known-facts/known-schemes)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/known-facts/known-schemes)

[Serde]: https://crates.io/crates/serde
[Clap]: https://crates.io/crates/clap
[feature flags]: https://github.com/known-facts/known-schemes/blob/master/lib/known-schemes/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html
