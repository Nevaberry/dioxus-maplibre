# E2E Tests

Behavioral Playwright tests for the dioxus-maplibre showcase app.

## Prerequisites

- [Node.js](https://nodejs.org/) 18+ (or [Bun](https://bun.sh/))
- [Dioxus CLI](https://dioxuslabs.com/) (`cargo install dioxus-cli`)
- Chromium browser (installed automatically by Playwright)

## Setup

```bash
cd e2e
npm install
npx playwright install chromium
```

## Running Tests

### Automatic (Playwright starts the dev server)

```bash
npx playwright test
```

Playwright will automatically run `dx serve --port 8080` in `../examples/showcase/` and wait up to 2 minutes for the WASM build to complete. If the server is already running, it reuses it.

### Manual (start server yourself)

In one terminal:

```bash
cd ../examples/showcase
dx serve --port 8080
```

In another terminal:

```bash
npx playwright test
```

### Headed mode (see the browser)

```bash
npx playwright test --headed
```

### Run a specific test file

```bash
npx playwright test tests/basic.spec.ts
```

### Debug mode (step through tests)

```bash
npx playwright test --debug
```

## Test Files

| File | What it tests |
|------|---------------|
| `basic.spec.ts` | Map canvas renders, click events, position tracking |
| `markers.spec.ts` | Marker DOM elements, add/remove buttons |
| `layers.spec.ts` | Layer rendering, visibility toggle, paint property changes |
| `controls.spec.ts` | Navigation, scale, fullscreen controls in DOM |
| `navigation.spec.ts` | flyTo, easeTo, jumpTo, fitBounds, zoom controls |
| `interaction.spec.ts` | Layer click/hover, feature state |
| `events.spec.ts` | Ready event, click event with coordinates |

## Test Approach

These are **behavioral tests** — no pixel comparison or screenshot matching. Assertions check:

- DOM elements exist and are visible (`canvas.maplibregl-canvas`, `.maplibregl-marker`, `.maplibregl-ctrl-*`)
- Event data appears in `[data-testid]` elements
- Button clicks produce expected state changes
- No console errors during operations

## Configuration

See `playwright.config.ts`:

- **Browser**: Chromium headless
- **Viewport**: 1280x720
- **Timeout**: 60s per test, 120s for dev server startup
- **Retries**: 1 (flaky tolerance for WASM load times)
- **Base URL**: `http://localhost:8080`

## Troubleshooting

**Tests timeout waiting for canvas**: The first WASM build takes 1-2 minutes. Subsequent runs reuse the build and are faster. If it still times out, start the server manually first.

**`dx` command not found**: Install with `cargo install dioxus-cli`.

**Chromium not found**: Run `npx playwright install chromium`.
