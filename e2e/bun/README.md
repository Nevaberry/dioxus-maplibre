# Visual Regression Tests

Validates vanilla MapLibre GL JS rendering against upstream expected images using Playwright + pixelmatch. This establishes a known-good baseline -- if vanilla MapLibre matches its own expected PNGs, we can later compare Dioxus-wrapped rendering against the same baselines.

## Quick Start

```bash
cd e2e/bun
bun install
bunx playwright install chromium
bun run scripts/copy-fixtures.ts   # Copy 1,522 fixtures from submodule
bun run test                        # Run tests (auto-starts server)
```

## Architecture

```
e2e/bun/
├── scripts/copy-fixtures.ts    # Copies from submodule, generates manifest
├── src/
│   ├── constants.ts            # CDN version, skip lists, thresholds
│   ├── localize-urls.ts        # Transforms local:// and mapbox:// URLs
│   ├── fixture-page.ts         # HTML page generator per fixture
│   ├── operations.ts           # JS codegen for test operations
│   └── server.ts               # Bun.serve() on port 3900
├── tests/
│   └── render.spec.ts          # Playwright test suite
├── fixtures/                   # GENERATED (gitignored)
│   ├── manifest.json
│   ├── .cache/                 # Locally cached MapLibre JS/CSS
│   ├── assets -> symlink       # Submodule assets (tiles, sprites, glyphs)
│   └── {category}/{test}/      # style.json + expected*.png
└── results/                    # GENERATED (gitignored)
    ├── summary.json
    └── diffs/{category}/{test}/actual.png + diff.png
```

**Flow:** The server reads `style.json`, localizes URLs, generates an HTML page that loads MapLibre from a local cache, creates a map, runs operations, and signals ready. Playwright screenshots the canvas and compares against expected PNGs with pixelmatch.

## Comparing Results

### summary.json

After a test run, `results/summary.json` has every fixture's outcome:

```json
{
  "total": 1522,
  "pass": 234,
  "fail": 12,
  "skip": 1272,
  "error": 4,
  "results": [
    { "id": "circle-radius/literal", "status": "pass", "difference": 0, "allowed": 0.00025 },
    { "id": "background-color/transition", "status": "fail", "difference": 1.0, "allowed": 0.00025 },
    ...
  ]
}
```

### Quick summary from the command line

```bash
# Counts by status
cat results/summary.json | jq '{pass: .pass, fail: .fail, skip: .skip, error: .error, total: .total}'

# List all failures with their diff scores
cat results/summary.json | jq '.results[] | select(.status == "fail") | {id, difference, allowed}'

# List all errors
cat results/summary.json | jq '.results[] | select(.status == "error") | {id, error}'
```

### Visual diff inspection

Failed fixtures get actual + diff PNGs saved to `results/diffs/`:

```
results/diffs/
  background-color/transition/
    actual.png     # What headless Chrome rendered
    diff.png       # Red pixels = differences from expected
```

Compare side-by-side with the expected image from the fixtures:

```bash
# Open expected vs actual vs diff for a specific fixture
open fixtures/background-color/transition/expected.png
open results/diffs/background-color/transition/actual.png
open results/diffs/background-color/transition/diff.png
```

### Comparing upstream vs our rendering

The upstream expected images live in the submodule at:
```
e2e/maplibre-gl-js/test/integration/render/tests/{category}/{test}/expected*.png
```

Our copies (identical) are at:
```
e2e/bun/fixtures/{category}/{test}/expected*.png
```

Our actual rendered output (after running tests) is at:
```
e2e/bun/results/diffs/{category}/{test}/actual.png
```

So for any fixture, you can compare three images:
1. **Expected** (upstream baseline): `fixtures/{id}/expected.png`
2. **Actual** (our headless render): `results/diffs/{id}/actual.png`
3. **Diff** (pixel differences): `results/diffs/{id}/diff.png`

Note: `actual.png` and `diff.png` are only saved for *failed* fixtures. Passing fixtures matched the expected image within the allowed threshold.

## Understanding Results

### Thresholds

Each fixture defines two thresholds (from upstream `metadata.test`):
- **`threshold`** (default 0.1285): Perceptual color sensitivity for pixelmatch. Lower = more sensitive.
- **`allowed`** (default 0.00025): Maximum fraction of differing pixels. A fixture passes if `diffPixels / totalPixels <= allowed`.

### Expected Failures

These are known to fail in headless Chrome with SwiftShader:

| Pattern | Reason |
|---------|--------|
| `*/transition` | Time-dependent animations don't settle deterministically |
| `*/@2x`, `*/2x-*` | Retina/HiDPI pixel ratio differences in SwiftShader |
| `basic-v9/*` | Full vector tile rendering has SwiftShader-specific rasterization |

### Skipped Categories

~80% of fixtures are skipped because they use GPU features that SwiftShader renders differently from real GPUs. See `src/constants.ts` for the full list. Key categories:

- **Symbol/Text/Icon**: Glyph rasterization, collision detection, label placement
- **Line**: Dash patterns, joins, caps differ in software rendering
- **Hillshade/Heatmap/Terrain**: Require half-float textures or compute shaders
- **Regressions**: Edge-case tests that often use skipped feature types

## Development

### Run a single fixture in the browser

```bash
bun run src/server.ts
# Visit: http://localhost:3900/fixture/circle-radius/literal
```

### Filter to specific fixtures

Edit the skip list in `src/constants.ts` to include/exclude categories, then re-run.

### Update fixtures after submodule bump

```bash
cd e2e && git submodule update --remote maplibre-gl-js
cd bun && bun run scripts/copy-fixtures.ts
bun run test
```

### Upstream reference

The test infrastructure is ported from:
- `e2e/maplibre-gl-js/test/integration/render/run_render_tests.ts` -- fixture discovery, map creation, operations, image comparison
- `e2e/maplibre-gl-js/test/integration/lib/localize-urls.ts` -- URL transformation
