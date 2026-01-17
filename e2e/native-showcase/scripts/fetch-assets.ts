/**
 * Fetch MapLibre GL JS test assets (tiles, sprites, glyphs, images)
 *
 * Downloads required test resources from MapLibre's integration test suite
 * to enable running fixture tests that require these resources.
 */

import { mkdir, writeFile, rm, readFile, stat } from "fs/promises";
import { join, dirname } from "path";

const MAPLIBRE_REPO = "maplibre/maplibre-gl-js";
const BRANCH = "main";
const ASSETS_PATH = "test/integration/assets";
const ASSETS_DIR = join(import.meta.dir, "..", "assets");

// Rate limit settings
const MAX_RETRIES = 5;
const INITIAL_DELAY_MS = 1000;
const MAX_DELAY_MS = 60000;
const CONCURRENT_REQUESTS = process.env.GITHUB_TOKEN ? 15 : 3;

interface GitHubContent {
  name: string;
  path: string;
  type: "file" | "dir";
  download_url?: string;
  size?: number;
}

async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function fetchWithRetry<T>(
  url: string,
  description: string,
  retries = MAX_RETRIES
): Promise<T> {
  let lastError: Error | null = null;
  let delay = INITIAL_DELAY_MS;

  for (let attempt = 1; attempt <= retries; attempt++) {
    try {
      const response = await fetch(url, {
        headers: {
          Accept: "application/vnd.github.v3+json",
          "User-Agent": "dioxus-maplibre-e2e",
          ...(process.env.GITHUB_TOKEN && {
            Authorization: `token ${process.env.GITHUB_TOKEN}`,
          }),
        },
      });

      if (response.status === 403 || response.status === 429) {
        const resetTime = response.headers.get("x-ratelimit-reset");
        const resetMs = resetTime
          ? parseInt(resetTime) * 1000 - Date.now()
          : delay;
        const waitTime = Math.min(Math.max(resetMs, delay), MAX_DELAY_MS);

        console.log(
          `\n  Rate limited. Waiting ${Math.ceil(waitTime / 1000)}s (attempt ${attempt}/${retries})...`
        );
        await sleep(waitTime);
        delay = Math.min(delay * 2, MAX_DELAY_MS);
        continue;
      }

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      return response.json() as Promise<T>;
    } catch (error) {
      lastError = error as Error;
      if (attempt < retries) {
        console.log(
          `\n  Error fetching ${description}: ${lastError.message}. Retrying in ${delay / 1000}s...`
        );
        await sleep(delay);
        delay = Math.min(delay * 2, MAX_DELAY_MS);
      }
    }
  }

  throw new Error(
    `Failed to fetch ${description} after ${retries} attempts: ${lastError?.message}`
  );
}

async function fetchBinary(url: string, description: string): Promise<Buffer> {
  let lastError: Error | null = null;
  let delay = INITIAL_DELAY_MS;

  for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
    try {
      const response = await fetch(url);
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }
      const buffer = await response.arrayBuffer();
      return Buffer.from(buffer);
    } catch (error) {
      lastError = error as Error;
      if (attempt < MAX_RETRIES) {
        await sleep(delay);
        delay = Math.min(delay * 2, MAX_DELAY_MS);
      }
    }
  }
  throw new Error(`Failed to fetch ${description}: ${lastError?.message}`);
}

async function fetchGitHubDirectory(path: string): Promise<GitHubContent[]> {
  const encodedPath = path
    .split("/")
    .map(encodeURIComponent)
    .join("/");
  const url = `https://api.github.com/repos/${MAPLIBRE_REPO}/contents/${encodedPath}?ref=${BRANCH}`;
  return fetchWithRetry<GitHubContent[]>(url, path);
}

async function downloadFile(
  content: GitHubContent,
  targetDir: string
): Promise<void> {
  if (!content.download_url) return;

  const targetPath = join(targetDir, content.name);
  await mkdir(dirname(targetPath), { recursive: true });

  const buffer = await fetchBinary(content.download_url, content.path);
  await writeFile(targetPath, buffer);
}

async function downloadDirectory(
  path: string,
  targetDir: string,
  depth = 0
): Promise<number> {
  const contents = await fetchGitHubDirectory(path);
  let downloadedCount = 0;

  const files = contents.filter((c) => c.type === "file");
  const dirs = contents.filter((c) => c.type === "dir");

  // Download files in batches
  for (let i = 0; i < files.length; i += CONCURRENT_REQUESTS) {
    const batch = files.slice(i, i + CONCURRENT_REQUESTS);
    await Promise.all(
      batch.map(async (file) => {
        try {
          await downloadFile(file, targetDir);
          downloadedCount++;
          process.stdout.write(".");
        } catch (error) {
          console.log(`\n  Warning: Failed to download ${file.path}`);
        }
      })
    );
  }

  // Recursively download subdirectories
  for (const dir of dirs) {
    const subDir = join(targetDir, dir.name);
    await mkdir(subDir, { recursive: true });
    downloadedCount += await downloadDirectory(dir.path, subDir, depth + 1);
  }

  return downloadedCount;
}

async function downloadAssets() {
  console.log("Fetching MapLibre test assets...\n");

  // Asset directories to download
  const assetDirs = ["glyphs", "sprites", "tiles", "image", "video", "geojson", "data"];

  // Create assets directory
  await mkdir(ASSETS_DIR, { recursive: true });

  let totalFiles = 0;

  for (const assetDir of assetDirs) {
    const targetDir = join(ASSETS_DIR, assetDir);
    console.log(`Downloading ${assetDir}...`);

    try {
      await mkdir(targetDir, { recursive: true });
      const count = await downloadDirectory(
        `${ASSETS_PATH}/${assetDir}`,
        targetDir
      );
      totalFiles += count;
      console.log(` ${count} files`);
    } catch (error) {
      console.log(` Error: ${(error as Error).message}`);
    }
  }

  console.log(`\nDownloaded ${totalFiles} asset files`);
  console.log(`Assets directory: ${ASSETS_DIR}`);

  if (!process.env.GITHUB_TOKEN) {
    console.log(
      `\nTip: Set GITHUB_TOKEN env var for higher rate limits (5000/hour vs 60/hour)`
    );
  }
}

// Parse command line arguments
const args = process.argv.slice(2);

if (args.includes("--help") || args.includes("-h")) {
  console.log(`
Usage: bun run fetch-assets.ts [options]

Options:
  --help     Show this help message

Environment:
  GITHUB_TOKEN   GitHub personal access token for higher rate limits
                 (60 req/hour without, 5000 req/hour with token)

This script downloads test assets (tiles, sprites, glyphs, images, video)
from the MapLibre GL JS repository for use in fixture verification tests.
`);
  process.exit(0);
}

downloadAssets().catch((error) => {
  console.error(`\nFatal error: ${error.message}`);
  process.exit(1);
});
