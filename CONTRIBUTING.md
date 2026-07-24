# Contributing to dioxus-maplibre

## Development Setup

```bash
# Clone the repo
git clone https://github.com/Nevaberry/dioxus-maplibre
cd dioxus-maplibre

# The repository pins Rust 1.97.1 in rust-toolchain.toml.
rustup show active-toolchain

# Install the matching Dioxus CLI (for running examples)
curl -fsSL https://dioxus.dev/install.sh | bash -s -- v0.7.9
```

## Running Tests

### Unit Tests (Rust)

```bash
# Run all unit tests
cargo test --locked --all-features

# Run with output
cargo test --locked --all-features -- --nocapture

# Run specific test
cargo test latlng_new
```

### Showcase App (Manual Testing)

The showcase app demonstrates all features and is useful for manual testing.

```bash
cd examples/showcase
dx serve --web --port 8080 --locked
```

Then open http://localhost:8080 in your browser.

### E2E Tests (Playwright)

Browser smoke, API, and interaction tests. Requires [Bun](https://bun.sh/).

```bash
# Install Bun (if not installed)
curl -fsSL https://bun.sh/install | bash

# Install Playwright and browsers
cd e2e
bun install --frozen-lockfile
bunx playwright install chromium
```

#### Installing System Dependencies for Browsers

Playwright requires system-level dependencies for Chromium. If you see errors like "Host system is missing dependencies to run browsers", install them:

**Debian/Ubuntu:**
```bash
sudo bunx playwright install-deps chromium
```

#### Running E2E Tests

**Important:** Use `bun run test`, NOT `bun test` (they are different commands).

```bash
cd e2e

# Run all Chromium tests
bun run test

# Run with visible browser
bun run test:headed

# Run with interactive UI
bun run test:ui

# View test report
bun run report
```

#### Running Specific Tests

```bash
# Run the configured Chromium project
bunx playwright test --project=chromium

# Run specific test file
bunx playwright test tests/showcase.spec.ts
```

#### Troubleshooting E2E Tests

**"Host system is missing dependencies to run browsers"**
- Install browser dependencies: `sudo bunx playwright install-deps chromium`

**"Playwright Test did not expect test.describe() to be called here"**
- You used `bun test` instead of `bun run test`. The correct command is `bun run test`.

**Tests time out while starting the showcase**
- Run `cd examples/showcase && dx bundle --web --release --debug-symbols=false --out-dir dist --locked` to verify the web build independently.

## Code Quality

```bash
# Format code
cargo fmt --all

# Lint
cargo clippy --locked --all-targets --all-features -- -D warnings

# Check compilation
cargo check --locked --all-targets --all-features
```

## Project Structure

```
src/                    # Library code (published to crates.io)
tests/                  # Unit tests
examples/showcase/      # Demo app for testing
e2e/                    # Playwright E2E tests (optional)
```

## Making Changes

### Branch Workflow

The repository has three long-lived branches:

- `development` is the integration branch for active development. Create
  short-lived feature and fix branches from it and open pull requests back to
  `development`.
- `main` is the default branch and the source for preview/staging deployments.
  Promote a tested `development` state with a `development` -> `main` pull
  request.
- `production` is the source for production deployments and crate releases.
  Promote an approved staging state with a `main` -> `production` pull request.

Do not develop directly on `main` or `production`.

### Pull Request Checklist

1. Add or update focused tests
2. Run `cargo test --locked --all-features` to verify
3. Test manually with the showcase app
4. Run the strict formatting, Clippy, and rustdoc commands above
5. Submit a pull request to the appropriate branch described above

## Testing Checklist

Before submitting a PR:

- [ ] `cargo test --locked --all-features` passes
- [ ] `cargo fmt --all --check` passes
- [ ] strict Clippy and rustdoc checks pass
- [ ] `cd examples/showcase && cargo check --target wasm32-unknown-unknown` passes
- [ ] Showcase app works (manual check)
- [ ] E2E tests pass: `cd e2e && bun run test` (or at minimum: `bunx playwright test --project=chromium`)
