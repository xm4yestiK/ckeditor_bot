CKEditor Bot 🚀
A Rust-based utility to bypass CKEditor’s copy-paste restrictions.
Automatically types user-provided text into any active application using hotkeys.

---

Features

* Hotkey Control

  * F8 → Types provided text into active window
  * F10 → Exits program
* Interactive Input

  * Accepts multiline text
  * Use !!END to stop input cleanly
* Structured logging via tracing
* ASCII banner via figlet-rs
* Error handling with anyhow

---

Installation

Clone and build:

```
git clone https://github.com/xm4yestiK/ckeditor-bot.git
cd ckeditor-bot
cargo build --release
```

Binary output:

```
target/release/ckeditor-bot
```

Requirements

* Rust toolchain ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))
* Tested on:

  * Windows 10/11
  * Linux (X11)
  * macOS (limited)

---

Usage
Run:

```
./ckeditor-bot
```

Steps:

1. Enter text (max 1000 words).

   * Type !!END to stop early.
2. Focus target app (e.g., CKEditor in browser).
3. Press F8 to type.
4. Press F10 to exit.

Example:

```
🤖 Enter the text you want me to type (max 1000 words):
Hello CKEditor!

⚠️ Input ended by Type !!END

✅ Bot ready — focus the target app and press F8 to type. Press F10 to quit.
```

Note: No trailing newline added after typing.

---

Development

Formatting & linting:

```
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Tests:

```
cargo test
```

Debug run:

```
cargo run
```

---

Project Structure

```
src/
 └─ main.rs
Cargo.toml
LICENSE
README.md
```

---

Roadmap

* [ ] Cross-platform (Linux/Wayland, macOS)
* [ ] Configurable hotkeys
* [ ] Config file (YAML/TOML)
* [ ] Pre-built binaries in Releases

---

Contributing
Open issues or PRs at [https://github.com/xm4yestiK/ckeditor-bot/issues](https://github.com/xm4yestiK/ckeditor-bot/issues)
Use Conventional Commits for messages.

---

License
MIT © 2025 m4yestiK
See LICENSE file for details.

---

Support
⭐ Star this project on GitHub if useful!
