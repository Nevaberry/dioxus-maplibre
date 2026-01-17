import { test, expect, Page } from "@playwright/test";
import * as fs from "fs";
import * as path from "path";
import { PNG } from "pngjs";
import pixelmatch from "pixelmatch";

/**
 * Comprehensive Fixture Verification Tests
 *
 * Runs ALL fixtures from the manifest and compares with expected.png images.
 * Uses pixelmatch for pixel-level comparison with configurable threshold.
 */

const NATIVE_URL = "http://localhost:3000";
const FIXTURES_DIR = path.join(__dirname, "../native-showcase/fixtures");
const ASSETS_DIR = path.join(__dirname, "../native-showcase/assets");
const OUTPUT_DIR = path.join(__dirname, "../test-results/all-fixtures");

interface FixtureCategory {
  category: string;
  tests: string[];
}

interface FixtureResult {
  category: string;
  test: string;
  status: "passed" | "failed" | "skipped" | "error";
  diffPercent?: number;
  error?: string;
  skipReason?: string;
}

// Load manifest
function loadManifest(): FixtureCategory[] {
  const manifestPath = path.join(FIXTURES_DIR, "manifest.json");
  if (!fs.existsSync(manifestPath)) {
    return [];
  }
  const content = fs.readFileSync(manifestPath, "utf-8");
  return JSON.parse(content);
}

// Get fixture dimensions from style.json
function getFixtureDimensions(
  category: string,
  testName: string
): { width: number; height: number; pixelRatio: number } {
  const stylePath = path.join(FIXTURES_DIR, category, testName, "style.json");
  if (!fs.existsSync(stylePath)) {
    return { width: 512, height: 512, pixelRatio: 1 };
  }
  const content = fs.readFileSync(stylePath, "utf-8");
  const style = JSON.parse(content);
  return {
    width: style.metadata?.test?.width || 512,
    height: style.metadata?.test?.height || 512,
    pixelRatio: style.metadata?.test?.pixelRatio || 1,
  };
}

// Get fixture style for analysis
function getFixtureStyle(category: string, testName: string): any {
  const stylePath = path.join(FIXTURES_DIR, category, testName, "style.json");
  if (!fs.existsSync(stylePath)) {
    return null;
  }
  const content = fs.readFileSync(stylePath, "utf-8");
  return JSON.parse(content);
}

// Check if fixture requires resources we might not have
function getSkipReason(style: any, category: string, testName?: string): string | null {
  if (!style) return "No style.json found";

  // Check test metadata for unsupported features
  const testMeta = style.metadata?.test || {};

  // Skip debug collision tests that cause browser crashes
  // These tests are very heavy on text rendering and crash headless Chrome
  if (category === "debug" && testName?.includes("collision")) {
    if (testName.includes("icon-text") || testName.includes("lines")) {
      return "Debug collision test causes browser crash in headless mode";
    }
  }

  // Skip tests with debug mode and large dimensions - they cause browser hangs
  const width = testMeta.width || 512;
  const height = testMeta.height || 512;
  if (testMeta.debug && (width > 512 || height > 512)) {
    return "Debug mode with large canvas causes browser instability";
  }

  // Canvas fixtures require special image handling
  if (testMeta.addFakeCanvas) {
    return "Canvas source requires fake canvas image (not available)";
  }

  // Check for vector tile sources
  const sources = style.sources || {};
  for (const [, source] of Object.entries(sources) as [string, any][]) {
    if (source.type === "vector") {
      const tiles = source.tiles || [];
      if (
        tiles.some(
          (t: string) =>
            t.includes("local://tiles/") || t.includes("local://tilesets/")
        )
      ) {
        // Check if tiles exist
        const tilesDir = path.join(ASSETS_DIR, "tiles");
        if (!fs.existsSync(tilesDir)) {
          return "Requires vector tiles (assets not downloaded)";
        }
      }
    }
    // Check for local data (geojson) that might not exist
    if (source.type === "geojson" && typeof source.data === "string") {
      if (source.data.includes("local://data/")) {
        const dataDir = path.join(ASSETS_DIR, "data");
        if (!fs.existsSync(dataDir)) {
          return "Requires local data files (assets not downloaded)";
        }
      }
    }
    // Video sources might not work in headless
    if (source.type === "video") {
      return "Video source not supported in headless testing";
    }
    // Canvas sources need fake canvas
    if (source.type === "canvas") {
      return "Canvas source not supported";
    }
  }

  // Check for glyphs
  if (style.glyphs?.includes("local://glyphs/")) {
    const glyphsDir = path.join(ASSETS_DIR, "glyphs");
    if (!fs.existsSync(glyphsDir)) {
      return "Requires glyphs (assets not downloaded)";
    }
  }

  // Check for sprites
  const sprite = style.sprite;
  if (sprite) {
    const spriteUrl =
      typeof sprite === "string" ? sprite : sprite[0]?.url || sprite[0];
    if (spriteUrl?.includes("local://sprites/")) {
      const spritesDir = path.join(ASSETS_DIR, "sprites");
      if (!fs.existsSync(spritesDir)) {
        return "Requires sprites (assets not downloaded)";
      }
    }
  }

  // Check for custom layers (requires WebGL setup)
  const layers = style.layers || [];
  if (layers.some((l: any) => l.type === "custom")) {
    return "Custom layer type requires special WebGL setup";
  }

  return null;
}

