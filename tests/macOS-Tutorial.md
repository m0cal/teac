# Testing on macOS with Docker

On macOS, the test suite uses Docker to compile and run Linux aarch64 binaries. This document covers common issues and their solutions.

## Prerequisites

- **Docker Desktop** or **OrbStack** installed and running
- No other tools required (cross-compilers, QEMU, etc.)

## How It Works

1. `std.c` is compiled inside a `gcc:latest` container → `std-linux.o`
2. Generated assembly is linked with `std-linux.o` inside Docker
3. The resulting Linux aarch64 binary runs inside a `debian:bookworm-slim` container

## Common Pitfalls

### 1. Docker Not Running

**Error:**
```
✗ Docker is not running.
Please start Docker Desktop.
```

**Solution:** Start Docker Desktop or OrbStack:
```bash
open -a Docker    # or
open -a OrbStack
```

---

### 2. First Run is Slow

The first `cargo test` downloads Docker images (~1.5GB total):
- `gcc:latest` (~1.4GB) - for compiling and linking
- `debian:bookworm-slim` (~80MB) - for running binaries

Subsequent runs use cached images and complete in ~8 seconds.

---

### 3. Docker Context Issues

If you have multiple Docker contexts (Docker Desktop + OrbStack), ensure the active one is running:

```bash
docker context ls           # See all contexts
docker context use default  # Switch to Docker Desktop
docker context use orbstack # Switch to OrbStack
```

---

### 4. Platform Mismatch (Intel Mac)

On Intel Macs, `--platform linux/arm64` requires QEMU emulation. This is slower and may require setup:

```bash
# Enable multi-platform support
docker run --privileged --rm tonistiigi/binfmt --install all
```

---

### 5. Volume Mount Permissions

If tests fail with permission errors, ensure Docker has access to the workspace:

**Docker Desktop:** Settings → Resources → File Sharing → Add your workspace path

---

### 6. Stale `std-linux.o`

If `std.c` was previously compiled with a different toolchain, delete and rebuild:

```bash
rm tests/std/std-linux.o
cargo test
```

---

### 7. Docker Daemon Socket Errors

**Error:**
```
Cannot connect to the Docker daemon at unix:///var/run/docker.sock
```

**Solutions:**
- Restart Docker Desktop/OrbStack
- Check socket permissions: `sudo chmod 666 /var/run/docker.sock`

---

### 8. Disk Space

Docker images consume ~1.5GB. If tests fail unexpectedly:

```bash
docker system df     # Check Docker disk usage
docker system prune  # Clean up unused data
```

---

### 9. Network Issues During Image Pull

If image downloads fail, check your network connection and retry:

```bash
docker pull gcc:latest
docker pull debian:bookworm-slim
```

---

## Quick Troubleshooting

| Symptom | Likely Cause | Fix |
|---------|--------------|-----|
| "Docker is not running" | Docker daemon not started | `open -a Docker` |
| Very slow first run | Downloading images | Wait, or pre-pull images |
| "File format not recognized" | Stale `std-linux.o` | `rm tests/std/std-linux.o` |
| "Cannot connect to daemon" | Docker crashed/not installed | Restart Docker |
| Permission denied on files | Volume mount issue | Check Docker file sharing settings |

## Running Tests

```bash
# Run all tests
cargo test

# Run a specific test
cargo test int_io

# Run with verbose output on failure
VERBOSE=1 cargo test
```

