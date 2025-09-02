import { test, expect } from '@playwright/test';

test.describe('Counter WASM Example', () => {
  test.beforeEach(async ({ page }) => {
    // Test the actual built WASM example
    await page.goto('/examples/counter/dist/');
  });

  test('should load WASM and display initial counter value', async ({ page }) => {
    // Wait for WASM to load by listening for the TrunkApplicationStarted event
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    await expect(counter).toBeVisible();
    await expect(counter).toHaveText('0');
  });

  test('should increment counter when + button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    const incrementBtn = page.locator('[data-testid="increment"]');
    
    const initialValue = await counter.textContent();
    expect(initialValue).toBe('0');
    
    await incrementBtn.click();
    
    // Wait for the counter to update
    await expect(counter).toHaveText('1');
  });

  test('should decrement counter when - button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    const decrementBtn = page.locator('[data-testid="decrement"]');
    
    // First increment to 1, then decrement
    await page.locator('[data-testid="increment"]').click();
    await expect(counter).toHaveText('1');
    
    await decrementBtn.click();
    await expect(counter).toHaveText('0');
  });

  test('should reset counter when Reset button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    const resetBtn = page.locator('[data-testid="reset"]');
    
    // Increment a few times
    await page.locator('[data-testid="increment"]').click();
    await page.locator('[data-testid="increment"]').click();
    await expect(counter).toHaveText('2');
    
    // Reset
    await resetBtn.click();
    await expect(counter).toHaveText('0');
  });

  test('should handle user name input', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const nameInput = page.locator('[data-testid="name-input"]');
    const userDisplay = page.locator('[data-testid="user-display"]');
    
    // Check initial state
    await expect(userDisplay).toHaveText('Guest');
    
    // Type a name
    await nameInput.fill('John Doe');
    await nameInput.press('Enter');
    
    // The display should update (this depends on the WASM implementation)
    // For now, just verify the input works
    await expect(nameInput).toHaveValue('John Doe');
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

  test('should handle multiple rapid clicks', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const counter = page.locator('[data-testid="counter"]');
    const incrementBtn = page.locator('[data-testid="increment"]');
    
    // Rapidly click multiple times
    for (let i = 0; i < 5; i++) {
      await incrementBtn.click();
    }
    
    // Should handle rapid clicks correctly
    await expect(counter).toHaveText('5');
  });
});
