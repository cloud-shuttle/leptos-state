// Global Teardown for Playwright Tests
// Following ADR-005: Playwright Testing Strategy

import { FullConfig } from '@playwright/test';

async function globalTeardown(config: FullConfig) {
  console.log('Tearing down Playwright tests...');
  
  // Perform any global cleanup tasks
  // For example, cleaning up test data, stopping services, etc.
  
  console.log('Global teardown completed');
}

export default globalTeardown;

