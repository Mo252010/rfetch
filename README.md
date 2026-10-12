# rfetch

A fast, live system info TUI with a built-in config editor — written in Rust.

`rfetch` renders a compact, live-updating view of your CPU, memory, swap, GPU,
storage, and more as an **inline panel** — it never takes over your terminal.
It comes with your OS/distro's ASCII logo and a full-screen config editor in
the same binary.

```
███████████████████████████    ┌──────────────────────────────────────────────┐
██           ██          ██    │ carhooked@omarchyPC                          │
██  ███████████    ████  ██    │ ───────────────────────────────────────────── │
██  ██               ██  ██    │ CPU      Intel Core i5-6300U (4 cores) @ 2.9GHz│
██  ██               ██  ██    │          [████████████████░░░░░░░░]  41.1%    │
██████               ██  ██    │ cores    ▄▄▃▂                                │
██  ██               ██  ██    │ Memory   [████████████░░░░░░░░░░] 5.8/15.4 GiB│
██  ██               ██  ██    │ GPU      Skylake-U GT2 [HD Graphics 520]      │
██  ██               ██  ██    │ Disk     / [btrfs] 56.7/58.8 GiB (97%)        │
██  ███████████████████  ██    └───────────────────────────────────────────────┘
██           ██          ██
███████████████  ██████████
```

## Features

- **Live, inline view** — renders at your cursor, sized to its content; the
  terminal is never taken over.
- **Real-time stats** — CPU usage, per-core sparkline + frequency, memory,
  swap, load average, GPU (usage/temp/freq/VRAM), storage with per-disk layout,
  uptime, kernel, OS, host.
- **Distro logos** — auto-detects your OS/distro from `/etc/os-release`
  (Omarchy, Arch, Ubuntu, Fedora, and 20+ more), with Tux/macOS/Windows
  fallbacks. Override with `RFETCH_ASCII`.
- **Built-in config editor** — press `e` for a full-screen TUI editor
  (General / Theme / Modules tabs). No external config tools.
- **Single binary** — no runtime app, no Electron, no web server. Compiles to
  ~1 MB.
- **Cross-platform** — Linux, macOS, and Windows (GPU stats via NVML on
  NVIDIA, sysfs on Linux, lspci names).

## Install

### macOS (Homebrew)

```sh
brew tap Mo252010/rfetch
brew install rfetch
```

Builds from source on both Intel and Apple Silicon Macs. Prebuilt universal
macOS binaries are attached to every [GitHub release](https://github.com/Mo252010/rfetch/releases).

### Prebuilt binaries

Download from the [releases page](https://github.com/Mo252010/rfetch/releases) —
`linux-x86_64`, `macos-universal` (Intel + Apple Silicon), `windows-x86_64`:

```sh
curl -Lo rfetch.tar.gz https://github.com/Mo252010/rfetch/releases/latest/download/rfetch-macos-universal.tar.gz
tar -xzf rfetch.tar.gz && ./rfetch
```

Binary is also `cargo binstall`-able: `cargo binstall rfetch`.

### From source

Requires Rust 1.85+:

```sh
cargo install --path .
git clone https://github.com/Mo252010/rfetch.git
cd rfetch
cargo build --release
./target/release/rfetch
```

### Arch / AUR

An [AUR PKGBUILD](packaging/aur/PKGBUILD) is included (submit/publish pending).

## Usage

```
rfetch [OPTIONS]

  -g, --gui        Open the config editor first
  -o, --once       Print one snapshot as plain text and exit
  -p, --path       Print the config file path and exit
  -h, --help       Print this help and exit
  -V, --version    Print version and exit
```

### Live view keys

| Key | Action |
| --- | --- |
| `e` | Open the config editor |
| `q` / `Esc` | Quit |
| `Ctrl-C` | Quit |

### Editor keys

| Key | Action |
| --- | --- |
| `1` `2` `3` / `Tab` | Switch section (General / Theme / Modules) |
| `Up` / `Down` | Select field or module |
| `Left` / `Right` | Change value / cycle palette / bar width |
| `Space` / `Enter` | Toggle or activate |
| `a` / `d` | Add / delete module |
| `J` / `K` | Move module down / up |
| `n` / `m` | Edit module label / mounts |
| `s` | Save config |
| `q` / `Esc` | Quit editor |

## Configuration

Config lives at `~/.config/rfetch/config.toml` and is auto-written with
defaults on first run. Tweak it by hand or with the built-in editor (`e`).

```toml
[general]
refresh_ms = 1000
title = ""
border = true
rounded = true
show_logo = true

[theme]
accent = "cyan"
border = "dark_gray"
label = "green"
value = "white"
title = "red"

[[modules]]
kind = "cpu"
label = "CPU"
bar_width = 28
enabled = true
```

`bar_width` controls each bar's length; storage modules can filter by mount
via `mounts`.

## Environment

| Variable | Meaning |
| --- | --- |
| `RFETCH_ASCII` | Override the detected logo by OS or distro name, e.g. `omarchy`, `arch`, `ubuntu`, `fedora`, `macos`, `windows`. |

## Overview

```
src/
├── main.rs       CLI dispatch, terminal lifecycle, inline viewport
├── collector.rs  system snapshot (CPU, memory, disks, load, …)
├── gpu.rs        GPU reader (NVML + Linux sysfs/lspci)
├── config.rs     config model + load/save
├── theme.rs      color parsing/palette
├── logo.rs       distro/OS ASCII logos + detection
└── ui/
    ├── display.rs  inline live view renderer
    ├── editor.rs   full-screen config editor
    └── widgets.rs  gauges, sparklines, formatting
```

## License

MIT