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

# Install Playwright and browsers
cd e2e
bun install
bunx playwright install
```

#### Installing System Dependencies for Browsers

Playwright requires system-level dependencies for WebKit/Safari. If you see errors like "Host system is missing dependencies to run browsers", install them:

**Debian/Ubuntu:**
```bash
sudo bunx playwright install-deps
```

Or install specific packages:
```bash
sudo apt-get install libwoff2dec1 libenchant-2-2 libmanette-0.2-0
```

#### Running E2E Tests

**Important:** Use `bun run test`, NOT `bun test` (they are different commands).

```bash
cd e2e

# Run all tests (all browsers)
bun run test

# Run with visible browser
bun run test:headed

# Run with interactive UI
bun run test:ui

# Update screenshots after intentional changes
bun run test:update-snapshots

# View test report
bun run report
```

#### Running Tests for Specific Browsers

If you have issues with certain browsers (e.g., WebKit/Safari dependencies), you can run tests for specific browsers only:

```bash
# Run only Chromium tests (recommended for quick verification)
bunx playwright test --project=chromium

# Run only Firefox tests
bunx playwright test --project=firefox

# Run only desktop browsers (skip mobile)
bunx playwright test --project=chromium --project=firefox --project=webkit

# Run specific test file
bunx playwright test tests/map-render.spec.ts
```

#### Troubleshooting E2E Tests

**"Host system is missing dependencies to run browsers"**
- Install browser dependencies: `sudo npx playwright install-deps`
- Or skip problematic browsers: `bunx playwright test --project=chromium`

**"Playwright Test did not expect test.describe() to be called here"**
- You used `bun test` instead of `bun run test`. The correct command is `bun run test`.

**Tests timeout or fail on first run**
- The showcase app needs time to build. Try running again - the server may not have been ready.

**Visual regression tests fail**
- If you intentionally changed the UI, update snapshots: `bun run test:update-snapshots`

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
  A push to
  `production` runs release-plz, which prepares a release pull request with the
  next version and changelog. Merging that generated pull request creates the
  `v<version>` tag and GitHub release and publishes the crate to crates.io.

Do not develop directly on `main` or `production`. Timestamped
`release-plz-*` branches are temporary automation branches, not development
branches.

### Pull Request Checklist

1. Write tests first (TDD encouraged)
2. Run `cargo test` to verify
3. Test manually with the showcase app
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request to the appropriate branch described above

## Testing Checklist

Before submitting a PR:

- [ ] `cargo test` passes
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy` has no warnings
- [ ] Showcase app works (manual check)
- [ ] E2E tests pass: `cd e2e && bun run test` (or at minimum: `bunx playwright test --project=chromium`)
