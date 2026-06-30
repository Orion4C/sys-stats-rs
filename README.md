# sys-stats-rs

![CI](https://github.com/Orion4C/sys-stats-rs/actions/workflows/ci.yml/badge.svg)

A command-line process tracker written in Rust. It samples running processes over a
short window, keeps a running average of each process's CPU, memory, and disk usage,
and prints the ones that cross configurable thresholds

## Features

- Per-process tracking of CPU, memory, and disk read/write, averaged across a sampling window.
- Tracks multiple live processes that share a name (e.g. several `chrome` workers) as
  distinct instances rather than collapsing them into one.
- Threshold filtering: only processes that exceed a minimum on at least one metric, and
  that were observed for a minimum share of the window, are listed.
- Fully configurable from the command line (thresholds, sampling interval, iteration count).

## Build

Requires a stable Rust toolchain (2024 edition).

```sh
git clone https://github.com/Orion4C/sys-stats-rs.git
cd sys-stats-rs
cargo build --release
```

## Usage

Run with defaults:

```sh
cargo run --release
```

Pass options after `--` so Cargo forwards them to the program:

```sh
cargo run --release -- --min-cpu 80 --min-mem 200 --iterations 50
```

Or run the compiled binary directly:

```sh
./target/release/sys-stats-rs --min-cpu 80 --interval-ms 500
```

See every option with `--help`:

```sh
cargo run --release -- --help
```

| Flag | Meaning | Default |
|------|---------|---------|
| `--min-cpu` | Minimum average CPU usage (%) to list a process | `100` |
| `--min-mem` | Minimum average memory (MB) | `500` |
| `--min-disk-read` | Minimum average disk read (MB) | `200` |
| `--min-disk-write` | Minimum average disk write (MB) | `200` |
| `--min-uptime` | Minimum runtime as a % of the observation window | `0` |
| `--interval-ms` | Sampling interval in milliseconds | sysinfo minimum |
| `--iterations` | Number of sampling iterations | `20` |

## How it works

**Metrics are driven by a single enum.** Each tracked statistic is a `Usage` variant, and
the code iterates over the variants to build and update every metric channel. Adding a new
statistic is a one-variant-plus-one-match-arm change rather than a new code path copied four
times.

**Process identity, not process name.** Several live processes can share a name, so each is
tracked as its own snapshot keyed by PID rather than folded together by name. When a process
disappears from the OS it is marked with an end time during a reap pass, so a later process
that reuses the same PID starts a fresh snapshot instead of inheriting stale averages.

**Running averages, computed incrementally.** Each metric keeps a count and a running mean
and folds in one new sample per tick, rather than retaining every reading. This keeps memory
flat regardless of how long the tool runs.

**CPU sampling respects the platform minimum.** CPU usage on `sysinfo` is a delta between two
refreshes, so the sampling interval is always clamped up to `MINIMUM_CPU_UPDATE_INTERVAL`;
sampling faster than that floor produces unreliable CPU figures.

**SI units, not binary.** Byte conversions use 1000-based MB/GB (so "500 MB" means 500,000,000
bytes), matching how `--min-mem` is interpreted on the command line.

## Testing

```sh
cargo test
```

Unit tests cover the pure logic: the running-average computation, the byte-unit
conversions, and the parameter dispatch and interval-clamping rules.
