# E2E Tests

This directory uses MapLibre GL JS's official render test infrastructure via a git submodule.

## Setup

```bash
# First time setup
cd e2e
npm run setup

# Or manually:
cd maplibre-gl-js
npm install
npm run build-dev
```

## Running Tests

```bash
# Run all render tests (requires xvfb on Linux)
npm test

# Run without xvfb (if you have a display)
npm run test:no-xvfb

# Run specific test(s)
npm run test:filter -- circle-radius
npm run test:filter -- "fill-*"
```

## How It Works

MapLibre's render test infrastructure:
- **Puppeteer** for browser automation
- **Port 2900**: Asset server for test fixtures
- **Port 2901**: MVT server for vector tiles
- **WebGL readPixels()**: Captures map renders (not browser screenshots)
- **188+ test categories**: Each with `style.json` + `expected.png`

Tests compare rendered output against expected images with pixel-level diffing.

## Updating Fixtures

When MapLibre updates their tests:

```bash
# Update submodule to latest
npm run update-submodule

# Rebuild and re-run
npm run build-maplibre
npm test
```

## Test Results

Results are written to `maplibre-gl-js/test/integration/render/results.html`.

## Directory Structure

```
e2e/
├── maplibre-gl-js/          # Git submodule (MapLibre GL JS repo)
│   ├── test/integration/
│   │   ├── render/          # Render test runner
│   │   └── assets/          # Test fixtures and expected images
│   └── ...
├── package.json             # Convenience scripts
└── README.md                # This file
```
