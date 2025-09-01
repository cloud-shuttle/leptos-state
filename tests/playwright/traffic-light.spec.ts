import { test, expect } from '@playwright/test';

test.describe('Traffic Light Example', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/test-pages/traffic-light.html');
  });

  test('should display initial state', async ({ page }) => {
    const stateDisplay = page.locator('[data-testid="current-state-display"]');
    await expect(stateDisplay).toBeVisible();
  });

  test('should transition states when timer button is clicked', async ({ page }) => {
    const timerBtn = page.locator('[data-testid="timer"]');
    const stateDisplay = page.locator('[data-testid="current-state-display"]');
    
    // Get initial state
    const initialState = await stateDisplay.textContent();
    expect(initialState).toBe('Red');
    
    // Click timer to advance state
    await timerBtn.click();
    
    // State should have changed to Yellow
    const newState = await stateDisplay.textContent();
    expect(newState).toBe('Yellow');
  });

  test('should handle pedestrian request', async ({ page }) => {
    const pedestrianBtn = page.locator('[data-testid="pedestrian"]');
    const pedestrianStatus = page.locator('[data-testid="pedestrian-waiting"]');
    
    await pedestrianBtn.click();
    await expect(pedestrianStatus).toContainText('Yes');
  });

  test('should handle emergency stop', async ({ page }) => {
    const emergencyBtn = page.locator('[data-testid="emergency"]');
    const stateDisplay = page.locator('[data-testid="current-state-display"]');
    
    await emergencyBtn.click();
    await expect(stateDisplay).toContainText('Red');
  });

  test('should reset to initial state', async ({ page }) => {
    const resetBtn = page.locator('[data-testid="reset"]');
    const stateDisplay = page.locator('[data-testid="current-state-display"]');
    
    // Get initial state
    const initialState = await stateDisplay.textContent();
    
    // Change state first
    await page.locator('[data-testid="timer"]').click();
    
    // Reset
    await resetBtn.click();
    
    // Should be back to initial state
    await expect(stateDisplay).toHaveText(initialState);
  });
});
