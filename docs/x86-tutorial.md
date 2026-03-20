# Running Tests on x86

The TeaLang compiler targets AArch64 (ARM64). On AArch64 hosts — Linux AArch64 and macOS Apple Silicon — tests compile, link, and run natively with no extra setup.

On **x86/x86_64** hosts, extra tools are needed to assemble, link, and execute the AArch64 binaries. This document covers the two supported x86 configurations.

## Platform Support Overview

| Host | How it works | Extra tools needed |
|------|-------------|-------------------|
| Linux AArch64 | Native | `gcc` (usually pre-installed) |
| macOS AArch64 (Apple Silicon) | Native | Xcode Command Line Tools |
| **Linux x86/x86_64** | **Cross-compile + QEMU** | `gcc-aarch64-linux-gnu`, `qemu-user` |
| **macOS x86_64 (Intel)** | **Docker** | Docker Desktop or OrbStack |

---

## Linux x86/x86_64 — Cross-compile + QEMU

This is the approach used by CI (`ubuntu-latest`).

### Prerequisites

Install the AArch64 cross-compiler and QEMU user-mode emulator:

```bash
sudo apt install gcc-aarch64-linux-gnu qemu-user
```

### How It Works

1. `std.c` is compiled with `aarch64-linux-gnu-gcc -c` → `std-linux.o`
2. The compiler emits Linux AArch64 assembly (auto-detected from host)
3. Assembly is linked with `std-linux.o` using `aarch64-linux-gnu-gcc -static`
4. The resulting static binary runs under `qemu-aarch64`

### Running Tests

```bash
cargo test              # run all tests
cargo test int_io       # run a specific test
VERBOSE=1 cargo test    # show expected vs actual on mismatch
```

### Troubleshooting

| Error | Cause | Fix |
|-------|-------|-----|
| `aarch64-linux-gnu-gcc not found` | Cross-compiler not installed | `sudo apt install gcc-aarch64-linux-gnu` |
| `qemu-aarch64 not found` | QEMU not installed | `sudo apt install qemu-user` |
| Stale `std-linux.o` | Compiled with wrong toolchain | `rm tests/std/std-linux.o && cargo test` |

---

## macOS x86_64 (Intel) — Docker

On Intel Macs, Docker provides an AArch64 Linux environment for linking and running.

### Prerequisites

- **Docker Desktop** ([download](https://www.docker.com/products/docker-desktop)) or **OrbStack** ([download](https://orbstack.dev/))
- Make sure Docker is **running** before you run tests

### How It Works

1. `std.c` is compiled inside a `gcc:latest` container (linux/arm64) → `std-linux.o`
2. The compiler emits Linux AArch64 assembly (forced via `--target linux`)
3. Assembly is linked with `std-linux.o` inside Docker using `gcc -static`
4. The resulting binary runs inside a `debian:bookworm-slim` container (linux/arm64)

Docker's built-in QEMU emulation handles arm64 execution transparently on the x86_64 host.

### Running Tests

```bash
# Start Docker first
open -a Docker    # or: open -a OrbStack

# Then run tests
cargo test
cargo test int_io
VERBOSE=1 cargo test
```

### First Run

The first `cargo test` downloads two Docker images (~1.5 GB total):

| Image | Size | Purpose |
|-------|------|---------|
| `gcc:latest` | ~1.4 GB | Compiling and linking |
| `debian:bookworm-slim` | ~80 MB | Running binaries |

Subsequent runs use cached images.

### Troubleshooting

| Error | Cause | Fix |
|-------|-------|-----|
| `Docker is not running` | Daemon not started | `open -a Docker` |
| `Cannot connect to the Docker daemon` | Docker crashed or not installed | Restart Docker Desktop / OrbStack |
| `File format not recognized` | Stale `std-linux.o` | `rm tests/std/std-linux.o && cargo test` |
| Permission denied on files | Volume mount issue | Docker Desktop → Settings → Resources → File Sharing → add workspace |
| Very slow first run | Downloading images | Wait, or pre-pull: `docker pull gcc:latest && docker pull debian:bookworm-slim` |

If you have multiple Docker contexts (Docker Desktop + OrbStack), make sure the active one is running:

```bash
docker context ls            # list contexts
docker context use default   # switch to Docker Desktop
docker context use orbstack  # switch to OrbStack
```

Intel Macs use QEMU under the hood for `--platform linux/arm64`. If you see platform errors:

```bash
docker run --privileged --rm tonistiigi/binfmt --install all
```
