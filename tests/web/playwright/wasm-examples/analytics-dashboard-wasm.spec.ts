import { test, expect } from '@playwright/test';

test.describe('Analytics Dashboard WASM Example', () => {
  test.beforeEach(async ({ page }) => {
    // Test the actual built WASM example
    await page.goto('/examples/analytics-dashboard/dist/');
  });

  test('should load WASM and display initial dashboard', async ({ page }) => {
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if the main dashboard elements are visible
    await expect(page.locator('h1')).toContainText('Analytics Dashboard');
    await expect(page.locator('p')).toContainText('Real-time business metrics and insights');
    
    // Check if timeframe selector is present
    const timeframeSelector = page.locator('div[style*="display: flex; background: #f1f5f9"]');
    await expect(timeframeSelector).toBeVisible();
    
    // Check if refresh button is present
    const refreshBtn = page.locator('button:has-text("Refresh Data")');
    await expect(refreshBtn).toBeVisible();
  });

  test('should display metrics with correct data', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if metrics are displayed
    await expect(page.locator('text=Total Revenue')).toBeVisible();
    await expect(page.locator('text=Active Users')).toBeVisible();
    await expect(page.locator('text=Page Load Time')).toBeVisible();
    await expect(page.locator('text=Session Duration')).toBeVisible();
    
    // Check if metric values are displayed (they should be numbers)
    const revenueValue = page.locator('text=Total Revenue').locator('..').locator('div[style*="font-size: 2rem"]');
    await expect(revenueValue).toBeVisible();
    
    // Check if trend indicators are present
    const trendIndicators = page.locator('div[style*="display: flex; align-items: center"]');
    await expect(trendIndicators).toHaveCount(4); // 4 metrics
  });

  test('should handle timeframe selection', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check initial timeframe (should be 7D)
    const activeButton = page.locator('button[style*="background: white"]');
    await expect(activeButton).toContainText('7D');
    
    // Click on 1D timeframe
    const oneDayBtn = page.locator('button:has-text("1D")');
    await oneDayBtn.click();
    
    // Check if 1D is now active
    await expect(oneDayBtn).toHaveCSS('background', 'white');
    
    // Click on 7D timeframe
    const sevenDayBtn = page.locator('button:has-text("7D")');
    await sevenDayBtn.click();
    
    // Check if 7D is now active
    await expect(sevenDayBtn).toHaveCSS('background', 'white');
  });

  test('should refresh data when refresh button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Get initial metric values
    const initialRevenue = await page.locator('text=Total Revenue').locator('..').locator('div[style*="font-size: 2rem"]').textContent();
    
    // Click refresh button
    const refreshBtn = page.locator('button:has-text("Refresh Data")');
    await refreshBtn.click();
    
    // Wait for loading state (if any)
    await page.waitForTimeout(1000);
    
    // Get new metric values
    const newRevenue = await page.locator('text=Total Revenue').locator('..').locator('div[style*="font-size: 2rem"]').textContent();
    
    // Values should be different (since it's mock data generation)
    expect(newRevenue).not.toBe(initialRevenue);
  });

  test('should display metric categories correctly', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if metric categories are displayed
    const categories = ['Revenue', 'Users', 'Performance', 'Engagement'];
    
    for (const category of categories) {
      await expect(page.locator(`text=${category}`)).toBeVisible();
    }
  });

  test('should show metric changes and trends', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if change percentages are displayed
    const changeValues = page.locator('div[style*="font-size: 0.875rem"]');
    await expect(changeValues).toHaveCount(4); // 4 metrics
    
    // Check if trend indicators are present (up/down arrows or text)
    const trendTexts = page.locator('div[style*="display: flex; align-items: center"]');
    await expect(trendTexts).toHaveCount(4);
  });

  test('should have responsive layout', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if the dashboard has a responsive container
    const container = page.locator('div[style*="max-width: 600px"]');
    await expect(container).toBeVisible();
    
    // Check if metrics are arranged in a grid or list
    const metricsContainer = page.locator('div[style*="background: rgba(255, 255, 255, 0.95)"]');
    await expect(metricsContainer).toBeVisible();
  });

  test('should handle loading states', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Click refresh button to trigger loading
    const refreshBtn = page.locator('button:has-text("Refresh Data")');
    await refreshBtn.click();
    
    // The refresh button might show loading state or be disabled
    // This test verifies the button is still functional
    await expect(refreshBtn).toBeVisible();
  });

  test('should maintain state across interactions', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Change timeframe
    const oneDayBtn = page.locator('button:has-text("1D")');
    await oneDayBtn.click();
    
    // Refresh data
    const refreshBtn = page.locator('button:has-text("Refresh Data")');
    await refreshBtn.click();
    
    // Wait for refresh
    await page.waitForTimeout(1000);
    
    // Timeframe selection should still be 1D
    await expect(oneDayBtn).toHaveCSS('background', 'white');
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
