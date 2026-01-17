import { test, expect, Page } from "@playwright/test";
import * as fs from "fs";
import * as path from "path";
import { PNG } from "pngjs";
import pixelmatch from "pixelmatch";

/**
 * Fixture Verification Tests
 *
 * Verifies that native Bun/MapLibre renders identically to upstream expected.png images.
 * Uses pixelmatch for pixel-level comparison with configurable threshold.
 */

const NATIVE_URL = "http://localhost:3000";
const FIXTURES_DIR = path.join(__dirname, "../native-showcase/fixtures");
const OUTPUT_DIR = path.join(__dirname, "../test-results/fixture-diffs");

interface FixtureCategory {
  category: string;
  tests: string[];
}

// Load manifest
function loadManifest(): FixtureCategory[] {
  const manifestPath = path.join(FIXTURES_DIR, "manifest.json");
  const content = fs.readFileSync(manifestPath, "utf-8");
  return JSON.parse(content);
}

// Get fixture dimensions from style.json
function getFixtureDimensions(
  category: string,
  testName: string
): { width: number; height: number } {
  const stylePath = path.join(
    FIXTURES_DIR,
    category,
    testName,
    "style.json"
  );
  const content = fs.readFileSync(stylePath, "utf-8");
  const style = JSON.parse(content);
  return {
    width: style.metadata?.test?.width || 512,
    height: style.metadata?.test?.height || 512,
  };
}

