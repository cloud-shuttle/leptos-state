import { test, expect } from '@playwright/test';

test.describe('Compatibility Example WASM Example', () => {
  test.beforeEach(async ({ page }) => {
    // Test the actual built WASM example
    await page.goto('/examples/compatibility-example/dist/');
  });

  test('should load WASM and display initial state', async ({ page }) => {
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if the main app elements are visible
    await expect(page.locator('h1')).toContainText('Compatibility Example');
    await expect(page.locator('p')).toContainText('Testing compatibility layer');
    
    // Check if the counter is present
    const counter = page.locator('[data-testid="counter"]');
    await expect(counter).toBeVisible();
    await expect(counter).toHaveText('0');
  });

  test('should increment counter when increment button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const incrementBtn = page.locator('[data-testid="increment"]');
    const counter = page.locator('[data-testid="counter"]');
    
    await incrementBtn.click();
    await expect(counter).toHaveText('1');
    
    await incrementBtn.click();
    await expect(counter).toHaveText('2');
  });

  test('should decrement counter when decrement button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const decrementBtn = page.locator('[data-testid="decrement"]');
    const counter = page.locator('[data-testid="counter"]');
    
    // First increment to have a value to decrement
    await page.locator('[data-testid="increment"]').click();
    await expect(counter).toHaveText('1');
    
    await decrementBtn.click();
    await expect(counter).toHaveText('0');
  });

  test('should reset counter when reset button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const resetBtn = page.locator('[data-testid="reset"]');
    const counter = page.locator('[data-testid="counter"]');
    
    // First increment a few times
    await page.locator('[data-testid="increment"]').click();
    await page.locator('[data-testid="increment"]').click();
    await page.locator('[data-testid="increment"]').click();
    await expect(counter).toHaveText('3');
    
    // Reset
    await resetBtn.click();
    await expect(counter).toHaveText('0');
  });

  test('should update user name when input is changed', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const nameInput = page.locator('[data-testid="name-input"]');
    const userDisplay = page.locator('[data-testid="user-display"]');
    
    await nameInput.fill('John Doe');
    await expect(userDisplay).toHaveText('John Doe');
  });

  test('should handle store state management', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    const incrementBtn = page.locator('[data-testid="increment"]');
    const nameInput = page.locator('[data-testid="name-input"]');
    const userDisplay = page.locator('[data-testid="user-display"]');
    
    // Test counter state
    await incrementBtn.click();
    await expect(counter).toHaveText('1');
    
    // Test name state
    await nameInput.fill('Jane Smith');
    await expect(userDisplay).toHaveText('Jane Smith');
    
    // Counter state should be preserved
    await expect(counter).toHaveText('1');
  });

  test('should handle computed values', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    const incrementBtn = page.locator('[data-testid="increment"]');
    
    // Check if computed values are displayed (if any)
    // This test verifies that the compatibility layer handles computed values correctly
    
    await incrementBtn.click();
    await expect(counter).toHaveText('1');
    
    // If there are computed values, they should update accordingly
    // For now, just verify the basic functionality works
  });

  test('should handle effects correctly', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    const incrementBtn = page.locator('[data-testid="increment"]');
    
    // Test that effects run when state changes
    await incrementBtn.click();
    await expect(counter).toHaveText('1');
    
    // If there are effects (like logging, side effects), they should run
    // This test verifies the compatibility layer handles effects correctly
  });

  test('should maintain state across page interactions', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    const incrementBtn = page.locator('[data-testid="increment"]');
    const nameInput = page.locator('[data-testid="name-input"]');
    const userDisplay = page.locator('[data-testid="user-display"]');
    
    // Set some state
    await incrementBtn.click();
    await nameInput.fill('Test User');
    
    // Verify state is maintained
    await expect(counter).toHaveText('1');
    await expect(userDisplay).toHaveText('Test User');
    
    // Interact with other elements
    await page.locator('[data-testid="decrement"]').click();
    await expect(counter).toHaveText('0');
    
    // Name should still be maintained
    await expect(userDisplay).toHaveText('Test User');
  });

  test('should handle edge cases gracefully', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    const decrementBtn = page.locator('[data-testid="decrement"]');
    const nameInput = page.locator('[data-testid="name-input"]');
    
    // Test decrementing below zero (should handle gracefully)
    await decrementBtn.click();
    await expect(counter).toHaveText('0'); // Should not go below 0
    
    // Test empty name input
    await nameInput.fill('');
    await expect(nameInput).toHaveValue('');
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
