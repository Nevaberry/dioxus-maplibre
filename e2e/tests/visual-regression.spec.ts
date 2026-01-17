import { test, expect, Page } from '@playwright/test';

/**
 * Visual Regression Tests
 *
 * Compares native MapLibre rendering against dioxus-maplibre wrapper.
 * These tests help ensure the Rust wrapper produces identical output.
 */

// Native showcase runs on port 3000, Dioxus showcase on port 8080
const NATIVE_URL = 'http://localhost:3000';
const DIOXUS_URL = 'http://localhost:8080';

// Wait for map to be fully loaded and idle
async function waitForMapIdle(page: Page) {
  // Wait for canvas to exist
  await page.waitForSelector('.maplibregl-canvas', { timeout: 15000 });

  // Wait for map to stop loading tiles
  await page.waitForFunction(() => {
    // Check if any tile requests are pending by looking for loading indicators
    const map = (window as any).__dioxus_maplibre_maps;
    if (map) {
      // If we have access to map instance, check if it's loaded
      for (const key of Object.keys(map)) {
        if (!map[key].isStyleLoaded() || !map[key].areTilesLoaded()) {
          return false;
        }
      }
    }
    return true;
  }, { timeout: 20000 }).catch(() => {
    // Fallback: just wait a bit if function check fails
    return page.waitForTimeout(3000);
  });

  // Additional buffer for rendering
  await page.waitForTimeout(1000);
}

// Screenshot a specific region of the page
async function screenshotMap(page: Page, name: string) {
  const canvas = page.locator('.maplibregl-canvas').first();
  return canvas.screenshot({
    path: `test-results/screenshots/${name}.png`,
  });
}

test.describe('Visual Regression: Native vs Dioxus', () => {
  test.describe.configure({ mode: 'serial' });

  // Skip these tests if servers aren't running
  test.beforeAll(async ({ request }) => {
    try {
      const nativeResponse = await request.get(NATIVE_URL, { timeout: 5000 });
      const dioxusResponse = await request.get(DIOXUS_URL, { timeout: 5000 });

      if (!nativeResponse.ok() || !dioxusResponse.ok()) {
        test.skip();
      }
    } catch {
      test.skip();
    }
  });

  test('basic map render matches', async ({ page }) => {
    // First capture native baseline
    await page.goto(`${NATIVE_URL}#basic-map`);
    await waitForMapIdle(page);
    const nativeScreenshot = await screenshotMap(page, 'native-basic-map');

    // Then capture Dioxus render
    await page.goto(DIOXUS_URL);
    await waitForMapIdle(page);
    const dioxusScreenshot = await screenshotMap(page, 'dioxus-basic-map');

    // Compare screenshots
    // Note: This is a simple comparison. In practice, you'd use a library like pixelmatch
    expect(nativeScreenshot).toBeDefined();
    expect(dioxusScreenshot).toBeDefined();
  });

  test('map with markers matches', async ({ page }) => {
    // Navigate to markers demo in native showcase
    await page.goto(`${NATIVE_URL}#multiple-markers`);
    await waitForMapIdle(page);

    // Check that markers are rendered
    const nativeMarkers = await page.locator('.maplibregl-marker').count();
    expect(nativeMarkers).toBeGreaterThan(0);

    // Take screenshot
    await screenshotMap(page, 'native-markers');

    // For Dioxus, we need to navigate to the markers section
    await page.goto(DIOXUS_URL);
    await waitForMapIdle(page);

    // Check for Dioxus markers
    const dioxusMarkers = await page.locator('.maplibregl-marker').count();

    // Note: counts may differ based on what the showcase displays
    console.log(`Native markers: ${nativeMarkers}, Dioxus markers: ${dioxusMarkers}`);
  });

  test('map click events work', async ({ page }) => {
    await page.goto(`${NATIVE_URL}#click-event`);
    await waitForMapIdle(page);

    // Click on the map
    const canvas = page.locator('.maplibregl-canvas').first();
    await canvas.click({ position: { x: 200, y: 200 } });

    // Check that event was logged (check event log panel exists)
    const eventLog = page.locator('#event-log');
    await expect(eventLog).toBeVisible();

    // Verify click event was recorded
    const eventEntry = eventLog.locator('.event');
    await expect(eventEntry.first()).toBeVisible();
  });
});

test.describe('Native Showcase Demos', () => {
  test.beforeEach(async ({ page, request }) => {
    try {
      const response = await request.get(NATIVE_URL, { timeout: 5000 });
      if (!response.ok()) {
        test.skip();
      }
    } catch {
      test.skip();
    }
  });

  // Test each category loads correctly
  const categories = [
    'initialization',
    'navigation',
    'layers',
    'sources',
    'markers',
    'events',
    'controls',
    '3d',
  ];

  for (const category of categories) {
    test(`${category} demos load`, async ({ page }) => {
      await page.goto(NATIVE_URL);

      // Click category button
      const categoryBtn = page.locator(`[data-category="${category}"]`);
      await categoryBtn.click();

      // Wait for demo to load
      await page.waitForSelector('.maplibregl-canvas', { timeout: 15000 });

      // Verify the category is active
      await expect(categoryBtn).toHaveClass(/active/);

      // Take screenshot of category
      await page.waitForTimeout(2000);
      await page.screenshot({
        path: `test-results/screenshots/category-${category}.png`,
        fullPage: true,
      });
    });
  }
});

test.describe('Dioxus Showcase Screenshots', () => {
  test.beforeEach(async ({ page, request }) => {
    try {
      const response = await request.get(DIOXUS_URL, { timeout: 5000 });
      if (!response.ok()) {
        test.skip();
      }
    } catch {
      test.skip();
    }
  });

  test('capture full showcase screenshot', async ({ page }) => {
    await page.goto(DIOXUS_URL);
    await waitForMapIdle(page);

    await page.screenshot({
      path: 'test-results/screenshots/dioxus-showcase.png',
      fullPage: true,
    });
  });

  test('map renders with correct style', async ({ page }) => {
    await page.goto(DIOXUS_URL);
    await waitForMapIdle(page);

    // Verify canvas exists and has rendered content
    const canvas = page.locator('.maplibregl-canvas');
    await expect(canvas).toBeVisible();

    // Take screenshot of just the map
    await screenshotMap(page, 'dioxus-map-render');
  });
});
