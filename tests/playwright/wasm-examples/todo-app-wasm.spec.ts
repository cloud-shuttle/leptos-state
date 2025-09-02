import { test, expect } from '@playwright/test';

test.describe('Todo App WASM Example', () => {
  test.beforeEach(async ({ page }) => {
    // Test the actual built WASM example
    await page.goto('/examples/todo-app/dist/');
  });

  test('should load WASM and display initial state', async ({ page }) => {
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if the main app elements are visible
    await expect(page.locator('h1')).toContainText('Simple Todo App');
    await expect(page.locator('p')).toContainText('Built with Leptos and Rust');
    
    // Check if the form is present
    const input = page.locator('input[type="text"]');
    const submitBtn = page.locator('button[type="submit"]');
    
    await expect(input).toBeVisible();
    await expect(submitBtn).toBeVisible();
    await expect(submitBtn).toContainText('Add Todo');
  });

  test('should add new todos', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const input = page.locator('input[type="text"]');
    const submitBtn = page.locator('button[type="submit"]');
    
    // Add first todo
    await input.fill('Buy groceries');
    await submitBtn.click();
    
    // Check if todo was added
    await expect(page.locator('text=Buy groceries')).toBeVisible();
    
    // Add second todo
    await input.fill('Walk the dog');
    await submitBtn.click();
    
    // Check if second todo was added
    await expect(page.locator('text=Walk the dog')).toBeVisible();
    
    // Check if input was cleared
    await expect(input).toHaveValue('');
  });

  test('should not add empty todos', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const input = page.locator('input[type="text"]');
    const submitBtn = page.locator('button[type="submit"]');
    
    // Try to add empty todo
    await input.fill('');
    await submitBtn.click();
    
    // Check that no empty todo was added
    const todoItems = page.locator('div[style*="display: flex; align-items: center"]');
    await expect(todoItems).toHaveCount(0);
  });

  test('should not add whitespace-only todos', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const input = page.locator('input[type="text"]');
    const submitBtn = page.locator('button[type="submit"]');
    
    // Try to add whitespace-only todo
    await input.fill('   ');
    await submitBtn.click();
    
    // Check that no whitespace-only todo was added
    const todoItems = page.locator('div[style*="display: flex; align-items: center"]');
    await expect(todoItems).toHaveCount(0);
  });

  test('should toggle todo completion status', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const input = page.locator('input[type="text"]');
    const submitBtn = page.locator('button[type="submit"]');
    
    // Add a todo
    await input.fill('Test completion');
    await submitBtn.click();
    
    // Find the checkbox for this todo
    const checkbox = page.locator('input[type="checkbox"]').first();
    const todoText = page.locator('text=Test completion');
    
    // Initially should not be completed
    await expect(checkbox).not.toBeChecked();
    
    // Click checkbox to complete
    await checkbox.click();
    await expect(checkbox).toBeChecked();
    
    // Text should be strikethrough
    await expect(todoText).toHaveCSS('text-decoration', 'line-through solid rgb(100, 116, 139)');
    
    // Click again to uncomplete
    await checkbox.click();
    await expect(checkbox).not.toBeChecked();
    
    // Text should not be strikethrough
    await expect(todoText).not.toHaveCSS('text-decoration', 'line-through solid rgb(100, 116, 139)');
  });

  test('should delete todos', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const input = page.locator('input[type="text"]');
    const submitBtn = page.locator('button[type="submit"]');
    
    // Add a todo
    await input.fill('Todo to delete');
    await submitBtn.click();
    
    // Verify it was added
    await expect(page.locator('text=Todo to delete')).toBeVisible();
    
    // Find and click the delete button
    const deleteBtn = page.locator('button:has-text("Delete")').first();
    await deleteBtn.click();
    
    // Verify it was deleted
    await expect(page.locator('text=Todo to delete')).not.toBeVisible();
  });

  test('should handle multiple todos correctly', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const input = page.locator('input[type="text"]');
    const submitBtn = page.locator('button[type="submit"]');
    
    // Add multiple todos
    const todos = ['First todo', 'Second todo', 'Third todo'];
    
    for (const todo of todos) {
      await input.fill(todo);
      await submitBtn.click();
    }
    
    // Verify all todos are present
    for (const todo of todos) {
      await expect(page.locator(`text=${todo}`)).toBeVisible();
    }
    
    // Check that we have the right number of todo items
    const todoItems = page.locator('div[style*="display: flex; align-items: center"]');
    await expect(todoItems).toHaveCount(3);
  });

  test('should maintain todo state across interactions', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const input = page.locator('input[type="text"]');
    const submitBtn = page.locator('button[type="submit"]');
    
    // Add a todo
    await input.fill('Persistent todo');
    await submitBtn.click();
    
    // Complete it
    const checkbox = page.locator('input[type="checkbox"]').first();
    await checkbox.click();
    await expect(checkbox).toBeChecked();
    
    // Add another todo
    await input.fill('Another todo');
    await submitBtn.click();
    
    // First todo should still be completed
    await expect(checkbox).toBeChecked();
    
    // Second todo should not be completed
    const secondCheckbox = page.locator('input[type="checkbox"]').nth(1);
    await expect(secondCheckbox).not.toBeChecked();
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
