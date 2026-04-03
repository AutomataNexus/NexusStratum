import { test, expect } from '@playwright/test';

// ── Landing Page ──────────────────────────────────────────────────

test.describe('Landing Page', () => {
  test('loads and shows hero', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/NexusStratum/);
    await expect(page.locator('h1')).toBeVisible();
  });

  test('hero code block is left-aligned', async ({ page }) => {
    await page.goto('/');
    const codeBlock = page.locator('.code-block pre').first();
    await expect(codeBlock).toBeVisible();
    const textAlign = await codeBlock.evaluate(el => getComputedStyle(el).textAlign);
    expect(textAlign).toBe('left');
  });

  test('nav links work', async ({ page }) => {
    await page.goto('/');
    await page.click('a[href="/docs/"]');
    await expect(page).toHaveURL(/\/docs\//);
    await expect(page.locator('h1')).toContainText('Introduction');
  });

  test('Get Started button navigates to installation', async ({ page }) => {
    await page.goto('/');
    await page.click('a[href="/docs/installation.html"]');
    await expect(page).toHaveURL(/installation/);
  });

  test('Browse Components button navigates to catalog', async ({ page }) => {
    await page.goto('/');
    await page.click('a[href="/components/"]');
    await expect(page).toHaveURL(/\/components\//);
  });

  test('stats section visible', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.stat-num').first()).toBeVisible();
  });

  test('favicon is set', async ({ page }) => {
    await page.goto('/');
    const favicon = page.locator('link[rel="icon"]').first();
    await expect(favicon).toHaveAttribute('href', /favicon/);
  });
});

// ── Components Catalog ────────────────────────────────────────────

test.describe('Components Catalog', () => {
  test('loads with component cards', async ({ page }) => {
    await page.goto('/components/');
    await expect(page).toHaveTitle(/Components/);
    const cards = page.locator('.comp-card');
    await expect(cards.first()).toBeVisible();
    expect(await cards.count()).toBeGreaterThan(10);
  });

  test('category filter works', async ({ page }) => {
    await page.goto('/components/');
    await page.click('[data-cat-filter="Overlay"]');
    // Overlay cards should be visible
    const dialogCard = page.locator('[data-comp-cat="Overlay"]').first();
    await expect(dialogCard).toBeVisible();
    // Forms cards should be hidden
    const formCard = page.locator('[data-comp-cat="Forms"]').first();
    await expect(formCard).toBeHidden();
  });

  test('component card links to detail page', async ({ page }) => {
    await page.goto('/components/');
    await page.click('a[href="/components/button.html"]');
    await expect(page).toHaveURL(/button\.html/);
    await expect(page.locator('h1')).toContainText('Button');
  });
});

// ── Component Detail Pages ────────────────────────────────────────

test.describe('Component Detail - Button', () => {
  test('loads with preview and code tabs', async ({ page }) => {
    await page.goto('/components/button.html');
    await expect(page.locator('h1')).toContainText('Button');
    await expect(page.locator('.comp-preview-tab').first()).toBeVisible();
  });

  test('preview/code toggle works', async ({ page }) => {
    await page.goto('/components/button.html');
    const codeTab = page.locator('.comp-preview-tab', { hasText: 'Code' }).first();
    await codeTab.click();
    await expect(page.locator('.code-block pre').first()).toBeVisible();
  });

  test('props table is present', async ({ page }) => {
    await page.goto('/components/button.html');
    await expect(page.locator('.props-table')).toBeVisible();
    await expect(page.locator('.props-table td', { hasText: 'variant' })).toBeVisible();
  });

  test('sidebar navigation works', async ({ page }) => {
    await page.goto('/components/button.html');
    await page.click('.sidebar-list a[href="/components/input.html"]');
    await expect(page).toHaveURL(/input\.html/);
    await expect(page.locator('h1')).toContainText('Input');
  });
});

// ── Docs Pages ────────────────────────────────────────────────────

