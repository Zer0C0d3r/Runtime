# 🚀 runtime

A modern, colorful, and minimal replacement for the classic `uptime` command. Written in Rust for speed, accuracy, and style.

## Why use runtime?

- See your system's uptime, users, and load at a glance
- Beautiful colored output with nerd font icons
- Works in containers and native Linux
- Fast, accurate, and no bloat

## Install

```sh
# Clone and build
git clone https://github.com/Zer0C0d3r/Runtime.git
cd Runtime
cargo build --release
sudo cp target/release/runtime /usr/local/bin/
```

## Usage

```sh
# Minimal dashboard (default)
runtime

# Standard uptime format
runtime --standard

# Pretty format
runtime --pretty

# Raw machine-readable output
runtime --raw

# Show boot time
runtime --since

# Container mode
runtime --container
```

## Example Output

```
────────────────────────────── SYSTEM UPTIME ──────────────────────────────
 Time:      19:36:59 +06:00
 Uptime:    27m 10s
 Boot:      2025-08-09 19:09:49
 Users:     1
 Load:      1.26, 1.66, 1.39
󰆧 Container
──────────────────────────────
```

## Features

- Direct `/proc` access for precise metrics
- Color-coded output (uptime, load, users)
- Nerd font icons for every stat
- Container detection and special output
- Multiple formats: dashboard, standard, pretty, raw, since
- Super fast (written in Rust)

## Container Support

If you're running in a container, runtime will show a container icon and adjust its metrics for your environment.

---

## Classic Uptime Format (for nostalgia)

If you want the old-school look, try:

```sh
runtime --standard
```

Example:

```
20:01:42 up 27 min,  1 user,  load average: 1.26, 1.66, 1.39
```

## Contributing

PRs welcome! Just fork, branch, and submit your changes.

## License

MIT
