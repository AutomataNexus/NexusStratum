/* =============================================================
   NexusStratum — Shared JS
   ============================================================= */

// Component data registry
const COMPONENTS = [
  { name: 'Button', cat: 'Forms', desc: 'Clickable actions with 6 variants and 5 sizes.', tags: ['pressable','interactive'] },
  { name: 'Input', cat: 'Forms', desc: 'Single-line text input with validation states.', tags: ['form','text'] },
  { name: 'Textarea', cat: 'Forms', desc: 'Multi-line text input with resize control.', tags: ['form','text'] },
  { name: 'Checkbox', cat: 'Forms', desc: 'Tri-state checkbox with indeterminate support.', tags: ['form','selection'] },
  { name: 'Radio', cat: 'Forms', desc: 'Mutually exclusive selection within a group.', tags: ['form','selection'] },
  { name: 'Switch', cat: 'Forms', desc: 'Binary toggle between on and off.', tags: ['form','toggle'] },
  { name: 'Select', cat: 'Forms', desc: 'Dropdown selection with search and keyboard nav.', tags: ['form','selection'] },
  { name: 'Form', cat: 'Forms', desc: 'Form container with field validation and labels.', tags: ['form','layout'] },
  { name: 'Slider', cat: 'Forms', desc: 'Range input with keyboard and touch support.', tags: ['form','range'] },
  { name: 'NumberInput', cat: 'Forms', desc: 'Numeric input with increment/decrement.', tags: ['form','number'] },
  { name: 'DatePicker', cat: 'Forms', desc: 'Calendar-based date selection.', tags: ['form','date'] },
  { name: 'Combobox', cat: 'Forms', desc: 'Searchable dropdown with autocomplete.', tags: ['form','search'] },
  { name: 'Dialog', cat: 'Overlay', desc: 'Modal dialog with focus trap and backdrop.', tags: ['overlay','modal'] },
  { name: 'AlertDialog', cat: 'Overlay', desc: 'Confirmation dialog requiring user action.', tags: ['overlay','modal'] },
  { name: 'Tooltip', cat: 'Overlay', desc: 'Contextual info on hover or focus.', tags: ['overlay','info'] },
  { name: 'Popover', cat: 'Overlay', desc: 'Anchored floating content panel.', tags: ['overlay','float'] },
  { name: 'Toast', cat: 'Overlay', desc: 'Temporary notification with auto-dismiss.', tags: ['overlay','feedback'] },
  { name: 'HoverCard', cat: 'Overlay', desc: 'Rich preview content on hover.', tags: ['overlay','info'] },
  { name: 'ContextMenu', cat: 'Overlay', desc: 'Right-click menu with keyboard support.', tags: ['overlay','menu'] },
  { name: 'Drawer', cat: 'Overlay', desc: 'Slide-in panel from any edge.', tags: ['overlay','panel'] },
  { name: 'Sheet', cat: 'Overlay', desc: 'Overlay panel for secondary content.', tags: ['overlay','panel'] },
  { name: 'Tabs', cat: 'Navigation', desc: 'Tabbed interface with arrow key navigation.', tags: ['nav','tabs'] },
  { name: 'Accordion', cat: 'Navigation', desc: 'Collapsible content sections.', tags: ['nav','disclosure'] },
  { name: 'NavigationMenu', cat: 'Navigation', desc: 'Site navigation with dropdowns.', tags: ['nav','menu'] },
  { name: 'Breadcrumb', cat: 'Navigation', desc: 'Path-based navigation trail.', tags: ['nav','path'] },
  { name: 'Pagination', cat: 'Navigation', desc: 'Page navigation controls.', tags: ['nav','page'] },
  { name: 'Menu', cat: 'Navigation', desc: 'Dropdown menu with keyboard and type-ahead.', tags: ['nav','menu'] },
  { name: 'DropdownMenu', cat: 'Navigation', desc: 'Triggered dropdown with submenus.', tags: ['nav','menu'] },
  { name: 'Card', cat: 'Data Display', desc: 'Container with header, body, and footer.', tags: ['layout','container'] },
  { name: 'Badge', cat: 'Data Display', desc: 'Status label with 4 variants.', tags: ['label','status'] },
  { name: 'Table', cat: 'Data Display', desc: 'Data table with sorting and selection.', tags: ['data','table'] },
  { name: 'DataTable', cat: 'Data Display', desc: 'Rich data table with pagination.', tags: ['data','table'] },
  { name: 'Avatar', cat: 'Data Display', desc: 'User image with fallback initials.', tags: ['user','image'] },
  { name: 'Tag', cat: 'Data Display', desc: 'Removable label tag.', tags: ['label','tag'] },
  { name: 'Progress', cat: 'Data Display', desc: 'Progress bar with determinate/indeterminate.', tags: ['feedback','loading'] },
  { name: 'Skeleton', cat: 'Data Display', desc: 'Loading placeholder animation.', tags: ['loading','placeholder'] },
  { name: 'Spinner', cat: 'Data Display', desc: 'Animated loading indicator.', tags: ['loading','spinner'] },
  { name: 'Carousel', cat: 'Data Display', desc: 'Scrollable content carousel.', tags: ['layout','scroll'] },
  { name: 'Timeline', cat: 'Data Display', desc: 'Chronological event display.', tags: ['data','time'] },
  { name: 'Stack', cat: 'Layout', desc: 'Flex container with direction and spacing.', tags: ['layout','flex'] },
  { name: 'Divider', cat: 'Layout', desc: 'Visual separator between content.', tags: ['layout','separator'] },
  { name: 'Grid', cat: 'Layout', desc: 'CSS grid layout container.', tags: ['layout','grid'] },
  { name: 'Container', cat: 'Layout', desc: 'Max-width centered container.', tags: ['layout','wrapper'] },
  { name: 'AspectRatio', cat: 'Layout', desc: 'Fixed aspect ratio container.', tags: ['layout','ratio'] },
  { name: 'ScrollArea', cat: 'Layout', desc: 'Custom scrollbar container.', tags: ['layout','scroll'] },
  { name: 'Resizable', cat: 'Layout', desc: 'Resizable panel layout.', tags: ['layout','resize'] },
  { name: 'Text', cat: 'Typography', desc: 'Text with configurable size, weight, color.', tags: ['text','typography'] },
  { name: 'Heading', cat: 'Typography', desc: 'Heading levels h1-h6 with size mapping.', tags: ['text','heading'] },
  { name: 'Link', cat: 'Typography', desc: 'Styled link with external support.', tags: ['text','link'] },
  { name: 'Code', cat: 'Typography', desc: 'Inline and block code display.', tags: ['text','code'] },
  { name: 'Kbd', cat: 'Typography', desc: 'Keyboard shortcut display.', tags: ['text','keyboard'] },
  { name: 'Alert', cat: 'Feedback', desc: 'Contextual feedback message.', tags: ['feedback','alert'] },
  { name: 'EmptyState', cat: 'Feedback', desc: 'Placeholder for empty content areas.', tags: ['feedback','empty'] },
  { name: 'Separator', cat: 'Utility', desc: 'Semantic or decorative divider.', tags: ['utility','divider'] },
  { name: 'VisuallyHidden', cat: 'Utility', desc: 'Screen-reader-only content.', tags: ['utility','a11y'] },
  { name: 'Portal', cat: 'Utility', desc: 'Render outside DOM hierarchy.', tags: ['utility','portal'] },
  { name: 'FocusScope', cat: 'Utility', desc: 'Focus trap for modals and dialogs.', tags: ['utility','focus'] },
];

