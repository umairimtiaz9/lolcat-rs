# 🌈 lolcat-rs

> **The high-performance, vibrant rainbow coloring tool for your terminal.**

`lolcat-rs` is a blazingly fast, memory-safe rewrite of the classic `lolcat` tool in Rust. It brings the full rainbow spectrum to your terminal output with zero runtime dependencies and perfect TrueColor support.

![Rainbow Terminal Example](https://github.com/tehmaze/lolcat/raw/master/lolcat.png)

## ✨ Features

- **🚀 Blazing Fast**: Written in pure Rust, capable of processing gigabytes of text instantly.
- **🎨 TrueColor Support**: Outputs 24-bit RGB colors for the smoothest gradients possible (16 million colors!).
- **🔮 Zero Dependencies**: Statically linked binary. No Python, Ruby, or gems required.
- **💎 Exact Parity**: Mathematical replication of the original sine-wave rainbow algorithm.
- **📄 Binary Safe**: Handles binary files and invalid UTF-8 gracefully (like `cat`).
- **💡 Saturation Boost**: Tuned for slightly more vibrant, brighter colors than the original.

## 📦 Installation

### From Source
```bash
git clone https://github.com/your-username/lolcat-rs.git
cd lolcat-rs
cargo install --path .
```

## 🚀 Usage

Pipe any command into `lolcat-rs`:

```bash
ls -la | lolcat-rs
```

Or read files directly:

```bash
lolcat-rs README.md
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f, --freq <float>` | Rainbow frequency (rainbow tightness) | `0.1` |
| `-s, --spread <float>` | Rainbow spread (gradient speed) | `2.6` |
| `-h, --help` | Show help message | |

## 🔧 Comparison

| Feature | Ruby (Original) | Python (Port) | **lolcat-rs** |
|:---|:---:|:---:|:---:|
| **Startup Time** | Slow (VM) | Medium (VM) | **Instant** |
| **Color Depth** | 256/TrueColor | 256 | **TrueColor (24-bit)** |
| **Memory Usage** | High | Medium | **Minimal** |
| **Dependencies** | Ruby Gems | Python | **None** |

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.

---
*Built with 🦀 and ❤️*