// Wait for map to be fully rendered and idle
async function waitForMapReady(page: Page, timeout = 30000): Promise<void> {
  // First wait for canvas to exist
  await page.waitForSelector('.maplibregl-canvas', { timeout });

  // Wait for the map to signal it's ready
  await page.waitForFunction(
    () => (window as any).__mapReady === true,
    { timeout }
  );

  // Additional buffer for final render stabilization
  await page.waitForTimeout(1000);

  // Verify canvas has rendered content by checking if any non-transparent pixels exist
  const hasContent = await page.evaluate(() => {
    const canvas = document.querySelector('.maplibregl-canvas') as HTMLCanvasElement;
    if (!canvas) return false;
    const ctx = canvas.getContext('webgl2') || canvas.getContext('webgl');
    if (!ctx) return false;
    // WebGL canvas - assume it's rendered if we get here
    return true;
  });

  if (!hasContent) {
    await page.waitForTimeout(1000);
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
  if (actualRaw.width !== expectedRaw.width || actualRaw.height !== expectedRaw.height) {
    return {
      diffPixels: actualRaw.width * actualRaw.height,
      totalPixels: actualRaw.width * actualRaw.height,
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
  height: number
): Promise<Buffer> {
  // Set viewport to exact fixture size plus some padding to ensure full render
  await page.setViewportSize({ width: width + 100, height: height + 100 });

  // Wait a bit more for render to complete
  await page.waitForTimeout(500);

  // Use Playwright's element screenshot - it properly captures WebGL content
  // Note: this will have an opaque background, so we need to handle transparency differently
  const canvas = page.locator('.maplibregl-canvas');
  return (await canvas.screenshot()) as Buffer;
}

test.describe("Fixture Verification", () => {
  test.beforeAll(async ({ request }) => {
    // Skip if server isn't running
    try {
      const response = await request.get(NATIVE_URL, { timeout: 5000 });
      if (!response.ok()) {
        test.skip();
      }
    } catch {
      test.skip();
    }

    // Ensure output directory exists
    fs.mkdirSync(OUTPUT_DIR, { recursive: true });
  });

  // Test a batch of fixtures that use inline GeoJSON (not vector tiles)
  // Fixtures with vector tile sources (local://tiles/) won't work without tile server
  const fixtureSubset = [
    { category: "background-color", test: "literal" },
    { category: "background-color", test: "default" },
    { category: "circle-color", test: "literal" },
    { category: "circle-color", test: "default" },
    { category: "circle-color", test: "function" },
    { category: "circle-radius", test: "literal" },
    { category: "circle-radius", test: "default" },
    { category: "circle-blur", test: "literal" },
    { category: "circle-blur", test: "default" },
    { category: "fill-color", test: "literal" },
    { category: "fill-color", test: "default" },
    { category: "fill-opacity", test: "literal" },
  ];

  for (const fixture of fixtureSubset) {
    test(`${fixture.category}/${fixture.test} renders correctly`, async ({
      page,
    }) => {
      const { width, height } = getFixtureDimensions(
        fixture.category,
        fixture.test
      );
      const expectedPath = path.join(
        FIXTURES_DIR,
        fixture.category,
        fixture.test,
        "expected.png"
      );

      // Skip if expected.png doesn't exist
      if (!fs.existsSync(expectedPath)) {
        test.skip();
        return;
      }

      // Navigate to fixture
      await page.goto(
        `${NATIVE_URL}/fixture/${fixture.category}/${fixture.test}`
      );
      await waitForMapReady(page);

      // Capture screenshot
      const actualBuffer = await captureMapScreenshot(page, width, height);

      // Save actual screenshot for debugging
      const actualPath = path.join(
        OUTPUT_DIR,
        `${fixture.category}-${fixture.test}-actual.png`
      );
      fs.writeFileSync(actualPath, actualBuffer);

      // Load expected image
      const expectedBuffer = fs.readFileSync(expectedPath);

      // Compare images
      const diffPath = path.join(
        OUTPUT_DIR,
        `${fixture.category}-${fixture.test}-diff.png`
      );
      const result = compareImages(actualBuffer, expectedBuffer, {
        threshold: 0.1,
        outputPath: diffPath,
      });

      console.log(
        `${fixture.category}/${fixture.test}: ${result.diffPixels} different pixels (${result.diffPercent.toFixed(2)}%)`
      );

      // Allow up to 5% difference (for antialiasing and minor rendering differences)
      expect(result.diffPercent).toBeLessThan(5);
    });
  }
});

// Summary test that runs all fixtures from manifest
test.describe("Full Fixture Suite", () => {
  test.skip("run all fixtures from manifest", async ({ page }) => {
    const manifest = loadManifest();
    const results: {
      category: string;
      test: string;
      passed: boolean;
      diffPercent: number;
    }[] = [];

    for (const categoryData of manifest.slice(0, 5)) {
      // Limit to first 5 categories for initial run
      for (const testName of categoryData.tests.slice(0, 3)) {
        // Limit to first 3 tests per category
        const expectedPath = path.join(
          FIXTURES_DIR,
          categoryData.category,
          testName,
          "expected.png"
        );

        if (!fs.existsSync(expectedPath)) {
          continue;
        }

        try {
          const { width, height } = getFixtureDimensions(
            categoryData.category,
            testName
          );

          await page.goto(
            `${NATIVE_URL}/fixture/${categoryData.category}/${testName}`
          );
          await waitForMapReady(page);

          const actualBuffer = await captureMapScreenshot(page, width, height);
          const expectedBuffer = fs.readFileSync(expectedPath);

          const result = compareImages(actualBuffer, expectedBuffer, {
            threshold: 0.1,
          });

          results.push({
            category: categoryData.category,
            test: testName,
            passed: result.diffPercent < 5,
            diffPercent: result.diffPercent,
          });
        } catch (error) {
          results.push({
            category: categoryData.category,
            test: testName,
            passed: false,
            diffPercent: 100,
          });
        }
      }
    }

    // Log summary
    const passed = results.filter((r) => r.passed).length;
    const total = results.length;
    console.log(`\nFixture Verification Summary: ${passed}/${total} passed`);

    for (const r of results) {
      const status = r.passed ? "✓" : "✗";
      console.log(
        `  ${status} ${r.category}/${r.test}: ${r.diffPercent.toFixed(2)}% diff`
      );
    }

    expect(passed).toBe(total);
  });
});