const CATEGORIES = [...new Set(COMPONENTS.map(c => c.cat))];

// ---- Search ----
function initSearch() {
  const overlay = document.getElementById('search-overlay');
  const input = document.getElementById('search-input');
  const results = document.getElementById('search-results');
  if (!overlay) return;

  // Open with Ctrl+K or clicking search button
  document.addEventListener('keydown', e => {
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') { e.preventDefault(); openSearch(); }
    if (e.key === 'Escape') closeSearch();
  });

  document.querySelectorAll('[data-search-trigger]').forEach(el => {
    el.addEventListener('click', openSearch);
  });

  overlay.addEventListener('click', e => { if (e.target === overlay) closeSearch(); });

  function openSearch() { overlay.classList.add('open'); input.value = ''; input.focus(); renderResults(''); }
  function closeSearch() { overlay.classList.remove('open'); }

  input.addEventListener('input', () => renderResults(input.value));

  function renderResults(query) {
    const q = query.toLowerCase().trim();
    const filtered = q ? COMPONENTS.filter(c =>
      c.name.toLowerCase().includes(q) ||
      c.cat.toLowerCase().includes(q) ||
      c.desc.toLowerCase().includes(q) ||
      c.tags.some(t => t.includes(q))
    ) : COMPONENTS.slice(0, 12);

    if (filtered.length === 0) {
      results.innerHTML = '<div class="search-empty">No components found.</div>';
      return;
    }

    results.innerHTML = filtered.map(c => `
      <a href="/components/${c.name.toLowerCase()}.html" class="search-result">
        <div>
          <div class="search-result-title">${c.name}</div>
          <div class="search-result-cat">${c.cat} &mdash; ${c.desc}</div>
        </div>
      </a>
    `).join('');
  }
}

