import { test, expect } from '@playwright/test';

test.describe('Simple Comprehensive WASM Examples Test Suite', () => {
  test('should verify counter example elements exist', async ({ page }) => {
    await page.goto('/examples/counter/dist/');
    await page.waitForLoadState('networkidle', { timeout: 15000 });
    
    // Test counter element exists
    const counter = page.locator('[data-testid="counter"]');
    await expect(counter).toBeVisible();
    
    // Test buttons exist
    const incrementBtn = page.locator('[data-testid="increment"]');
    const decrementBtn = page.locator('[data-testid="decrement"]');
    const resetBtn = page.locator('[data-testid="reset"]');
    
    await expect(incrementBtn).toBeVisible();
    await expect(decrementBtn).toBeVisible();
    await expect(resetBtn).toBeVisible();
    
    console.log('✅ Counter example elements verified');
  });

  test('should verify traffic-light example elements exist', async ({ page }) => {
    await page.goto('/examples/traffic-light/dist/');
    await page.waitForLoadState('networkidle', { timeout: 15000 });
    
    // Test current state element exists
    const currentState = page.locator('[data-testid="current-state"]');
    await expect(currentState).toBeVisible();
    
    // Test buttons exist
    const timerBtn = page.locator('[data-testid="timer"]');
    const pedestrianBtn = page.locator('[data-testid="pedestrian"]');
    const emergencyBtn = page.locator('[data-testid="emergency"]');
    const resetBtn = page.locator('[data-testid="reset"]');
    
    await expect(timerBtn).toBeVisible();
    await expect(pedestrianBtn).toBeVisible();
    await expect(emergencyBtn).toBeVisible();
    await expect(resetBtn).toBeVisible();
    
    // Test status elements exist
    const currentStateValue = page.locator('[data-testid="current-state-value"]');
    const pedestrianWaiting = page.locator('[data-testid="pedestrian-waiting"]');
    const emergencyMode = page.locator('[data-testid="emergency-mode"]');
    
    await expect(currentStateValue).toBeVisible();
    await expect(pedestrianWaiting).toBeVisible();
    await expect(emergencyMode).toBeVisible();
    
    console.log('✅ Traffic-light example elements verified');
  });

  test('should verify analytics-dashboard example loads', async ({ page }) => {
    await page.goto('/examples/analytics-dashboard/dist/');
    await page.waitForLoadState('networkidle', { timeout: 15000 });
    
    // Basic page validation
    const content = await page.content();
    expect(content.length).toBeGreaterThan(100);
    
    // Check title
    const titleText = await page.locator('title').innerText();
    expect(titleText).toContain('Analytics');
    
    console.log('✅ Analytics dashboard example loaded successfully');
  });

  test('should verify compatibility-example elements exist', async ({ page }) => {
    await page.goto('/examples/compatibility-example/dist/');
    await page.waitForLoadState('networkidle', { timeout: 15000 });
    
    // Test counter element
    const counter = page.locator('[data-testid="counter"]');
    await expect(counter).toBeVisible();
    
    // Test user display
    const userDisplay = page.locator('[data-testid="user-display"]');
    await expect(userDisplay).toBeVisible();
    
    // Test buttons
    const incrementBtn = page.locator('[data-testid="increment"]');
    const decrementBtn = page.locator('[data-testid="decrement"]');
    const resetBtn = page.locator('[data-testid="reset"]');
    
    await expect(incrementBtn).toBeVisible();
    await expect(decrementBtn).toBeVisible();
    await expect(resetBtn).toBeVisible();
    
    // Test name input
    const nameInput = page.locator('[data-testid="name-input"]');
    await expect(nameInput).toBeVisible();
    
    console.log('✅ Compatibility example elements verified');
  });

  test('should verify codegen example elements exist', async ({ page }) => {
    await page.goto('/examples/codegen/dist/');
    await page.waitForLoadState('networkidle', { timeout: 15000 });
    
    // Test game state elements
    const gameState = page.locator('[data-testid="game-state"]');
    const score = page.locator('[data-testid="score"]');
    const level = page.locator('[data-testid="level"]');
    const lives = page.locator('[data-testid="lives"]');
    
    await expect(gameState).toBeVisible();
    await expect(score).toBeVisible();
    await expect(level).toBeVisible();
    await expect(lives).toBeVisible();
    
    // Test game control buttons
    const startBtn = page.locator('[data-testid="start-game"]');
    const pauseBtn = page.locator('[data-testid="pause-game"]');
    const resumeBtn = page.locator('[data-testid="resume-game"]');
    const stopBtn = page.locator('[data-testid="stop-game"]');
    
    await expect(startBtn).toBeVisible();
    await expect(pauseBtn).toBeVisible();
    await expect(resumeBtn).toBeVisible();
    await expect(stopBtn).toBeVisible();
    
    // Test generated files section
    const generatedFiles = page.locator('[data-testid="generated-files"]');
    await expect(generatedFiles).toBeVisible();
    
    console.log('✅ Codegen example elements verified');
  });

  test('should verify history example elements exist', async ({ page }) => {
    await page.goto('/examples/history/dist/');
    await page.waitForLoadState('networkidle', { timeout: 15000 });
    
    // Test counter element
    const counter = page.locator('[data-testid="counter"]');
    await expect(counter).toBeVisible();
    
    // Test control buttons
    const incrementBtn = page.locator('[data-testid="increment"]');
    const decrementBtn = page.locator('[data-testid="decrement"]');
    const resetBtn = page.locator('[data-testid="reset"]');
    
    await expect(incrementBtn).toBeVisible();
    await expect(decrementBtn).toBeVisible();
    await expect(resetBtn).toBeVisible();
    
    // Test history buttons
    const deepHistoryBtn = page.locator('[data-testid="deep-history"]');
    const shallowHistoryBtn = page.locator('[data-testid="shallow-history"]');
    const clearHistoryBtn = page.locator('[data-testid="clear-history"]');
    
    await expect(deepHistoryBtn).toBeVisible();
    await expect(shallowHistoryBtn).toBeVisible();
    await expect(clearHistoryBtn).toBeVisible();
    
    // Test history display
    const history = page.locator('[data-testid="history"]');
    await expect(history).toBeVisible();
    
    console.log('✅ History example elements verified');
  });

  test('should verify all examples are accessible and have required elements', async ({ page }) => {
    const examples = [
      { name: 'Counter', path: '/examples/counter/dist/', expectedElement: '[data-testid="counter"]' },
      { name: 'Traffic Light', path: '/examples/traffic-light/dist/', expectedElement: '[data-testid="current-state"]' },
      { name: 'Analytics Dashboard', path: '/examples/analytics-dashboard/dist/', expectedElement: '#app' },
      { name: 'Compatibility Example', path: '/examples/compatibility-example/dist/', expectedElement: '[data-testid="counter"]' },
      { name: 'Codegen Example', path: '/examples/codegen/dist/', expectedElement: '[data-testid="game-state"]' },
      { name: 'History Example', path: '/examples/history/dist/', expectedElement: '[data-testid="counter"]' }
    ];
    
    for (const example of examples) {
      try {
        await page.goto(example.path);
        await page.waitForLoadState('networkidle', { timeout: 15000 });
        
        // Verify the expected element exists and is accessible
        const element = page.locator(example.expectedElement);
        await expect(element).toHaveCount(1);
        
        // For analytics dashboard, just check if the page loaded
        if (example.name === 'Analytics Dashboard') {
            const content = await page.content();
            expect(content.length).toBeGreaterThan(100);
        } else {
            await expect(element).toBeVisible();
        }
        
        console.log(`✅ ${example.name} is accessible and has required elements`);
      } catch (error) {
        console.log(`❌ ${example.name} failed: ${error.message}`);
        throw error;
      }
    }
    
    console.log('✅ All examples are accessible and have required elements');
  });
});
