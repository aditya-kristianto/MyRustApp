import { browser } from 'k6/browser';
import { check } from 'https://jslib.k6.io/k6-utils/1.5.0/index.js';
import http from 'k6/http';

const BASE_URL = 'https://adira.co.id/'; // Target URL

export const options = {
  scenarios: {
    httpLoadTest: {
      exec: 'httpTest',
      executor: 'ramping-arrival-rate',
      startRate: 1000, // Start with 1,000 requests per minute
      timeUnit: '1m', // Per minute
      preAllocatedVUs: 100, // Pre-allocate VUs for better scaling
      maxVUs: 5000, // Max VUs allowed
      stages: [
        { duration: '1m', target: 1000 },  // Start at 1,000 req/min
        { duration: '1m', target: 10000 }, // Increase to 10,000 req/min
        // { duration: '1m', target: 100000 }, // Increase to 100,000 req/min
        // { duration: '1m', target: 1000000 }, // Increase to 1,000,000 req/min
        // { duration: '1m', target: 10000000 }, // Increase to 10,000,000 req/min
        // { duration: '5m', target: 10000000 }, // Sustain 10M req/min for 5 minutes
      ],
    },
    browserTest: {
      exec: 'browserTest',
      executor: 'constant-vus',
      vus: 1, // Single browser instance for real UI testing
      duration: '10m', // Run for 10 minutes
      options: {
        browser: {
          type: 'chromium',
        },
      },
    },
  },
  thresholds: {
    http_req_failed: ['rate<0.01'], // Less than 1% failure rate
    http_req_duration: ['p(95)<500', 'p(99)<1000'], // 95% requests < 500ms, 99% < 1000ms
    browser_web_vital_fcp: ['p(95) < 1000'], // First Contentful Paint < 1s
    browser_web_vital_lcp: ['p(95) < 2000'], // Largest Contentful Paint < 2s
  },
};

// ✅ Function to load the page via HTTP requests
export function httpTest() {
  const res = http.get(BASE_URL);

  check(res, {
    'status is 200': (r) => r.status === 200,
    'response time < 500ms': (r) => r.timings.duration < 500,
  });
}

// ✅ Function to load the page in the browser and interact
export async function browserTest() {
  const page = await browser.newPage();

  try {
    await page.goto(BASE_URL);

    await check(page.locator('h1'), {
      'header exists': async (lo) => (await lo.textContent()) !== '',
    });

    await page.screenshot({ path: `screenshots/${__ITER}.png` });
  } finally {
    await page.close();
  }
}
