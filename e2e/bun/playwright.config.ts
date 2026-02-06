import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  timeout: 600_000, // 10 minutes for all fixtures
  expect: { timeout: 10_000 },
  fullyParallel: false,
  workers: 1, // Single worker for WebGL stability
  retries: 0,
  reporter: [["list"], ["json", { outputFile: "results/test-results.json" }]],
  use: {
    baseURL: "http://localhost:3900",
    launchOptions: {
      args: [
        "--use-gl=angle",
        "--use-angle=swiftshader",
        "--enable-unsafe-swiftshader",
        "--disable-gpu-sandbox",
        "--disable-setuid-sandbox",
        "--no-sandbox",
      ],
    },
  },
  projects: [
    {
      name: "chromium",
      use: {
        browserName: "chromium",
        viewport: { width: 1024, height: 1024 },
      },
    },
  ],
  webServer: {
    command: "bun run src/server.ts",
    port: 3900,
    reuseExistingServer: !process.env.CI,
    timeout: 10_000,
  },
});
