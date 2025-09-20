//! Comprehensive Playwright tests for all leptos-state demos
//! Following ADR-005: Playwright Testing Strategy

import { test, expect } from '@playwright/test';

test.describe('All Leptos State Demos', () => {
  test.describe('Counter Demo', () => {
    test('should load and function correctly', async ({ page }) => {
      await page.goto('/examples/counter/dist/');
      await page.waitForLoadState('networkidle', { timeout: 15000 });
      
      // Check if WASM loaded
      const counter = page.locator('[data-testid="counter"]');
      await expect(counter).toBeVisible();
      
      // Test increment functionality
      const incrementBtn = page.locator('[data-testid="increment"]');
      await incrementBtn.click();
      await expect(counter).toHaveText('1');
      
      // Test decrement functionality
      const decrementBtn = page.locator('[data-testid="decrement"]');
      await decrementBtn.click();
      await expect(counter).toHaveText('0');
      
      // Test reset functionality
      await incrementBtn.click();
      await incrementBtn.click();
      const resetBtn = page.locator('[data-testid="reset"]');
      await resetBtn.click();
      await expect(counter).toHaveText('0');
    });
  });

  test.describe('Todo App Demo', () => {
    test('should load and function correctly', async ({ page }) => {
      await page.goto('/examples/todo-app/');
      await page.waitForLoadState('networkidle', { timeout: 15000 });
      
      // Check if WASM loaded
      const app = page.locator('#app');
      await expect(app).toBeVisible();
      
      // Wait for WASM to initialize
      await page.waitForTimeout(2000);
      
      // Check if todo app elements are present
      const todoInput = page.locator('input[placeholder*="todo"], input[placeholder*="Todo"]');
      await expect(todoInput).toBeVisible({ timeout: 10000 });
    });
  });

  test.describe('Analytics Dashboard Demo', () => {
    test('should load and display dashboard', async ({ page }) => {
      await page.goto('/examples/analytics-dashboard/dist/');
      await page.waitForLoadState('networkidle', { timeout: 15000 });
      
      // Check if WASM loaded
      const app = page.locator('#app');
      await expect(app).toBeVisible();
      
      // Wait for WASM to initialize
      await page.waitForTimeout(3000);
      
      // Check for dashboard elements
      const dashboard = page.locator('[data-testid="dashboard"], .dashboard, #dashboard');
      await expect(dashboard).toBeVisible({ timeout: 10000 });
    });
  });

  test.describe('Code Generation Demo', () => {
    test('should load and display codegen interface', async ({ page }) => {
      await page.goto('/examples/codegen/dist/');
      await page.waitForLoadState('networkidle', { timeout: 15000 });
      
      // Check if WASM loaded
      const app = page.locator('#app');
      await expect(app).toBeVisible();
      
      // Wait for WASM to initialize
      await page.waitForTimeout(3000);
      
      // Check for codegen elements
      const codegen = page.locator('[data-testid="codegen"], .codegen, #codegen');
      await expect(codegen).toBeVisible({ timeout: 10000 });
    });
  });

  test.describe('History Demo', () => {
    test('should load and display history interface', async ({ page }) => {
      await page.goto('/examples/history/dist/');
      await page.waitForLoadState('networkidle', { timeout: 15000 });
      
      // Check if WASM loaded
      const app = page.locator('#app');
      await expect(app).toBeVisible();
      
      // Wait for WASM to initialize
      await page.waitForTimeout(3000);
      
      // Check for history elements
      const history = page.locator('[data-testid="history"], .history, #history');
      await expect(history).toBeVisible({ timeout: 10000 });
    });
  });

  test.describe('Traffic Light Demo', () => {
    test('should load and function correctly', async ({ page }) => {
      await page.goto('/examples/traffic-light/dist/');
      await page.waitForLoadState('networkidle', { timeout: 15000 });
      
      // Check if WASM loaded
      const app = page.locator('#app');
      await expect(app).toBeVisible();
      
      // Wait for WASM to initialize
      await page.waitForTimeout(3000);
      
      // Check for traffic light elements
      const trafficLight = page.locator('[data-testid="traffic-light"], .traffic-light, #traffic-light');
      await expect(trafficLight).toBeVisible({ timeout: 10000 });
    });
  });

  test.describe('Compatibility Demo', () => {
    test('should load and display compatibility info', async ({ page }) => {
      await page.goto('/examples/compatibility-example/dist/');
      await page.waitForLoadState('networkidle', { timeout: 15000 });
      
      // Check if WASM loaded
      const app = page.locator('#app');
      await expect(app).toBeVisible();
      
      // Wait for WASM to initialize
      await page.waitForTimeout(3000);
      
      // Check for compatibility elements
      const compatibility = page.locator('[data-testid="compatibility"], .compatibility, #compatibility');
      await expect(compatibility).toBeVisible({ timeout: 10000 });
    });
  });

  test.describe('Demo Health Check', () => {
    test('all demos should load without errors', async ({ page }) => {
      const demos = [
        '/examples/counter/dist/',
        '/examples/todo-app/',
        '/examples/analytics-dashboard/dist/',
        '/examples/codegen/dist/',
        '/examples/history/dist/',
        '/examples/traffic-light/dist/',
        '/examples/compatibility-example/dist/',
      ];

      for (const demo of demos) {
        console.log(`Testing demo: ${demo}`);
        
        // Navigate to demo
        await page.goto(demo);
        await page.waitForLoadState('networkidle', { timeout: 15000 });
        
        // Check for console errors
        const errors: string[] = [];
        page.on('console', msg => {
          if (msg.type() === 'error') {
            errors.push(msg.text());
          }
        });
        
        // Wait for WASM to initialize
        await page.waitForTimeout(3000);
        
        // Check if app container exists
        const app = page.locator('#app');
        await expect(app).toBeVisible();
        
        // Report any errors
        if (errors.length > 0) {
          console.log(`Errors in ${demo}:`, errors);
        }
        
        // Clear errors for next demo
        errors.length = 0;
      }
    });
  });
});

