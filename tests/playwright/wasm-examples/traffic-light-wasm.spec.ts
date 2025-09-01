import { test, expect } from '@playwright/test';

test.describe('Traffic Light WASM Example', () => {
  test.beforeEach(async ({ page }) => {
    // Test the actual built WASM example
    await page.goto('/examples/traffic-light/dist/');
  });

  test('should load WASM and display initial state', async ({ page }) => {
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const stateDisplay = page.locator('[data-testid="current-state"]');
    await expect(stateDisplay).toBeVisible();
    
    // Check if it shows a valid traffic light state
    const stateText = await stateDisplay.textContent();
    expect(['Red', 'Yellow', 'Green']).toContain(stateText);
  });

  test('should transition states when timer button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const timerBtn = page.locator('[data-testid="timer"]');
    const stateDisplay = page.locator('[data-testid="current-state"]');
    
    const initialState = await stateDisplay.textContent();
    expect(['Red', 'Yellow', 'Green']).toContain(initialState);
    
    await timerBtn.click();
    
    // Wait for state transition and check if it changed
    await page.waitForTimeout(100); // Small delay for state transition
    
    const newState = await stateDisplay.textContent();
    expect(newState).not.toBe(initialState);
    expect(['Red', 'Yellow', 'Green']).toContain(newState);
  });

  test('should handle pedestrian request', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const pedestrianBtn = page.locator('[data-testid="pedestrian"]');
    const pedestrianStatus = page.locator('[data-testid="pedestrian-waiting"]');
    
    // Check initial state
    const initialStatus = await pedestrianStatus.textContent();
    
    await pedestrianBtn.click();
    
    // Wait for status change and verify it's different
    await page.waitForTimeout(100);
    const newStatus = await pedestrianStatus.textContent();
    expect(newStatus).not.toBe(initialStatus);
  });

  test('should handle emergency stop', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const emergencyBtn = page.locator('[data-testid="emergency"]');
    const stateDisplay = page.locator('[data-testid="current-state"]');
    
    // Click emergency button
    await emergencyBtn.click();
    
    // Wait for emergency mode to activate
    await page.waitForTimeout(100);
    
    // Emergency should force red state
    const emergencyState = await stateDisplay.textContent();
    expect(emergencyState).toBe('Red');
  });

  test('should reset to initial state', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const resetBtn = page.locator('[data-testid="reset"]');
    const stateDisplay = page.locator('[data-testid="current-state"]');
    
    const initialState = await stateDisplay.textContent();
    
    // Change state first
    await page.locator('[data-testid="timer"]').click();
    await page.waitForTimeout(100);
    
    // Reset
    await resetBtn.click();
    await page.waitForTimeout(100);
    
    // Should be back to initial state
    const resetState = await stateDisplay.textContent();
    expect(resetState).toBe(initialState);
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

  test('should handle multiple rapid timer clicks', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const timerBtn = page.locator('[data-testid="timer"]');
    const stateDisplay = page.locator('[data-testid="current-state"]');
    
    const initialState = await stateDisplay.textContent();
    
    // Rapidly click timer multiple times
    for (let i = 0; i < 3; i++) {
      await timerBtn.click();
      await page.waitForTimeout(50);
    }
    
    // Should handle rapid clicks without crashing
    const finalState = await stateDisplay.textContent();
    expect(['Red', 'Yellow', 'Green']).toContain(finalState);
  });

  test('should maintain state consistency across interactions', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const stateDisplay = page.locator('[data-testid="current-state"]');
    const timerBtn = page.locator('[data-testid="timer"]');
    const emergencyBtn = page.locator('[data-testid="emergency"]');
    const resetBtn = page.locator('[data-testid="reset"]');
    
    // Get initial state
    const initialState = await stateDisplay.textContent();
    
    // Perform a sequence of actions
    await timerBtn.click();
    await page.waitForTimeout(100);
    
    await emergencyBtn.click();
    await page.waitForTimeout(100);
    
    await resetBtn.click();
    await page.waitForTimeout(100);
    
    // Should be back to initial state
    const finalState = await stateDisplay.textContent();
    expect(finalState).toBe(initialState);
  });
});
