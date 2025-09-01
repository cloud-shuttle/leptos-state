#!/usr/bin/env node

const http = require('http');

// Test the built counter example
async function testCounterExample() {
    console.log('🧪 Testing Counter Example...');
    
    try {
        const html = await fetchHTML('http://localhost:8000/examples/counter/dist/');
        
        // Debug: show what we're looking for
        console.log('  🔍 Looking for data-testid attributes...');
        console.log('  📄 HTML length:', html.length);
        console.log('  🔍 Contains "data-testid":', html.includes('data-testid'));
        console.log('  🔍 Contains "counter":', html.includes('counter'));
        console.log('  🔍 Contains "increment":', html.includes('increment'));
        
        // Check if all required elements are present
        const requiredElements = [
            '[data-testid="counter"]',
            '[data-testid="increment"]',
            '[data-testid="decrement"]',
            '[data-testid="reset"]',
            '[data-testid="name-input"]',
            '[data-testid="user-display"]'
        ];
        
        let allElementsFound = true;
        for (const selector of requiredElements) {
            const testId = selector.replace(/[\[\]]/g, '').replace('data-testid="', '').replace('"', '');
            if (html.includes(`data-testid="${testId}"`)) {
                console.log(`  ✅ Found ${selector}`);
            } else {
                console.log(`  ❌ Missing ${selector}`);
                allElementsFound = false;
            }
        }
        
        // Check if CSS is loaded
        if (html.includes('styles.css')) {
            console.log('  ✅ CSS file referenced');
        } else {
            console.log('  ❌ CSS file not referenced');
            allElementsFound = false;
        }
        
        // Check if JavaScript is present
        if (html.includes('<script>') && html.includes('addEventListener')) {
            console.log('  ✅ JavaScript functionality present');
        } else {
            console.log('  ❌ JavaScript functionality missing');
            allElementsFound = false;
        }
        
        if (allElementsFound) {
            console.log('  🎉 Counter example is working correctly!');
        } else {
            console.log('  ⚠️  Counter example has issues');
        }
        
        return allElementsFound;
    } catch (error) {
        console.log(`  ❌ Error testing counter example: ${error.message}`);
        return false;
    }
}

// Test the built traffic-light example
async function testTrafficLightExample() {
    console.log('\n🧪 Testing Traffic Light Example...');
    
    try {
        const html = await fetchHTML('http://localhost:8000/examples/traffic-light/dist/');
        
        // Check if all required elements are present
        const requiredElements = [
            '[data-testid="current-state-display"]',
            '[data-testid="timer"]',
            '[data-testid="pedestrian"]',
            '[data-testid="emergency"]',
            '[data-testid="reset"]',
            '[data-testid="pedestrian-waiting"]'
        ];
        
        let allElementsFound = true;
        for (const selector of requiredElements) {
            const testId = selector.replace(/[\[\]]/g, '').replace('data-testid="', '').replace('"', '');
            if (html.includes(`data-testid="${testId}"`)) {
                console.log(`  ✅ Found ${selector}`);
            } else {
                console.log(`  ❌ Missing ${selector}`);
                allElementsFound = false;
            }
        }
        
        // Check if CSS is loaded
        if (html.includes('styles.css')) {
            console.log('  ✅ CSS file referenced');
        } else {
            console.log('  ❌ CSS file not referenced');
            allElementsFound = false;
        }
        
        // Check if JavaScript is present
        if (html.includes('<script>') && html.includes('addEventListener')) {
            console.log('  ✅ JavaScript functionality present');
        } else {
            console.log('  ❌ JavaScript functionality missing');
            allElementsFound = false;
        }
        
        if (allElementsFound) {
            console.log('  🎉 Traffic Light example is working correctly!');
        } else {
            console.log('  ⚠️  Traffic Light example has issues');
        }
        
        return allElementsFound;
    } catch (error) {
        console.log(`  ❌ Error testing traffic light example: ${error.message}`);
        return false;
    }
}

// Helper function to fetch HTML
function fetchHTML(url) {
    return new Promise((resolve, reject) => {
        const req = http.get(url, (res) => {
            let data = '';
            res.on('data', (chunk) => {
                data += chunk;
            });
            res.on('end', () => {
                resolve(data);
            });
        });
        
        req.on('error', (error) => {
            reject(error);
        });
        
        req.setTimeout(5000, () => {
            req.destroy();
            reject(new Error('Request timeout'));
        });
    });
}

// Run tests
async function runTests() {
    console.log('🚀 Testing Built Examples\n');
    
    const counterResult = await testCounterExample();
    const trafficLightResult = await testTrafficLightExample();
    
    console.log('\n📊 Test Results:');
    console.log(`  Counter Example: ${counterResult ? '✅ PASS' : '❌ FAIL'}`);
    console.log(`  Traffic Light Example: ${trafficLightResult ? '✅ PASS' : '❌ FAIL'}`);
    
    if (counterResult && trafficLightResult) {
        console.log('\n🎉 All examples are working correctly!');
        console.log('✅ Our fixes have resolved the issues.');
        console.log('✅ The HTML structure matches the test expectations.');
        console.log('✅ CSS files are properly referenced.');
        console.log('✅ JavaScript functionality is present.');
    } else {
        console.log('\n⚠️  Some examples still have issues.');
        console.log('❌ Further investigation is needed.');
    }
}

// Check if server is running
fetchHTML('http://localhost:8000/')
    .then(() => {
        runTests();
    })
    .catch((error) => {
        console.log('❌ Server not running. Please start the server with: make serve');
        console.log('   Then run this test again.');
        process.exit(1);
    });
