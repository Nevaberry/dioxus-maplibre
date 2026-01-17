/**
 * Clean up downloaded MapLibre test fixtures
 */

import { rm } from "fs/promises";
import { join } from "path";

const FIXTURES_DIR = join(import.meta.dir, "..", "fixtures");
const PROGRESS_FILE = join(FIXTURES_DIR, ".progress.json");

async function clean() {
  console.log("Cleaning up fixtures...");

  try {
    await rm(FIXTURES_DIR, { recursive: true, force: true });
    console.log("  Removed fixtures/");
  } catch {
    console.log("  fixtures/ not found");
  }

  console.log("Done.");
}

clean().catch(console.error);
