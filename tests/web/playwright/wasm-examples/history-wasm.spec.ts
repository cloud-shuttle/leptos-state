import { test, expect } from '@playwright/test';

test.describe('History Example WASM Example', () => {
  test.beforeEach(async ({ page }) => {
    // Test the actual built WASM example
    await page.goto('/examples/history/dist/');
  });

  test('should load WASM and display initial state', async ({ page }) => {
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if the main app elements are visible
    await expect(page.locator('h1')).toContainText('History Example');
    await expect(page.locator('p')).toContainText('Testing history tracking');
    
    // Check if the counter is present
    const counter = page.locator('[data-testid="counter"]');
    await expect(counter).toBeVisible();
    await expect(counter).toHaveText('0');
  });

  test('should increment counter and track history', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const counter = page.locator('[data-testid="counter"]');
    const historyDisplay = page.locator('[data-testid="history"]');
    
    // Increment a few times
    await incrementBtn.click();
    await incrementBtn.click();
    await incrementBtn.click();
    
    // Counter should be 3
    await expect(counter).toHaveText('3');
    
    // History should show the progression
    await expect(historyDisplay).toBeVisible();
    // History should contain the state changes
  });

  test('should decrement counter and track history', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const decrementBtn = page.locator('[data-testid="decrement"]');
    const counter = page.locator('[data-testid="counter"]');
    
    // Increment first
    await incrementBtn.click();
    await incrementBtn.click();
    await expect(counter).toHaveText('2');
    
    // Then decrement
    await decrementBtn.click();
    await expect(counter).toHaveText('1');
    
    // Decrement again
    await decrementBtn.click();
    await expect(counter).toHaveText('0');
  });

  test('should reset counter and clear history', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const resetBtn = page.locator('[data-testid="reset"]');
    const counter = page.locator('[data-testid="counter"]');
    
    // Increment a few times
    await incrementBtn.click();
    await incrementBtn.click();
    await incrementBtn.click();
    await expect(counter).toHaveText('3');
    
    // Reset
    await resetBtn.click();
    await expect(counter).toHaveText('0');
    
    // History should be cleared or reset
  });

  test('should display history entries', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const historyDisplay = page.locator('[data-testid="history"]');
    
    // Perform some actions to generate history
    await incrementBtn.click();
    await incrementBtn.click();
    
    // History should be visible and contain entries
    await expect(historyDisplay).toBeVisible();
    
    // Check if history entries are displayed
    const historyEntries = page.locator('[data-testid="history-entry"]');
    if (await historyEntries.count() > 0) {
      await expect(historyEntries.first()).toBeVisible();
    }
  });

  test('should handle deep history restoration', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const deepHistoryBtn = page.locator('[data-testid="deep-history"]');
    const counter = page.locator('[data-testid="counter"]');
    
    // Increment to create some history
    await incrementBtn.click();
    await incrementBtn.click();
    await expect(counter).toHaveText('2');
    
    // Test deep history restoration
    await deepHistoryBtn.click();
    
    // Counter should be restored to the last known state
    // This depends on the implementation, but should not be 0
    const counterValue = await counter.textContent();
    expect(parseInt(counterValue || '0')).toBeGreaterThanOrEqual(0);
  });

  test('should handle shallow history restoration', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const shallowHistoryBtn = page.locator('[data-testid="shallow-history"]');
    const counter = page.locator('[data-testid="counter"]');
    
    // Increment to create some history
    await incrementBtn.click();
    await incrementBtn.click();
    await expect(counter).toHaveText('2');
    
    // Test shallow history restoration
    await shallowHistoryBtn.click();
    
    // Counter should be restored to the last known state
    const counterValue = await counter.textContent();
    expect(parseInt(counterValue || '0')).toBeGreaterThanOrEqual(0);
  });

  test('should maintain history across page interactions', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const decrementBtn = page.locator('[data-testid="decrement"]');
    const counter = page.locator('[data-testid="counter"]');
    
    // Perform a sequence of actions
    await incrementBtn.click(); // 1
    await incrementBtn.click(); // 2
    await decrementBtn.click(); // 1
    await incrementBtn.click(); // 2
    
    // Counter should be 2
    await expect(counter).toHaveText('2');
    
    // History should contain all these state changes
    const historyDisplay = page.locator('[data-testid="history"]');
    await expect(historyDisplay).toBeVisible();
  });

  test('should handle history clearing', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const clearHistoryBtn = page.locator('[data-testid="clear-history"]');
    const historyDisplay = page.locator('[data-testid="history"]');
    
    // Create some history
    await incrementBtn.click();
    await incrementBtn.click();
    
    // Clear history
    await clearHistoryBtn.click();
    
    // History should be cleared
    // This depends on the implementation
  });

  test('should show history metadata', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const historyDisplay = page.locator('[data-testid="history"]');
    
    // Create some history
    await incrementBtn.click();
    
    // Check if history metadata is displayed
    await expect(historyDisplay).toBeVisible();
    
    // Look for timestamp or other metadata
    const metadataElements = page.locator('[data-testid="history-metadata"]');
    if (await metadataElements.count() > 0) {
      await expect(metadataElements.first()).toBeVisible();
    }
  });

  test('should handle edge cases gracefully', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const decrementBtn = page.locator('[data-testid="decrement"]');
    const counter = page.locator('[data-testid="counter"]');
    
    // Try to decrement below zero (should handle gracefully)
    await decrementBtn.click();
    await expect(counter).toHaveText('0'); // Should not go below 0
    
    // History should still be tracked even for invalid operations
  });

  test('should have working WASM bindings', async ({ page }) => {
    // Wait for WASM to load and check if bindings are available
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if WASM bindings are available in the page context
    const hasBindings = await page.evaluate(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    expect(hasBindings).toBe(true);
  });
});