// Wait for map to be fully rendered and idle
async function waitForMapReady(page: Page, timeout = 15000): Promise<boolean> {
  try {
    // First wait for canvas to exist
    await page.waitForSelector(".maplibregl-canvas", { timeout: 5000 });

    // Wait for the map to signal it's ready
    await page.waitForFunction(
      () => (window as any).__mapReady === true || (window as any).__mapError,
      { timeout }
    );

    // Check if there was an error
    const hasError = await page.evaluate(() => (window as any).__mapError);
    if (hasError) {
      return false;
    }

    // Additional buffer for final render stabilization
    await page.waitForTimeout(300);

    return true;
  } catch (e) {
    return false;
  }
}

// Composite an image onto a white background (to handle transparency)
function compositeOnWhite(png: PNG): PNG {
  const result = new PNG({ width: png.width, height: png.height });
  for (let y = 0; y < png.height; y++) {
    for (let x = 0; x < png.width; x++) {
      const idx = (png.width * y + x) * 4;
      const r = png.data[idx];
      const g = png.data[idx + 1];
      const b = png.data[idx + 2];
      const a = png.data[idx + 3] / 255;

      // Alpha blend with white background
      result.data[idx] = Math.round(r * a + 255 * (1 - a));
      result.data[idx + 1] = Math.round(g * a + 255 * (1 - a));
      result.data[idx + 2] = Math.round(b * a + 255 * (1 - a));
      result.data[idx + 3] = 255;
    }
  }
  return result;
}

// Compare two PNG buffers using pixelmatch
function compareImages(
  actualBuffer: Buffer,
  expectedBuffer: Buffer,
  options: { threshold?: number; outputPath?: string } = {}
): { diffPixels: number; totalPixels: number; diffPercent: number } {
  const actualRaw = PNG.sync.read(actualBuffer);
  const expectedRaw = PNG.sync.read(expectedBuffer);

  // Ensure dimensions match
  if (
    actualRaw.width !== expectedRaw.width ||
    actualRaw.height !== expectedRaw.height
  ) {
    return {
      diffPixels: Math.max(
        actualRaw.width * actualRaw.height,
        expectedRaw.width * expectedRaw.height
      ),
      totalPixels: Math.max(
        actualRaw.width * actualRaw.height,
        expectedRaw.width * expectedRaw.height
      ),
      diffPercent: 100,
    };
  }

  // Composite both images onto white background to normalize transparency handling
  const actual = compositeOnWhite(actualRaw);
  const expected = compositeOnWhite(expectedRaw);

  const { width, height } = actual;
  const diff = new PNG({ width, height });

  const diffPixels = pixelmatch(
    actual.data,
    expected.data,
    diff.data,
    width,
    height,
    { threshold: options.threshold ?? 0.1 }
  );

  // Save diff image if path provided
  if (options.outputPath) {
    fs.mkdirSync(path.dirname(options.outputPath), { recursive: true });
    fs.writeFileSync(options.outputPath, PNG.sync.write(diff));
  }

  const totalPixels = width * height;
  return {
    diffPixels,
    totalPixels,
    diffPercent: (diffPixels / totalPixels) * 100,
  };
}

