// scrape.js
const { webkit } = require('playwright');  // Or 'chromium' or 'firefox'

(async () => {
    const browser = await webkit.launch();
    const page = await browser.newPage();
    await page.goto('https://www.idx.co.id/id/data-pasar/ringkasan-perdagangan/ringkasan-saham/');  // Replace with the target URL
    
    // Perform necessary actions to navigate, wait, and scrape content
    await page.waitForSelector('a');  // Example: wait for links to load
    
    const links = await page.$$eval('a', anchors => {
        return anchors.map(anchor => anchor.href);
    });
    
    console.log(JSON.stringify(links));  // Print links as JSON

    await browser.close();
})();
