// Counter Example Playwright Tests
// Following ADR-005: Playwright Testing Strategy

import { test, expect } from '@playwright/test';

test.describe('Counter Example', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/counter');
  });

  test('should display initial counter value', async ({ page }) => {
    await expect(page.locator('[data-testid="counter-value"]')).toHaveText('0');
  });

  test('should increment counter when increment button is clicked', async ({ page }) => {
    await page.click('[data-testid="increment-button"]');
    await expect(page.locator('[data-testid="counter-value"]')).toHaveText('1');
  });

  test('should decrement counter when decrement button is clicked', async ({ page }) => {
    await page.click('[data-testid="increment-button"]');
    await page.click('[data-testid="decrement-button"]');
    await expect(page.locator('[data-testid="counter-value"]')).toHaveText('0');
  });

  test('should reset counter when reset button is clicked', async ({ page }) => {
    await page.click('[data-testid="increment-button"]');
    await page.click('[data-testid="increment-button"]');
    await page.click('[data-testid="reset-button"]');
    await expect(page.locator('[data-testid="counter-value"]')).toHaveText('0');
  });

  test('should handle multiple rapid clicks', async ({ page }) => {
    for (let i = 0; i < 5; i++) {
      await page.click('[data-testid="increment-button"]');
    }
    await expect(page.locator('[data-testid="counter-value"]')).toHaveText('5');
  });
});

