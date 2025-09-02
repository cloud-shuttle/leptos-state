import { test, expect } from '@playwright/test';

test.describe('Codegen Example WASM Example', () => {
  test.beforeEach(async ({ page }) => {
    // Test the actual built WASM example
    await page.goto('/examples/codegen/dist/');
  });

  test('should load WASM and display initial state', async ({ page }) => {
    // Wait for WASM to load
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if the main app elements are visible
    await expect(page.locator('h1')).toContainText('State Machine Code Generation Example');
    
    // Check if the game state is displayed
    const stateDisplay = page.locator('[data-testid="game-state"]');
    await expect(stateDisplay).toBeVisible();
    
    // Initial state should be "idle"
    await expect(stateDisplay).toContainText('idle');
  });

  test('should start game when start button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const startBtn = page.locator('[data-testid="start-game"]');
    const stateDisplay = page.locator('[data-testid="game-state"]');
    
    // Click start button
    await startBtn.click();
    
    // State should change to "playing"
    await expect(stateDisplay).toContainText('playing');
  });

  test('should pause game when pause button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const startBtn = page.locator('[data-testid="start-game"]');
    const pauseBtn = page.locator('[data-testid="pause-game"]');
    const stateDisplay = page.locator('[data-testid="game-state"]');
    
    // Start the game first
    await startBtn.click();
    await expect(stateDisplay).toContainText('playing');
    
    // Pause the game
    await pauseBtn.click();
    await expect(stateDisplay).toContainText('paused');
  });

  test('should resume game when resume button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const startBtn = page.locator('[data-testid="start-game"]');
    const pauseBtn = page.locator('[data-testid="pause-game"]');
    const resumeBtn = page.locator('[data-testid="resume-game"]');
    const stateDisplay = page.locator('[data-testid="game-state"]');
    
    // Start the game
    await startBtn.click();
    await expect(stateDisplay).toContainText('playing');
    
    // Pause the game
    await pauseBtn.click();
    await expect(stateDisplay).toContainText('paused');
    
    // Resume the game
    await resumeBtn.click();
    await expect(stateDisplay).toContainText('playing');
  });

  test('should stop game when stop button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const startBtn = page.locator('[data-testid="start-game"]');
    const stopBtn = page.locator('[data-testid="stop-game"]');
    const stateDisplay = page.locator('[data-testid="game-state"]');
    
    // Start the game
    await startBtn.click();
    await expect(stateDisplay).toContainText('playing');
    
    // Stop the game
    await stopBtn.click();
    await expect(stateDisplay).toContainText('idle');
  });

  test('should handle game over when game over button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const startBtn = page.locator('[data-testid="start-game"]');
    const gameOverBtn = page.locator('[data-testid="game-over"]');
    const stateDisplay = page.locator('[data-testid="game-state"]');
    
    // Start the game
    await startBtn.click();
    await expect(stateDisplay).toContainText('playing');
    
    // Trigger game over
    await gameOverBtn.click();
    await expect(stateDisplay).toContainText('game_over');
  });

  test('should display player information', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if player information is displayed
    const playerInfo = page.locator('[data-testid="player-info"]');
    await expect(playerInfo).toBeVisible();
    
    // Check if score is displayed
    const scoreDisplay = page.locator('[data-testid="score"]');
    await expect(scoreDisplay).toBeVisible();
    await expect(scoreDisplay).toContainText('0');
  });

  test('should update score when score button is clicked', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const startBtn = page.locator('[data-testid="start-game"]');
    const scoreBtn = page.locator('[data-testid="add-score"]');
    const scoreDisplay = page.locator('[data-testid="score"]');
    
    // Start the game first
    await startBtn.click();
    
    // Add score
    await scoreBtn.click();
    
    // Score should increase
    const newScore = await scoreDisplay.textContent();
    expect(parseInt(newScore || '0')).toBeGreaterThan(0);
  });

  test('should handle level up', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const startBtn = page.locator('[data-testid="start-game"]');
    const levelUpBtn = page.locator('[data-testid="level-up"]');
    const levelDisplay = page.locator('[data-testid="level"]');
    
    // Start the game first
    await startBtn.click();
    
    // Get initial level
    const initialLevel = await levelDisplay.textContent();
    
    // Level up
    await levelUpBtn.click();
    
    // Level should increase
    const newLevel = await levelDisplay.textContent();
    expect(parseInt(newLevel || '1')).toBeGreaterThan(parseInt(initialLevel || '1'));
  });

  test('should display generated code files', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if generated files section is visible
    const generatedFiles = page.locator('[data-testid="generated-files"]');
    await expect(generatedFiles).toBeVisible();
    
    // Check if there are generated files listed
    const fileList = page.locator('[data-testid="file-item"]');
    await expect(fileList).toHaveCount(3); // Should have Rust, TypeScript, and Python files
  });

  test('should show code generation status', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    // Check if code generation status is displayed
    const statusDisplay = page.locator('[data-testid="generation-status"]');
    await expect(statusDisplay).toBeVisible();
    
    // Status should indicate success
    await expect(statusDisplay).toContainText('Generated');
  });

  test('should handle state transitions correctly', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const startBtn = page.locator('[data-testid="start-game"]');
    const pauseBtn = page.locator('[data-testid="pause-game"]');
    const resumeBtn = page.locator('[data-testid="resume-game"]');
    const stopBtn = page.locator('[data-testid="stop-game"]');
    const stateDisplay = page.locator('[data-testid="game-state"]');
    
    // Test complete state transition cycle
    await startBtn.click();
    await expect(stateDisplay).toContainText('playing');
    
    await pauseBtn.click();
    await expect(stateDisplay).toContainText('paused');
    
    await resumeBtn.click();
    await expect(stateDisplay).toContainText('playing');
    
    await stopBtn.click();
    await expect(stateDisplay).toContainText('idle');
  });

  test('should maintain game state across interactions', async ({ page }) => {
    await page.waitForFunction(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    
    const startBtn = page.locator('[data-testid="start-game"]');
    const scoreBtn = page.locator('[data-testid="add-score"]');
    const scoreDisplay = page.locator('[data-testid="score"]');
    const stateDisplay = page.locator('[data-testid="game-state"]');
    
    // Start game and add score
    await startBtn.click();
    await scoreBtn.click();
    
    const score = await scoreDisplay.textContent();
    await expect(stateDisplay).toContainText('playing');
    
    // Score should be maintained
    await expect(scoreDisplay).toContainText(score || '0');
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
