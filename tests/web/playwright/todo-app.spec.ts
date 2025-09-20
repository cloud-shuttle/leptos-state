// Todo App Example Playwright Tests
// Following ADR-005: Playwright Testing Strategy

import { test, expect } from '@playwright/test';

test.describe('Todo App Example', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/todo-app');
  });

  test('should display empty todo list initially', async ({ page }) => {
    await expect(page.locator('[data-testid="todo-list"]')).toBeEmpty();
  });

  test('should add new todo item', async ({ page }) => {
    await page.fill('[data-testid="todo-input"]', 'Test todo item');
    await page.click('[data-testid="add-todo-button"]');
    
    await expect(page.locator('[data-testid="todo-list"]')).toContainText('Test todo item');
  });

  test('should mark todo as completed', async ({ page }) => {
    await page.fill('[data-testid="todo-input"]', 'Test todo item');
    await page.click('[data-testid="add-todo-button"]');
    
    await page.click('[data-testid="todo-checkbox-0"]');
    await expect(page.locator('[data-testid="todo-item-0"]')).toHaveClass(/completed/);
  });

  test('should delete todo item', async ({ page }) => {
    await page.fill('[data-testid="todo-input"]', 'Test todo item');
    await page.click('[data-testid="add-todo-button"]');
    
    await page.click('[data-testid="delete-todo-button-0"]');
    await expect(page.locator('[data-testid="todo-list"]')).toBeEmpty();
  });

  test('should filter todos by status', async ({ page }) => {
    // Add multiple todos
    await page.fill('[data-testid="todo-input"]', 'Todo 1');
    await page.click('[data-testid="add-todo-button"]');
    
    await page.fill('[data-testid="todo-input"]', 'Todo 2');
    await page.click('[data-testid="add-todo-button"]');
    
    // Mark one as completed
    await page.click('[data-testid="todo-checkbox-0"]');
    
    // Filter by completed
    await page.click('[data-testid="filter-completed"]');
    await expect(page.locator('[data-testid="todo-list"]')).toContainText('Todo 1');
    await expect(page.locator('[data-testid="todo-list"]')).not.toContainText('Todo 2');
  });
});

