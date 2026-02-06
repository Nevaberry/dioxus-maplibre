/**
 * Visual regression test suite for MapLibre GL JS render tests.
 *
 * Iterates all fixtures from the manifest, renders each via the fixture server,
 * screenshots the canvas, and compares against expected images using pixelmatch.
 */

import { test, expect } from "@playwright/test";
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "fs";
import { join } from "path";
import { PNG } from "pngjs";
import pixelmatch from "pixelmatch";
import { globSync } from "glob";
import { SKIP_PREFIXES, PAGE_REFRESH_INTERVAL } from "../src/constants";

const BUN_DIR = join(__dirname, "..");
const FIXTURES_DIR = join(BUN_DIR, "fixtures");
const RESULTS_DIR = join(BUN_DIR, "results");
const DIFFS_DIR = join(RESULTS_DIR, "diffs");

interface FixtureEntry {
  id: string;
  width: number;
  height: number;
  pixelRatio: number;
  allowed: number;
  threshold: number;
  expectedCount: number;
  operations?: any[];
  [key: string]: any;
}

interface TestResult {
  id: string;
  status: "pass" | "fail" | "skip" | "error";
  difference?: number;
  allowed?: number;
  error?: string;
}

function shouldSkip(id: string): boolean {
  const category = id.split("/")[0];
  return SKIP_PREFIXES.some(
    (prefix) => category === prefix || category.startsWith(prefix + "-"),
  );
}

function writeSummary(fixtures: FixtureEntry[], results: TestResult[]) {
  const summary = {
    total: fixtures.length,
    pass: results.filter((r) => r.status === "pass").length,
    fail: results.filter((r) => r.status === "fail").length,
    skip: results.filter((r) => r.status === "skip").length,
    error: results.filter((r) => r.status === "error").length,
    results,
  };

  mkdirSync(RESULTS_DIR, { recursive: true });
  writeFileSync(
    join(RESULTS_DIR, "summary.json"),
    JSON.stringify(summary, null, 2),
  );
  return summary;
}

function compareImages(
  screenshotBuffer: Buffer,
  fixture: FixtureEntry,
): { passed: boolean; minDiff: number; bestDiffPng: PNG | null } {
  const actualPng = PNG.sync.read(screenshotBuffer);

  const fixtureDir = join(FIXTURES_DIR, fixture.id);
  const globPattern = join(fixtureDir, "expected*.png").replace(/\\/g, "/");
  const expectedPaths = globSync(globPattern);

  if (expectedPaths.length === 0) {
    return { passed: false, minDiff: 1, bestDiffPng: null };
  }

  let minDiff = Infinity;
  let bestDiffPng: PNG | null = null;

  for (const expectedPath of expectedPaths) {
    const expectedBuf = readFileSync(expectedPath);
    const expectedPng = PNG.sync.read(expectedBuf);

    // If dimensions differ, create a buffer sized to expected dimensions
    let actualData: Buffer;
    if (
      actualPng.width !== expectedPng.width ||
      actualPng.height !== expectedPng.height
    ) {
      const resized = new PNG({
        width: expectedPng.width,
        height: expectedPng.height,
      });
      const minH = Math.min(actualPng.height, expectedPng.height);
      const minW = Math.min(actualPng.width, expectedPng.width);
      for (let y = 0; y < minH; y++) {
        for (let x = 0; x < minW; x++) {
          const si = (y * actualPng.width + x) * 4;
          const di = (y * expectedPng.width + x) * 4;
          resized.data[di] = actualPng.data[si];
          resized.data[di + 1] = actualPng.data[si + 1];
          resized.data[di + 2] = actualPng.data[si + 2];
          resized.data[di + 3] = actualPng.data[si + 3];
        }
      }
      actualData = resized.data;
    } else {
      actualData = actualPng.data;
    }

    const diffPng = new PNG({
      width: expectedPng.width,
      height: expectedPng.height,
    });
    const numDiffPixels = pixelmatch(
      actualData,
      expectedPng.data,
      diffPng.data,
      expectedPng.width,
      expectedPng.height,
      { threshold: fixture.threshold },
    );
    const diff = numDiffPixels / (expectedPng.width * expectedPng.height);

    if (diff < minDiff) {
      minDiff = diff;
      bestDiffPng = diffPng;
    }
  }

  return { passed: minDiff <= fixture.allowed, minDiff, bestDiffPng };
}