test.describe('Documentation', () => {
  const docPages = [
    { path: '/docs/', title: 'Introduction' },
    { path: '/docs/installation.html', title: 'Installation' },
    { path: '/docs/theming.html', title: 'Theming' },
    { path: '/docs/dark-mode.html', title: 'Dark Mode' },
    { path: '/docs/cli.html', title: 'CLI' },
    { path: '/docs/primitives.html', title: 'Headless Primitives' },
    { path: '/docs/components.html', title: 'Styled Components' },
    { path: '/docs/adapters.html', title: 'Framework Adapters' },
    { path: '/docs/accessibility.html', title: 'Accessibility' },
    { path: '/docs/security.html', title: 'Security' },
    { path: '/docs/leptos.html', title: 'Leptos' },
    { path: '/docs/dioxus.html', title: 'Dioxus' },
  ];

  for (const { path, title } of docPages) {
    test(`${title} page loads`, async ({ page }) => {
      await page.goto(path);
      await expect(page.locator('h1')).toContainText(title);
      await expect(page.locator('.sidebar')).toBeVisible();
    });
  }
});

// ── Blocks Page ───────────────────────────────────────────────────

test.describe('Blocks', () => {
  test('loads with block cards', async ({ page }) => {
    await page.goto('/blocks/');
    await expect(page).toHaveTitle(/Blocks/);
    const cards = page.locator('.block-card');
    await expect(cards.first()).toBeVisible();
    expect(await cards.count()).toBeGreaterThan(3);
  });
});

// ── Search ────────────────────────────────────────────────────────

test.describe('Search', () => {
  test('opens with Ctrl+K', async ({ page }) => {
    await page.goto('/');
    await page.keyboard.press('Control+k');
    await expect(page.locator('#search-overlay')).toHaveClass(/open/);
  });

  test('search input filters results', async ({ page }) => {
    await page.goto('/');
    await page.keyboard.press('Control+k');
    await page.fill('#search-input', 'button');
    const results = page.locator('.search-result');
    await expect(results.first()).toBeVisible();
    await expect(results.first()).toContainText('Button');
  });

  test('closes with Escape', async ({ page }) => {
    await page.goto('/');
    await page.keyboard.press('Control+k');
    await expect(page.locator('#search-overlay')).toHaveClass(/open/);
    await page.keyboard.press('Escape');
    await expect(page.locator('#search-overlay')).not.toHaveClass(/open/);
  });
});

// ── Cross-page Navigation ─────────────────────────────────────────

test.describe('Navigation Integrity', () => {
  test('every component page has a working back link to catalog', async ({ page }) => {
    await page.goto('/components/dialog.html');
    await expect(page.locator('h1')).toContainText('Dialog');
    // Sidebar should have a link back to components index
    const catalogLink = page.locator('.sidebar-list a[href="/components/button.html"]');
    await expect(catalogLink).toBeVisible();
  });

  test('docs sidebar links all resolve', async ({ page }) => {
    await page.goto('/docs/');
    const links = page.locator('.sidebar-list a');
    const count = await links.count();
    expect(count).toBeGreaterThan(8);
    // Click the last link in sidebar to verify it works
    const lastLink = links.last();
    const href = await lastLink.getAttribute('href');
    await lastLink.click();
    await expect(page).toHaveURL(new RegExp(href!.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')));
  });
});

// ── Copy Button ───────────────────────────────────────────────────

test.describe('Copy Button', () => {
  test('copy button exists on code blocks', async ({ page }) => {
    await page.goto('/components/button.html');
    const copyBtn = page.locator('.code-copy').first();
    await expect(copyBtn).toBeVisible();
    await expect(copyBtn).toContainText('Copy');
  });
});

// ── Responsive ────────────────────────────────────────────────────

test.describe('Responsive', () => {
  test('mobile viewport hides sidebar', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/components/button.html');
    await expect(page.locator('.sidebar')).toBeHidden();
  });

  test('mobile viewport hides nav links', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/');
    await expect(page.locator('.nav-links')).toBeHidden();
  });
});
