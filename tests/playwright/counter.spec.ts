import { test, expect } from '@playwright/test';

test.describe('Counter Example', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/test-pages/counter.html');
  });

  test('should display initial counter value', async ({ page }) => {
    const counter = page.locator('[data-testid="counter"]');
    await expect(counter).toHaveText('0');
  });

  test('should increment counter when increment button is clicked', async ({ page }) => {
    const incrementBtn = page.locator('[data-testid="increment"]');
    const counter = page.locator('[data-testid="counter"]');
    
    await incrementBtn.click();
    await expect(counter).toHaveText('1');
    
    await incrementBtn.click();
    await expect(counter).toHaveText('2');
  });

  test('should decrement counter when decrement button is clicked', async ({ page }) => {
    const decrementBtn = page.locator('[data-testid="decrement"]');
    const counter = page.locator('[data-testid="counter"]');
    
    // First increment to have a value to decrement
    await page.locator('[data-testid="increment"]').click();
    await expect(counter).toHaveText('1');
    
    await decrementBtn.click();
    await expect(counter).toHaveText('0');
  });

  test('should reset counter when reset button is clicked', async ({ page }) => {
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
    const nameInput = page.locator('[data-testid="name-input"]');
    const userDisplay = page.locator('[data-testid="user-display"]');
    
    await nameInput.fill('John Doe');
    await expect(userDisplay).toHaveText('John Doe');
  });
});
