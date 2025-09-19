Berikut versi **diperbarui** dokumentasi README kamu agar lebih akurat dengan perubahan terbaru:

* Menghapus newline ekstra di akhir teks
* Ctrl+Z / Cmd+Z log bersih
* Menjaga input multiline tetap utuh

````markdown
# CKEditor Bot 🚀

A Rust-based utility to **bypass CKEditor’s copy-paste restrictions**.  
This bot automatically types user-provided text into the target application using hotkeys.

---

## ✨ Features

- ⚡ **Hotkey Control**
  - `F8` → Automatically types the provided text into the active application
  - `F10` → Exits the program
- 📝 **Interactive input**: Provide text once, then trigger typing on demand
  - Supports **multiline input**
  - Detects **Ctrl+Z / Cmd+Z** to end input without polluting text
- 📊 **Structured logging**: Powered by [`tracing`](https://docs.rs/tracing)
- 🎨 **ASCII banner**: Rendered with [`figlet-rs`](https://crates.io/crates/figlet-rs)
- 🔒 **Robust error handling**: Built with [`anyhow`](https://crates.io/crates/anyhow)

---

## 📦 Installation

### Clone & Build

```bash
git clone https://github.com/xm4yestiK/ckeditor-bot.git
cd ckeditor-bot
cargo build --release
````

The binary will be available at:

```
target/release/ckeditor-bot
```

### Requirements

* **Rust toolchain**: [Install Rust](https://www.rust-lang.org/tools/install)
* Tested on:

  * ✅ Windows 10/11
  * ✅ Linux (X11)
  * ✅ macOS (limited support)

---

## ▶ Usage

Run the program:

```bash
./ckeditor-bot
```

1. Enter the text you want the bot to type (up to 1000 words).

   * Press **Ctrl+Z** (Windows) / **Cmd+Z** (Mac) to end input early.
2. Switch focus to the target application (e.g., CKEditor in a browser).
3. Press **F8** → The text will be typed automatically.
4. Press **F10** → The program will exit.

Example output:

```
🤖 Enter the text you want me to type (max 1000 words):
Hello CKEditor!

⚠️ Input ended by Ctrl+Z / Cmd+Z

✅ Bot ready — focus the target app and press F8 to type. Press F10 to quit.
```

> Note: The bot no longer adds an extra newline at the end of typed text.

---

## 🔧 Development

### Code Formatting & Linting

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

### Run Tests

```bash
cargo test
```

### Run in Debug Mode

```bash
cargo run
```

---

## 📂 Project Structure

```
src/
 └─ main.rs        # Entrypoint
Cargo.toml         # Dependencies & metadata
LICENSE            # MIT license
README.md          # Project documentation
```

---

## 🚀 Roadmap

* [ ] Cross-platform testing (Linux/Wayland, macOS)
* [ ] Configurable hotkeys
* [ ] Config file support (YAML/TOML)
* [ ] Pre-built binaries in GitHub Releases

---

## 🤝 Contributing

Contributions are welcome! 🙌
Please open an [issue](https://github.com/xm4yestiK/ckeditor-bot/issues) or submit a pull request.
Commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/) standard.

---

## 📜 License

MIT License © 2025 [m4yestiK](https://github.com/xm4yestiK)

See the [LICENSE](LICENSE) file for details.

---

## ⭐ Support

If you find this project useful, please consider giving it a **star** on GitHub! 🌟