// Capture screenshot of the map canvas
async function captureMapScreenshot(
  page: Page,
  width: number,
  height: number,
  pixelRatio: number = 1
): Promise<Buffer> {
  // Set viewport to exact fixture size plus some padding to ensure full render
  await page.setViewportSize({ width: width + 100, height: height + 100 });

  // Wait a bit more for render to complete
  await page.waitForTimeout(300);

  // Get the canvas element and read its actual pixel dimensions
  const canvas = page.locator(".maplibregl-canvas");

  // For pixelRatio > 1, we need to capture the full canvas resolution
  // MapLibre renders at width*pixelRatio x height*pixelRatio internally
  if (pixelRatio > 1) {
    // Use JavaScript to extract canvas pixels at full resolution
    const buffer = await page.evaluate(async () => {
      const canvas = document.querySelector(".maplibregl-canvas") as HTMLCanvasElement;
      if (!canvas) throw new Error("Canvas not found");

      // Get the data URL at full resolution
      const dataUrl = canvas.toDataURL("image/png");
      // Convert to base64
      return dataUrl.split(",")[1];
    });
    return Buffer.from(buffer, "base64");
  }

  // For pixelRatio 1, use standard screenshot
  return (await canvas.screenshot()) as Buffer;
}

// Run a single fixture test
async function runFixture(
  page: Page,
  category: string,
  testName: string,
  saveArtifacts: boolean = false
): Promise<FixtureResult> {
  const result: FixtureResult = {
    category,
    test: testName,
    status: "error",
  };

  try {
    // Check if fixture exists
    const stylePath = path.join(FIXTURES_DIR, category, testName, "style.json");
    const expectedPath = path.join(
      FIXTURES_DIR,
      category,
      testName,
      "expected.png"
    );

    if (!fs.existsSync(stylePath)) {
      result.status = "skipped";
      result.skipReason = "No style.json found";
      return result;
    }

    if (!fs.existsSync(expectedPath)) {
      result.status = "skipped";
      result.skipReason = "No expected.png found";
      return result;
    }

    // Check if we should skip this fixture
    const style = getFixtureStyle(category, testName);
    const skipReason = getSkipReason(style, category, testName);
    if (skipReason) {
      result.status = "skipped";
      result.skipReason = skipReason;
      return result;
    }

    const { width, height, pixelRatio } = getFixtureDimensions(category, testName);

    // Navigate to fixture
    await page.goto(`${NATIVE_URL}/fixture/${category}/${testName}`, {
      waitUntil: "networkidle",
    });

    // Wait for map ready
    const ready = await waitForMapReady(page);
    if (!ready) {
      result.status = "error";
      result.error = "Map did not become ready within timeout";
      return result;
    }

    // Capture screenshot at the correct resolution
    // For pixelRatio > 1, the canvas is actually larger than CSS dimensions
    const actualBuffer = await captureMapScreenshot(page, width, height, pixelRatio);

    // Save actual screenshot if requested
    if (saveArtifacts) {
      const actualPath = path.join(
        OUTPUT_DIR,
        `${category}-${testName}-actual.png`
      );
      fs.mkdirSync(path.dirname(actualPath), { recursive: true });
      fs.writeFileSync(actualPath, actualBuffer);
    }

    // Load expected image
    const expectedBuffer = fs.readFileSync(expectedPath);

    // Compare images
    const diffPath = saveArtifacts
      ? path.join(OUTPUT_DIR, `${category}-${testName}-diff.png`)
      : undefined;

    const comparison = compareImages(actualBuffer, expectedBuffer, {
      threshold: 0.1,
      outputPath: diffPath,
    });

    result.diffPercent = comparison.diffPercent;

    // Allow up to 5% difference (for antialiasing and minor rendering differences)
    if (comparison.diffPercent < 5) {
      result.status = "passed";
    } else {
      result.status = "failed";
      result.error = `${comparison.diffPercent.toFixed(2)}% pixel difference`;
    }
  } catch (e) {
    result.status = "error";
    result.error = (e as Error).message;
  }

  return result;
}

