// Error Handling Utilities for Playwright Tests
// Following ADR-005: Playwright Testing Strategy

import { Page, expect } from '@playwright/test';

export class ErrorHandler {
  static async handleNetworkError(page: Page, action: () => Promise<void>) {
    try {
      await action();
    } catch (error) {
      if (error.message.includes('net::ERR_')) {
        console.log('Network error detected, retrying...');
        await page.waitForTimeout(1000);
        await action();
      } else {
        throw error;
      }
    }
  }

  static async handleElementNotFound(page: Page, selector: string, action: () => Promise<void>) {
    try {
      await action();
    } catch (error) {
      if (error.message.includes('locator')) {
        console.log(`Element not found: ${selector}, waiting and retrying...`);
        await page.waitForSelector(selector, { timeout: 5000 });
        await action();
      } else {
        throw error;
      }
    }
  }

  static async expectNoConsoleErrors(page: Page) {
    const errors: string[] = [];
    
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    return {
      checkErrors: () => {
        expect(errors).toHaveLength(0);
      }
    };
  }
}

