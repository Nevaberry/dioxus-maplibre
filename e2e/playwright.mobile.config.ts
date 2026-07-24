import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './mobile-tests',
  timeout: 60_000,
  retries: 1,
  fullyParallel: false,
  use: {
    baseURL: 'http://localhost:8081',
    headless: true,
    ...devices['Pixel 7'],
  },
  projects: [
    {
      name: 'mobile-chromium',
      use: { browserName: 'chromium' },
    },
  ],
  webServer: {
    command: 'cd ../examples/showcase-mobile && dx bundle --web --release --debug-symbols=false --out-dir dist --locked && cd ../../e2e && bun run serve:mobile',
    port: 8081,
    timeout: 120_000,
    reuseExistingServer: true,
  },
});
