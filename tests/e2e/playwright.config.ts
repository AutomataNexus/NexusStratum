import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: '.',
  timeout: 30000,
  retries: 0,
  use: {
    baseURL: 'https://stratum-ui.com',
    screenshot: 'only-on-failure',
  },
  projects: [
    { name: 'chromium', use: { browserName: 'chromium', headless: true, launchOptions: { executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || undefined } } },
  ],
});