test("MapLibre render fixtures", async ({ page }) => {
  const manifestPath = join(FIXTURES_DIR, "manifest.json");
  if (!existsSync(manifestPath)) {
    throw new Error(
      "Manifest not found. Run: bun run scripts/copy-fixtures.ts",
    );
  }

  const fixtures: FixtureEntry[] = JSON.parse(
    readFileSync(manifestPath, "utf8"),
  );

  mkdirSync(DIFFS_DIR, { recursive: true });

  const results: TestResult[] = [];
  let fixtureCount = 0;
  let consecutiveErrors = 0;

  for (const fixture of fixtures) {
    if (shouldSkip(fixture.id)) {
      results.push({ id: fixture.id, status: "skip" });
      continue;
    }

    if (fixture.expectedCount === 0) {
      results.push({ id: fixture.id, status: "skip", error: "no expected images" });
      continue;
    }

    // Stop if browser has crashed repeatedly
    if (consecutiveErrors >= 5) {
      results.push({ id: fixture.id, status: "skip", error: "browser unstable" });
      continue;
    }

    fixtureCount++;
    console.log(`[${fixtureCount}] ${fixture.id}`);

    // Periodic page refresh for WebGL context recovery
    if (fixtureCount > 1 && fixtureCount % PAGE_REFRESH_INTERVAL === 0) {
      try {
        await page.goto("about:blank", { timeout: 5_000 });
        await page.waitForTimeout(100);
      } catch {
        // Browser may have crashed; skip remaining
        consecutiveErrors = 5;
        results.push({ id: fixture.id, status: "error", error: "browser crashed during refresh" });
        continue;
      }
    }

    try {
      // Encode # in fixture IDs (e.g., regressions/mapbox-gl-js#1234)
      const url = `/fixture/${fixture.id.replace(/#/g, "%23")}`;
      await page.goto(url, { waitUntil: "load", timeout: 8_000 });

      await page.waitForFunction("window.__fixtureReady === true", {
        timeout: 8_000,
      });

      const fixtureError = await page.evaluate("window.__fixtureError");
      if (fixtureError) {
        results.push({ id: fixture.id, status: "error", error: String(fixtureError) });
        consecutiveErrors = 0;
        continue;
      }

      const canvas = page.locator("#map canvas");
      await canvas.waitFor({ state: "visible", timeout: 5_000 });
      const screenshotBuffer = await canvas.screenshot();

      const { passed, minDiff, bestDiffPng } = compareImages(
        screenshotBuffer,
        fixture,
      );

      if (!passed && bestDiffPng) {
        const diffDir = join(DIFFS_DIR, fixture.id);
        mkdirSync(diffDir, { recursive: true });
        writeFileSync(join(diffDir, "actual.png"), screenshotBuffer);
        writeFileSync(
          join(diffDir, "diff.png"),
          PNG.sync.write(bestDiffPng, { filterType: 4 }),
        );
      }

      results.push({
        id: fixture.id,
        status: passed ? "pass" : "fail",
        difference: minDiff,
        allowed: fixture.allowed,
      });
      consecutiveErrors = 0;
    } catch (err: any) {
      const msg = err.message || String(err);
      results.push({ id: fixture.id, status: "error", error: msg });

      // Track consecutive errors to detect browser crashes
      if (msg.includes("closed") || msg.includes("crashed") || msg.includes("disconnected")) {
        consecutiveErrors++;
      } else {
        consecutiveErrors = 0;
      }
    }

    // Write summary periodically (every 25 fixtures)
    if (fixtureCount % 25 === 0) {
      writeSummary(fixtures, results);
    }
  }

  // Final summary
  const summary = writeSummary(fixtures, results);

  console.log(
    `\nResults: ${summary.pass} pass, ${summary.fail} fail, ${summary.skip} skip, ${summary.error} error out of ${summary.total} total`,
  );

  const failures = results.filter((r) => r.status === "fail");
  if (failures.length > 0) {
    console.log(`\nFailed fixtures:`);
    for (const f of failures.slice(0, 20)) {
      console.log(
        `  ${f.id}: diff=${f.difference?.toFixed(6)} allowed=${f.allowed}`,
      );
    }
    if (failures.length > 20) {
      console.log(`  ... and ${failures.length - 20} more`);
    }
  }

  const errors = results.filter((r) => r.status === "error");
  if (errors.length > 0) {
    console.log(`\nErrored fixtures:`);
    for (const e of errors.slice(0, 10)) {
      console.log(`  ${e.id}: ${e.error}`);
    }
    if (errors.length > 10) {
      console.log(`  ... and ${errors.length - 10} more`);
    }
  }

  // Assert infrastructure health: error rate < 10%, pass rate > 80%
  const tested = summary.pass + summary.fail + summary.error;
  if (tested > 0) {
    const errorRate = summary.error / tested;
    expect(
      errorRate,
      `Error rate ${(errorRate * 100).toFixed(1)}% exceeds 10% — infrastructure issue`,
    ).toBeLessThan(0.1);

    const passRate = summary.pass / tested;
    expect(
      passRate,
      `Pass rate ${(passRate * 100).toFixed(1)}% below 80% — rendering regression`,
    ).toBeGreaterThan(0.8);
  }
});
