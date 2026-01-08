# Contributing to dioxus-maplibre

## Development Setup

```bash
# Clone the repo
git clone https://github.com/Nevaberry/dioxus-maplibre
cd dioxus-maplibre

# Install Dioxus CLI (for running examples)
cargo install dioxus-cli
```

## Running Tests

### Unit Tests (Rust)

```bash
# Run all unit tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test latlng_new
```

### Showcase App (Manual Testing)

The showcase app demonstrates all features and is useful for manual testing.

```bash
cd examples/showcase
dx serve --port 8080
```

Then open http://localhost:8080 in your browser.

### E2E Tests (Playwright)

Visual regression and interaction tests. Requires [Bun](https://bun.sh/).

```bash
# Install Bun (if not installed)
curl -fsSL https://bun.sh/install | bash

# Install Playwright
cd e2e
bun install
bunx playwright install chromium

# Run tests (starts showcase automatically)
bun test

# Run with visible browser
bun test:headed

# Update screenshots after intentional changes
bun test:update-snapshots

# View test report
bun report
```

## Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Check compilation
cargo check
```

## Project Structure

```
src/                    # Library code (published to crates.io)
tests/                  # Unit tests
examples/showcase/      # Demo app for testing
e2e/                    # Playwright E2E tests (optional)
```

## Making Changes

1. Write tests first (TDD encouraged)
2. Run `cargo test` to verify
3. Test manually with the showcase app
4. Run `cargo fmt` and `cargo clippy`
5. Submit PR

## Testing Checklist

Before submitting a PR:

- [ ] `cargo test` passes
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy` has no warnings
- [ ] Showcase app works (manual check)
- [ ] E2E tests pass (if you have Bun installed)