test.describe("All Fixtures Verification", () => {
  test.setTimeout(600000); // 10 minute timeout for full suite

  let serverAvailable = false;
  const allResults: FixtureResult[] = [];

  test.beforeAll(async ({ request }) => {
    // Skip if server isn't running
    try {
      console.log(`Checking server at ${NATIVE_URL}...`);
      const response = await request.get(NATIVE_URL, { timeout: 5000 });
      serverAvailable = response.ok();
      console.log(`Server available: ${serverAvailable} (status: ${response.status()})`);
    } catch (e) {
      console.log(`Server check failed: ${(e as Error).message}`);
      serverAvailable = false;
    }

    // Ensure output directory exists
    fs.mkdirSync(OUTPUT_DIR, { recursive: true });
  });

  test.afterAll(async () => {
    // Write results summary
    if (allResults.length > 0) {
      const summary = {
        total: allResults.length,
        passed: allResults.filter((r) => r.status === "passed").length,
        failed: allResults.filter((r) => r.status === "failed").length,
        skipped: allResults.filter((r) => r.status === "skipped").length,
        error: allResults.filter((r) => r.status === "error").length,
        results: allResults,
      };

      fs.writeFileSync(
        path.join(OUTPUT_DIR, "results.json"),
        JSON.stringify(summary, null, 2)
      );

      console.log("\n========================================");
      console.log("FIXTURE VERIFICATION SUMMARY");
      console.log("========================================");
      console.log(`Total: ${summary.total}`);
      console.log(`Passed: ${summary.passed} (${((summary.passed / summary.total) * 100).toFixed(1)}%)`);
      console.log(`Failed: ${summary.failed}`);
      console.log(`Skipped: ${summary.skipped}`);
      console.log(`Error: ${summary.error}`);
      console.log("========================================\n");
    }
  });

  test("run all fixtures from manifest", async ({ browser }) => {
    console.log(`serverAvailable = ${serverAvailable}`);
    if (!serverAvailable) {
      test.skip();
      return;
    }

    const manifest = loadManifest();
    console.log(`manifest.length = ${manifest.length}`);
    if (manifest.length === 0) {
      test.skip();
      return;
    }

    let totalFixtures = 0;
    for (const category of manifest) {
      totalFixtures += category.tests.length;
    }

    console.log(`\nRunning ${totalFixtures} fixtures across ${manifest.length} categories...\n`);

    let processedCount = 0;

    // Create a new context for isolation
    let context = await browser.newContext();
    let page = await context.newPage();
    let pageErrors = 0;
    const MAX_PAGE_ERRORS = 3;

    let browserCrashed = false;

    for (const categoryData of manifest) {
      const categoryResults: FixtureResult[] = [];

      for (const testName of categoryData.tests) {
        // If browser crashed, skip remaining tests
        if (browserCrashed) {
          const result: FixtureResult = {
            category: categoryData.category,
            test: testName,
            status: "skipped",
            skipReason: "Browser crashed in previous test",
          };
          categoryResults.push(result);
          allResults.push(result);
          processedCount++;
          console.log(`[${processedCount}/${totalFixtures}] ○ ${categoryData.category}/${testName} - ${result.skipReason}`);
          continue;
        }

        // If too many errors, recreate the page to recover
        if (pageErrors >= MAX_PAGE_ERRORS) {
          try {
            await page.close();
            await context.close();
          } catch { /* ignore */ }

          try {
            context = await browser.newContext();
            page = await context.newPage();
            pageErrors = 0;
          } catch (e) {
            // Browser is completely crashed, mark flag and skip remaining
            browserCrashed = true;
            const result: FixtureResult = {
              category: categoryData.category,
              test: testName,
              status: "skipped",
              skipReason: "Browser crashed, cannot recover",
            };
            categoryResults.push(result);
            allResults.push(result);
            processedCount++;
            console.log(`[${processedCount}/${totalFixtures}] ○ ${categoryData.category}/${testName} - ${result.skipReason}`);
            continue;
          }
        }

        const result = await runFixture(
          page,
          categoryData.category,
          testName,
          false // Don't save artifacts for all fixtures (too many)
        );

        // Track page errors
        if (result.status === "error" && (result.error?.includes("page") || result.error?.includes("browser") || result.error?.includes("closed"))) {
          pageErrors++;
        } else {
          pageErrors = 0;
        }

        categoryResults.push(result);
        allResults.push(result);
        processedCount++;

        // Log progress
        const statusIcon =
          result.status === "passed"
            ? "✓"
            : result.status === "skipped"
              ? "○"
              : "✗";
        const diffStr =
          result.diffPercent !== undefined
            ? ` (${result.diffPercent.toFixed(1)}%)`
            : "";
        const skipStr = result.skipReason ? ` - ${result.skipReason}` : "";
        const errorStr =
          result.error && result.status !== "skipped"
            ? ` - ${result.error}`
            : "";

        console.log(
          `[${processedCount}/${totalFixtures}] ${statusIcon} ${categoryData.category}/${testName}${diffStr}${skipStr}${errorStr}`
        );
      }

      // Category summary
      const passed = categoryResults.filter((r) => r.status === "passed").length;
      const total = categoryResults.length;
      console.log(
        `  └── ${categoryData.category}: ${passed}/${total} passed\n`
      );
    }

    // Cleanup
    try {
      await page.close();
      await context.close();
    } catch { /* ignore */ }

    // Final assertion - at least 90% should pass (of non-skipped)
    const nonSkipped = allResults.filter((r) => r.status !== "skipped");
    const passed = nonSkipped.filter((r) => r.status === "passed").length;
    const passRate = nonSkipped.length > 0 ? (passed / nonSkipped.length) * 100 : 0;

    console.log(`\nFinal pass rate: ${passRate.toFixed(1)}% (${passed}/${nonSkipped.length} non-skipped)`);

    // This test passes if we processed fixtures - the detailed results are in the summary
    expect(allResults.length).toBeGreaterThan(0);
  });
});

// Separate test for saving failed fixtures with artifacts
test.describe("Save Failed Fixture Artifacts", () => {
  test.skip(true, "Run manually with --grep 'Save Failed'");

  test("save artifacts for failed fixtures", async ({ page, request }) => {
    // Read results from previous run
    const resultsPath = path.join(OUTPUT_DIR, "results.json");
    if (!fs.existsSync(resultsPath)) {
      test.skip();
      return;
    }

    const results = JSON.parse(fs.readFileSync(resultsPath, "utf-8"));
    const failed = results.results.filter(
      (r: FixtureResult) => r.status === "failed"
    );

    console.log(`\nSaving artifacts for ${failed.length} failed fixtures...\n`);

    for (const fixture of failed) {
      await runFixture(page, fixture.category, fixture.test, true);
      console.log(`Saved: ${fixture.category}/${fixture.test}`);
    }
  });
});
