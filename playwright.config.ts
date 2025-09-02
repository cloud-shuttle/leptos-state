import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/playwright',
  fullyParallel: false, // Disable parallel to prevent hanging
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1, // Force single worker to prevent hanging
  reporter: 'line', // Use line reporter for immediate feedback
  timeout: 30000, // 30 second timeout
  expect: {
    timeout: 10000, // 10 second expect timeout
  },
  use: {
    baseURL: 'http://localhost:8000',
    trace: 'off', // Disable tracing to prevent hanging
    screenshot: 'only-on-failure',
    actionTimeout: 10000, // 10 second action timeout
    navigationTimeout: 15000, // 15 second navigation timeout
  },

  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
  ],

  webServer: {
    command: 'python3 -m http.server 8000 --directory .',
    url: 'http://localhost:8000',
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
});