// ---- Preview Tabs (Preview/Code toggle) ----
function initPreviewTabs() {
  document.querySelectorAll('[data-preview-tabs]').forEach(container => {
    const tabs = container.querySelectorAll('.comp-preview-tab');
    const panels = container.querySelectorAll('[data-panel]');
    tabs.forEach(tab => {
      tab.addEventListener('click', () => {
        tabs.forEach(t => t.classList.remove('active'));
        panels.forEach(p => { p.classList.remove('active'); p.style.display = 'none'; });
        tab.classList.add('active');
        const target = container.querySelector(`[data-panel="${tab.dataset.tab}"]`);
        if (target) { target.classList.add('active'); target.style.display = tab.dataset.tab === 'preview' ? 'flex' : 'block'; }
      });
    });
  });
}

// ---- Copy to clipboard ----
function initCopyButtons() {
  document.querySelectorAll('.code-copy').forEach(btn => {
    btn.addEventListener('click', () => {
      const code = btn.closest('.code-block').querySelector('pre').textContent;
      navigator.clipboard.writeText(code).then(() => {
        const orig = btn.textContent;
        btn.textContent = 'Copied!';
        setTimeout(() => btn.textContent = orig, 1500);
      });
    });
  });
}

// ---- Interactive mini components ----
function initInteractive() {
  // Checkboxes
  document.querySelectorAll('.render-checkbox').forEach(cb => {
    cb.addEventListener('click', () => cb.classList.toggle('checked'));
  });
  // Switches
  document.querySelectorAll('.render-switch').forEach(sw => {
    sw.addEventListener('click', () => sw.classList.toggle('on'));
  });
  // Render tabs
  document.querySelectorAll('.render-tab').forEach(tab => {
    tab.addEventListener('click', () => {
      const group = tab.parentElement;
      group.querySelectorAll('.render-tab').forEach(t => t.classList.remove('active'));
      tab.classList.add('active');
    });
  });
}

// ---- Category filter (components page) ----
function initCategoryFilter() {
  const filterBtns = document.querySelectorAll('[data-cat-filter]');
  const cards = document.querySelectorAll('[data-comp-cat]');
  if (!filterBtns.length) return;

  filterBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      filterBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      const cat = btn.dataset.catFilter;
      cards.forEach(card => {
        card.style.display = (cat === 'all' || card.dataset.compCat === cat) ? '' : 'none';
      });
    });
  });
}

// ---- Init everything ----
document.addEventListener('DOMContentLoaded', () => {
  initSearch();
  initPreviewTabs();
  initCopyButtons();
  initInteractive();
  initCategoryFilter();
});
