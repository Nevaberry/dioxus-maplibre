/**
 * Scans the MapLibre GL JS submodule for render test fixtures,
 * copies style.json + expected*.png files, and generates a manifest.
 *
 * Usage: bun run scripts/copy-fixtures.ts
 */

import { readFileSync, writeFileSync, mkdirSync, cpSync, existsSync, symlinkSync, readdirSync, statSync } from "fs";
import { join, dirname, relative } from "path";
import { globSync } from "glob";
import {
  DEFAULT_WIDTH,
  DEFAULT_HEIGHT,
  DEFAULT_PIXEL_RATIO,
  DEFAULT_ALLOWED,
  DEFAULT_THRESHOLD,
} from "../src/constants";

const ROOT = dirname(dirname(import.meta.dir));
const SUBMODULE_TESTS = join(
  ROOT,
  "maplibre-gl-js/test/integration/render/tests",
);
const SUBMODULE_ASSETS = join(ROOT, "maplibre-gl-js/test/integration/assets");
const OUTPUT_DIR = join(dirname(import.meta.dir), "fixtures");

interface FixtureEntry {
  id: string;
  width: number;
  height: number;
  pixelRatio: number;
  allowed: number;
  threshold: number;
  fadeDuration?: number;
  localIdeographFontFamily?: string | false;
  crossSourceCollisions?: boolean;
  maxPitch?: number;
  continuesRepaint?: boolean;
  debug?: boolean;
  showOverdrawInspector?: boolean;
  showPadding?: boolean;
  collisionDebug?: boolean;
  operations?: any[];
  addFakeCanvas?: { id: string; image: string };
  reportWidth?: number;
  reportHeight?: number;
  expectedCount: number;
}

function main() {
  if (!existsSync(SUBMODULE_TESTS)) {
    console.error(
      `Submodule tests not found at ${SUBMODULE_TESTS}.\nRun: git submodule update --init`,
    );
    process.exit(1);
  }

  // Clean and recreate output directory (keep assets symlink check separate)
  mkdirSync(OUTPUT_DIR, { recursive: true });

  // Symlink assets directory
  const assetsLink = join(OUTPUT_DIR, "assets");
  if (!existsSync(assetsLink)) {
    try {
      const relAssets = relative(OUTPUT_DIR, SUBMODULE_ASSETS);
      symlinkSync(relAssets, assetsLink, "dir");
      console.log(`Symlinked assets -> ${relAssets}`);
    } catch {
      console.log("Symlink failed, copying assets directory...");
      cpSync(SUBMODULE_ASSETS, assetsLink, { recursive: true });
    }
  }

  // Discover all style.json files
  const styleFiles = globSync("**/style.json", { cwd: SUBMODULE_TESTS });
  console.log(`Found ${styleFiles.length} style.json files`);

  const manifest: FixtureEntry[] = [];

  for (const relPath of styleFiles) {
    const id = dirname(relPath); // e.g. "circle-radius/literal"
    const srcDir = join(SUBMODULE_TESTS, dirname(relPath));
    const dstDir = join(OUTPUT_DIR, id);

    mkdirSync(dstDir, { recursive: true });

    // Read and parse style.json
    const stylePath = join(SUBMODULE_TESTS, relPath);
    const styleRaw = readFileSync(stylePath, "utf8");
    let style: any;
    try {
      style = JSON.parse(styleRaw);
    } catch {
      console.warn(`  Skipping ${id}: invalid JSON`);
      continue;
    }

    // Copy style.json
    writeFileSync(join(dstDir, "style.json"), styleRaw);

    // Extract test metadata with upstream defaults
    const testMeta = style.metadata?.test || {};
    const entry: FixtureEntry = {
      id,
      width: testMeta.width ?? DEFAULT_WIDTH,
      height: testMeta.height ?? DEFAULT_HEIGHT,
      pixelRatio: testMeta.pixelRatio ?? DEFAULT_PIXEL_RATIO,
      allowed: testMeta.allowed ?? DEFAULT_ALLOWED,
      threshold: testMeta.threshold ?? DEFAULT_THRESHOLD,
      expectedCount: 0,
    };

    // Copy optional fields
    if (testMeta.fadeDuration !== undefined)
      entry.fadeDuration = testMeta.fadeDuration;
    if (testMeta.localIdeographFontFamily !== undefined)
      entry.localIdeographFontFamily = testMeta.localIdeographFontFamily;
    if (testMeta.crossSourceCollisions !== undefined)
      entry.crossSourceCollisions = testMeta.crossSourceCollisions;
    if (testMeta.maxPitch !== undefined) entry.maxPitch = testMeta.maxPitch;
    if (testMeta.continuesRepaint !== undefined)
      entry.continuesRepaint = testMeta.continuesRepaint;
    if (testMeta.debug !== undefined) entry.debug = testMeta.debug;
    if (testMeta.showOverdrawInspector !== undefined)
      entry.showOverdrawInspector = testMeta.showOverdrawInspector;
    if (testMeta.showPadding !== undefined)
      entry.showPadding = testMeta.showPadding;
    if (testMeta.collisionDebug !== undefined)
      entry.collisionDebug = testMeta.collisionDebug;
    if (testMeta.operations !== undefined)
      entry.operations = testMeta.operations;
    if (testMeta.addFakeCanvas !== undefined)
      entry.addFakeCanvas = testMeta.addFakeCanvas;
    if (testMeta.reportWidth !== undefined)
      entry.reportWidth = testMeta.reportWidth;
    if (testMeta.reportHeight !== undefined)
      entry.reportHeight = testMeta.reportHeight;

    // Copy expected*.png files
    const srcEntries = readdirSync(srcDir);
    let expectedCount = 0;
    for (const name of srcEntries) {
      if (name.match(/^expected.*\.png$/)) {
        cpSync(join(srcDir, name), join(dstDir, name));
        expectedCount++;
      }
    }
    entry.expectedCount = expectedCount;

    // Copy any subdirectories (e.g. image/ for canvas tests)
    for (const name of srcEntries) {
      const srcPath = join(srcDir, name);
      if (statSync(srcPath).isDirectory()) {
        cpSync(srcPath, join(dstDir, name), { recursive: true });
      }
    }

    manifest.push(entry);
  }

  // Write manifest
  const manifestPath = join(OUTPUT_DIR, "manifest.json");
  writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));

  console.log(
    `Copied ${manifest.length} fixtures to ${OUTPUT_DIR}`,
  );
  console.log(
    `Total expected images: ${manifest.reduce((sum, f) => sum + f.expectedCount, 0)}`,
  );
  console.log(`Manifest written to ${manifestPath}`);
}

main();
