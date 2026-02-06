# CLAUDE.md — e2e/bun

Visual regression tests for MapLibre GL JS rendering. Compares headless Chrome screenshots against upstream expected PNGs using Playwright + pixelmatch.

## Commands

```bash
bun install                          # Install deps
bunx playwright install chromium     # Install browser
bun run scripts/copy-fixtures.ts     # Copy 1,522 fixtures from submodule
bun run test                         # Run tests (auto-starts server)
bun run test:headed                  # Run with visible browser
bun run src/server.ts                # Start server manually
# Then visit: http://localhost:3900/report        (results viewer)
#             http://localhost:3900/fixture/{id}   (single fixture)
```

## Architecture

```
src/
├── server.ts          # Bun.serve() on port 3900 — fixture pages, assets, report viewer
├── constants.ts       # MapLibre version, skip lists, thresholds
├── localize-urls.ts   # Transforms local:// and mapbox:// URLs to localhost
├── fixture-page.ts    # Generates HTML that loads MapLibre + renders a fixture
├── operations.ts      # JS codegen for test operations (wait, idle, addImage, etc.)
└── report-page.ts     # HTML report page for visual inspection of results
scripts/
└── copy-fixtures.ts   # Copies from submodule → fixtures/, generates manifest.json
tests/
└── render.spec.ts     # Playwright test: iterate fixtures, screenshot, pixelmatch
```

**Flow:** Server reads `style.json` → localizes URLs → generates HTML page → MapLibre renders map → Playwright screenshots canvas → pixelmatch compares against expected PNGs.

## Key Constraints

- **`"type": "commonjs"` in package.json** — required because Playwright runs tests in Node.js (not Bun). Node 22 with native TS strip treats `import.meta` as ESM marker, breaking Playwright's CJS require chain.
- **`__dirname` not `import.meta.dir`** — `import.meta.dir` is Bun-only. Tests run in Node.js via Playwright, so use `__dirname` in `render.spec.ts`.
- **`import.meta.dir` is fine in `src/`** — server files run under Bun directly.
- **Single Playwright worker** — WebGL (SwiftShader) is unstable with parallel contexts.
- **SwiftShader flags** — `--use-gl=angle --use-angle=swiftshader --enable-unsafe-swiftshader` in playwright.config.ts.
- **`#` in fixture IDs** — IDs like `regressions/mapbox-gl-js#1234` must be URL-encoded as `%23`. Server uses `decodeURIComponent()`.

## Thresholds (from upstream)

- **`threshold`** (default 0.1285): pixelmatch perceptual color sensitivity. Lower = stricter.
- **`allowed`** (default 0.00025): max fraction of differing pixels to still pass.
- Per-fixture overrides come from `style.json → metadata.test`.

## Skip List

~80% of fixtures are skipped in `src/constants.ts` because SwiftShader can't faithfully render them (symbols, text, icons, lines, hillshade, heatmap, terrain, etc.). Edit `SKIP_PREFIXES` to include/exclude categories.

## Generated Directories (gitignored)

- `fixtures/` — copied from submodule: `style.json` + `expected*.png` per fixture, plus `assets` symlink and `.cache/` for MapLibre JS/CSS
- `results/` — test output: `summary.json` and `diffs/{id}/actual.png` + `diff.png` for failures

## Updating After Submodule Bump

```bash
cd e2e && git submodule update --remote maplibre-gl-js
cd bun && bun run scripts/copy-fixtures.ts && bun run test
```

## Upstream Reference

Ported from `e2e/maplibre-gl-js/test/integration/render/run_render_tests.ts` and `e2e/maplibre-gl-js/test/integration/lib/localize-urls.ts`.
