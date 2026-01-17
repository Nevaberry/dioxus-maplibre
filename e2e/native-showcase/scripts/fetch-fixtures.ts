/**
 * Fetch MapLibre GL JS render test fixtures
 *
 * Downloads style.json files from MapLibre's render test suite
 * to use as baseline test cases for visual regression testing.
 */

import { mkdir, writeFile, rm, readFile } from "fs/promises";
import { join } from "path";

const MAPLIBRE_REPO = "maplibre/maplibre-gl-js";
const BRANCH = "main";
const TESTS_PATH = "test/integration/render/tests";
const FIXTURES_DIR = join(import.meta.dir, "..", "fixtures");
const PROGRESS_FILE = join(FIXTURES_DIR, ".progress.json");

// Rate limit settings
const MAX_RETRIES = 5;
const INITIAL_DELAY_MS = 1000;
const MAX_DELAY_MS = 60000;
// With GITHUB_TOKEN: 5000 req/hour, without: 60 req/hour
const CONCURRENT_REQUESTS = process.env.GITHUB_TOKEN ? 15 : 3;

interface GitHubContent {
  name: string;
  path: string;
  type: "file" | "dir";
  download_url?: string;
}

interface Progress {
  completedCategories: string[];
  failedCategories: string[];
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
          // Add GitHub token if available (greatly increases rate limit)
          ...(process.env.GITHUB_TOKEN && {
            Authorization: `token ${process.env.GITHUB_TOKEN}`,
          }),
        },
      });

      // Check rate limit headers
      const remaining = response.headers.get("x-ratelimit-remaining");
      const resetTime = response.headers.get("x-ratelimit-reset");

      if (response.status === 403 || response.status === 429) {
        // Rate limited
        const resetMs = resetTime
          ? (parseInt(resetTime) * 1000 - Date.now())
          : delay;
        const waitTime = Math.min(Math.max(resetMs, delay), MAX_DELAY_MS);

        console.log(
          `\n  Rate limited on ${description}. Waiting ${Math.ceil(waitTime / 1000)}s (attempt ${attempt}/${retries})...`
        );
        await sleep(waitTime);
        delay = Math.min(delay * 2, MAX_DELAY_MS);
        continue;
      }

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      // Warn if rate limit is getting low
      if (remaining && parseInt(remaining) < 10) {
        console.log(`\n  Warning: Only ${remaining} API requests remaining`);
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

async function fetchGitHubDirectory(path: string): Promise<GitHubContent[]> {
  // Encode path segments to handle special characters like # in directory names
  const encodedPath = path.split('/').map(encodeURIComponent).join('/');
  const url = `https://api.github.com/repos/${MAPLIBRE_REPO}/contents/${encodedPath}?ref=${BRANCH}`;
  return fetchWithRetry<GitHubContent[]>(url, path);
}

async function fetchFile(url: string, description: string): Promise<string> {
  let lastError: Error | null = null;
  let delay = INITIAL_DELAY_MS;

  for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
    try {
      const response = await fetch(url);
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }
      return response.text();
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

async function loadProgress(): Promise<Progress> {
  try {
    const data = await readFile(PROGRESS_FILE, "utf-8");
    return JSON.parse(data);
  } catch {
    return { completedCategories: [], failedCategories: [] };
  }
}

async function saveProgress(progress: Progress): Promise<void> {
  await writeFile(PROGRESS_FILE, JSON.stringify(progress, null, 2));
}

async function downloadFixtures(resume = false) {
  console.log("Fetching MapLibre render test fixtures...\n");

  // Load progress if resuming
  let progress = resume ? await loadProgress() : { completedCategories: [], failedCategories: [] };

  if (!resume) {
    // Clean up old fixtures directory
    console.log("Cleaning up old fixtures...");
    try {
      await rm(FIXTURES_DIR, { recursive: true, force: true });
    } catch {
      // Directory might not exist
    }
  }

  // Create fixtures directory
  await mkdir(FIXTURES_DIR, { recursive: true });

  // Get test categories
  const categories = await fetchGitHubDirectory(TESTS_PATH);
  const testDirs = categories.filter((c) => c.type === "dir");

  console.log(`Found ${testDirs.length} test categories`);

  if (resume && progress.completedCategories.length > 0) {
    console.log(`Resuming from ${progress.completedCategories.length} completed categories\n`);
  } else {
    console.log();
  }

  let totalTests = 0;
  const manifest: { category: string; tests: string[] }[] = [];
  const failedCategories: string[] = [];

  for (const category of testDirs) {
    // Skip already completed categories when resuming
    if (resume && progress.completedCategories.includes(category.name)) {
      console.log(`  ${category.name}: skipped (already completed)`);
      continue;
    }

    const categoryDir = join(FIXTURES_DIR, category.name);
    await mkdir(categoryDir, { recursive: true });

    try {
      // Get tests in this category
      const tests = await fetchGitHubDirectory(category.path);
      const testSubDirs = tests.filter((t) => t.type === "dir");

      const categoryTests: string[] = [];
      let completedCount = 0;

      // Process a single test
      const processTest = async (test: GitHubContent): Promise<string | null> => {
        try {
          const files = await fetchGitHubDirectory(test.path);
          const styleFile = files.find((f) => f.name === "style.json");

          if (styleFile && styleFile.download_url) {
            const testDir = join(categoryDir, test.name);
            await mkdir(testDir, { recursive: true });

            // Download style.json and expected.png in parallel
            const expectedFile = files.find((f) => f.name === "expected.png");
            const downloads: Promise<void>[] = [
              fetchFile(styleFile.download_url, `${category.name}/${test.name}/style.json`)
                .then(content => writeFile(join(testDir, "style.json"), content)),
            ];
            if (expectedFile?.download_url) {
              downloads.push(
                fetchBinary(expectedFile.download_url, `${category.name}/${test.name}/expected.png`)
                  .then(buffer => writeFile(join(testDir, "expected.png"), buffer))
              );
            }

            // Download subdirectories (like image/) that contain fixture-specific assets
            const subDirs = files.filter((f) => f.type === "dir");
            for (const subDir of subDirs) {
              try {
                const subFiles = await fetchGitHubDirectory(subDir.path);
                const subDirPath = join(testDir, subDir.name);
                await mkdir(subDirPath, { recursive: true });

                for (const subFile of subFiles.filter((f) => f.type === "file" && f.download_url)) {
                  downloads.push(
                    fetchBinary(subFile.download_url!, `${category.name}/${test.name}/${subDir.name}/${subFile.name}`)
                      .then(buffer => writeFile(join(subDirPath, subFile.name), buffer))
                  );
                }
              } catch {
                // Subdirectory download failed, not critical
              }
            }

            await Promise.all(downloads);
            return test.name;
          }
          return null;
        } catch (error) {
          console.log(`\n  Warning: Failed to download ${category.name}/${test.name}: ${(error as Error).message}`);
          return null;
        } finally {
          completedCount++;
          process.stdout.write(`\r  ${category.name}: ${completedCount}/${testSubDirs.length} tests`);
        }
      };

      // Process tests in parallel batches
      for (let i = 0; i < testSubDirs.length; i += CONCURRENT_REQUESTS) {
        const batch = testSubDirs.slice(i, i + CONCURRENT_REQUESTS);
        const results = await Promise.all(batch.map(processTest));
        categoryTests.push(...results.filter((r): r is string => r !== null));
      }

      totalTests += categoryTests.length;

      if (categoryTests.length > 0) {
        manifest.push({ category: category.name, tests: categoryTests });
        console.log(); // newline after progress

        // Mark category as completed
        progress.completedCategories.push(category.name);
        await saveProgress(progress);
      }
    } catch (error) {
      console.log(
        `\n  Error processing category ${category.name}: ${(error as Error).message}`
      );
      failedCategories.push(category.name);
      progress.failedCategories.push(category.name);
      await saveProgress(progress);
    }
  }

  // Write manifest
  await writeFile(
    join(FIXTURES_DIR, "manifest.json"),
    JSON.stringify(manifest, null, 2)
  );

  // Clean up progress file on success
  if (failedCategories.length === 0) {
    try {
      await rm(PROGRESS_FILE);
    } catch {
      // Ignore
    }
  }

  console.log(`\nDownloaded ${totalTests} test fixtures`);
  console.log(`Manifest written to: fixtures/manifest.json`);

  if (failedCategories.length > 0) {
    console.log(`\nFailed categories (${failedCategories.length}):`);
    failedCategories.forEach((c) => console.log(`  - ${c}`));
    console.log(`\nRun with --resume to retry failed categories`);
  }

  // Show rate limit tip if no token
  if (!process.env.GITHUB_TOKEN) {
    console.log(
      `\nTip: Set GITHUB_TOKEN env var for higher rate limits (5000/hour vs 60/hour)`
    );
  }
}

// Simpler approach: just download a curated list of important tests
async function downloadCuratedFixtures() {
  console.log("Downloading curated MapLibre test fixtures...\n");

  // Curated list of important render tests
  const curatedTests = [
    // Background layer tests
    "background-color/basic",
    "background-pattern/basic",
    // Fill layer tests
    "fill-color/data-driven",
    "fill-opacity/zoom-and-property-function",
    "fill-outline-color/basic",
    // Line layer tests
    "line-cap/round",
    "line-color/data-driven",
    "line-dasharray/basic",
    "line-width/data-driven",
    // Circle layer tests
    "circle-color/data-driven",
    "circle-radius/zoom-and-property-function",
    // Symbol layer tests
    "text-field/basic",
    "text-size/data-driven",
    "icon-image/basic",
    // Raster layer tests
    "raster-opacity/basic",
    // Heatmap layer tests
    "heatmap/basic",
    // Fill extrusion tests
    "fill-extrusion-color/basic",
    "fill-extrusion-height/data-driven",
  ];

  // Clean up old fixtures
  console.log("Cleaning up old fixtures...");
  try {
    await rm(FIXTURES_DIR, { recursive: true, force: true });
  } catch {
    // Directory might not exist
  }

  await mkdir(FIXTURES_DIR, { recursive: true });

  const downloaded: string[] = [];
  const failed: string[] = [];

  const processTest = async (testPath: string): Promise<void> => {
    try {
      const fullPath = `${TESTS_PATH}/${testPath}`;
      const files = await fetchGitHubDirectory(fullPath);
      const styleFile = files.find((f) => f.name === "style.json");

      if (styleFile && styleFile.download_url) {
        const [category, name] = testPath.split("/");
        const testDir = join(FIXTURES_DIR, category, name);
        await mkdir(testDir, { recursive: true });

        const styleContent = await fetchFile(
          styleFile.download_url,
          `${testPath}/style.json`
        );
        await writeFile(join(testDir, "style.json"), styleContent);

        downloaded.push(testPath);
        console.log(`  Downloaded: ${testPath}`);
      }
    } catch (error) {
      console.log(`  Failed: ${testPath} - ${(error as Error).message}`);
      failed.push(testPath);
    }
  };

  // Process in parallel batches
  for (let i = 0; i < curatedTests.length; i += CONCURRENT_REQUESTS) {
    const batch = curatedTests.slice(i, i + CONCURRENT_REQUESTS);
    await Promise.all(batch.map(processTest));
  }

  // Write manifest
  const manifest = downloaded.reduce(
    (acc, path) => {
      const [category, name] = path.split("/");
      const existing = acc.find((c) => c.category === category);
      if (existing) {
        existing.tests.push(name);
      } else {
        acc.push({ category, tests: [name] });
      }
      return acc;
    },
    [] as { category: string; tests: string[] }[]
  );

  await writeFile(
    join(FIXTURES_DIR, "manifest.json"),
    JSON.stringify(manifest, null, 2)
  );

  console.log(`\nDownloaded ${downloaded.length} fixtures`);

  if (failed.length > 0) {
    console.log(`Failed: ${failed.length} fixtures`);
  }

  // Show rate limit tip if no token
  if (!process.env.GITHUB_TOKEN) {
    console.log(
      `\nTip: Set GITHUB_TOKEN env var for higher rate limits (5000/hour vs 60/hour)`
    );
  }
}

// Parse command line arguments
const args = process.argv.slice(2);
const fullDownload = args.includes("--full");
const resume = args.includes("--resume");

if (args.includes("--help") || args.includes("-h")) {
  console.log(`
Usage: bun run fetch-fixtures.ts [options]

Options:
  --full     Download all test fixtures (default: curated list)
  --resume   Resume a previous incomplete download
  --help     Show this help message

Environment:
  GITHUB_TOKEN   GitHub personal access token for higher rate limits
                 (60 req/hour without, 5000 req/hour with token)

Examples:
  bun run fetch-fixtures.ts              # Download curated fixtures
  bun run fetch-fixtures.ts --full       # Download all fixtures
  bun run fetch-fixtures.ts --resume     # Resume incomplete full download
  GITHUB_TOKEN=xxx bun run fetch-fixtures.ts --full  # With auth
`);
  process.exit(0);
}

if (fullDownload || resume) {
  downloadFixtures(resume).catch((error) => {
    console.error(`\nFatal error: ${error.message}`);
    console.log("Run with --resume to continue from where it stopped");
    process.exit(1);
  });
} else {
  downloadCuratedFixtures().catch((error) => {
    console.error(`\nFatal error: ${error.message}`);
    process.exit(1);
  });
}
