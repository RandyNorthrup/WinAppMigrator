// Scan every page of the built website with axe-core, in a real browser, for the WCAG 2.2 A and AA rules, in light and
// dark color schemes, and fail if any page has a violation. Automated rules find some problems, not all; see
// docs/ACCESSIBILITY.md for what is checked by hand.
//
//   node .github/axe/scan-site.mjs <built site folder> <the site's address, as it was built for>
//
// The browser is given the built files at the site's own address, so every page loads what it would on the live site,
// from the same origin its Content-Security-Policy allows, and absolute links work.
import { readFile, readdir } from 'node:fs/promises';
import { extname, join, normalize, relative, sep } from 'node:path';
import { chromium } from 'playwright';
import { AxeBuilder } from '@axe-core/playwright';

const TAGS = ['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa', 'wcag22a', 'wcag22aa', 'best-practice'];
const TYPES = { '.html': 'text/html; charset=utf-8', '.css': 'text/css', '.js': 'text/javascript', '.png': 'image/png',
  '.ico': 'image/x-icon', '.xml': 'application/xml', '.txt': 'text/plain' };

const [root, address] = process.argv.slice(2);
if (!root || !address) {
  console.error('usage: node scan-site.mjs <built site folder> <site address>');
  process.exit(2);
}
const base = address.replace(/\/+$/, '') + '/';

async function pages(folder) {
  const found = [];
  for (const entry of await readdir(folder, { withFileTypes: true })) {
    const path = join(folder, entry.name);
    if (entry.isDirectory()) found.push(...await pages(path));
    else if (entry.name.endsWith('.html')) found.push(relative(root, path).split(sep).join('/'));
  }
  return found.sort();
}

// GitHub Pages serves "Page" for "Page.html" and "folder/" for "folder/index.html"; so does this.
async function serve(route) {
  let path = decodeURIComponent(new URL(route.request().url()).pathname.slice(new URL(base).pathname.length));
  if (path === '' || path.endsWith('/')) path += 'index.html';
  const file = normalize(join(root, path));
  if (!file.startsWith(normalize(root))) return route.fulfill({ status: 403 });
  for (const candidate of [file, `${file}.html`]) {
    try {
      const body = await readFile(candidate);
      return route.fulfill({ status: 200, body, contentType: TYPES[extname(candidate)] ?? 'application/octet-stream' });
    } catch {
      // try the next
    }
  }
  return route.fulfill({ status: 404, body: '' });
}

const browser = await chromium.launch();
let violations = 0;
const list = await pages(root);
for (const scheme of ['light', 'dark']) {
  const context = await browser.newContext({ colorScheme: scheme });
  await context.route(`${base}**`, serve);
  for (const page of list) {
    const tab = await context.newPage();
    await tab.goto(base + page.replace(/(^|\/)index\.html$/, '$1').replace(/\.html$/, ''), { waitUntil: 'load' });
    const result = await new AxeBuilder({ page: tab }).withTags(TAGS).analyze();
    for (const violation of result.violations) {
      violations += 1;
      console.log(`${page} (${scheme}): ${violation.id} [${violation.impact}] ${violation.help}`);
      for (const node of violation.nodes.slice(0, 5)) console.log(`    ${node.target.join(' ')}: ${node.failureSummary.split('\n').join(' ')}`);
    }
    await tab.close();
  }
  await context.close();
}
await browser.close();
console.log(`${list.length} pages scanned in light and dark: ${violations} violation(s)`);
process.exit(violations === 0 ? 0 : 1);
