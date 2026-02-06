/**
 * Generates a self-contained HTML report page for visual regression test results.
 * Fetches summary.json client-side and renders an interactive comparison view.
 */

export function generateReportPage(): string {
  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Visual Regression Report</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, monospace; background: #0d1117; color: #c9d1d9; }

  .header { padding: 16px 24px; border-bottom: 1px solid #21262d; display: flex; align-items: center; gap: 16px; flex-wrap: wrap; }
  .header h1 { font-size: 18px; font-weight: 600; }

  .summary { display: flex; gap: 8px; flex-wrap: wrap; }
  .stat { padding: 4px 12px; border-radius: 16px; font-size: 13px; font-weight: 600; }
  .stat-pass { background: #238636; color: #fff; }
  .stat-fail { background: #da3633; color: #fff; }
  .stat-error { background: #d29922; color: #000; }
  .stat-skip { background: #30363d; color: #8b949e; }

  .filters { padding: 8px 24px; border-bottom: 1px solid #21262d; display: flex; gap: 4px; }
  .filter-btn { background: #21262d; color: #c9d1d9; border: 1px solid #30363d; padding: 4px 14px; border-radius: 6px; cursor: pointer; font-size: 13px; }
  .filter-btn:hover { background: #30363d; }
  .filter-btn.active { background: #388bfd; color: #fff; border-color: #388bfd; }

  .list { padding: 8px 24px; }
  .fixture { border: 1px solid #21262d; border-radius: 6px; margin-bottom: 4px; overflow: hidden; }
  .fixture-row { display: flex; align-items: center; gap: 12px; padding: 8px 12px; cursor: pointer; }
  .fixture-row:hover { background: #161b22; }

  .badge { display: inline-block; padding: 2px 8px; border-radius: 12px; font-size: 11px; font-weight: 600; text-transform: uppercase; min-width: 44px; text-align: center; }
  .badge-pass { background: #238636; color: #fff; }
  .badge-fail { background: #da3633; color: #fff; }
  .badge-error { background: #d29922; color: #000; }
  .badge-skip { background: #30363d; color: #8b949e; }

  .fixture-id { flex: 1; font-size: 13px; font-family: monospace; }
  .fixture-diff { font-size: 12px; color: #8b949e; }

  .comparison { display: none; padding: 12px; background: #161b22; border-top: 1px solid #21262d; }
  .comparison.open { display: block; }
  .images { display: flex; gap: 12px; flex-wrap: wrap; }
  .img-col { flex: 1; min-width: 200px; text-align: center; }
  .img-col h3 { font-size: 12px; color: #8b949e; margin-bottom: 6px; text-transform: uppercase; }
  .img-col img { max-width: 100%; border: 1px solid #30363d; background: #fff; image-rendering: pixelated; }
  .img-col .no-img { padding: 40px; color: #484f58; font-size: 13px; border: 1px dashed #30363d; }

  .error-msg { font-size: 12px; color: #d29922; padding: 8px 12px; background: #161b22; border-top: 1px solid #21262d; font-family: monospace; }

  .loading { padding: 48px; text-align: center; color: #8b949e; }
  .empty { padding: 24px; text-align: center; color: #484f58; }
</style>
</head>
<body>

<div class="header">
  <h1>Visual Regression Report</h1>
  <div class="summary" id="summary"></div>
</div>

<div class="filters" id="filters">
  <button class="filter-btn active" data-filter="fail">Fail</button>
  <button class="filter-btn" data-filter="error">Error</button>
  <button class="filter-btn" data-filter="pass">Pass</button>
  <button class="filter-btn" data-filter="skip">Skip</button>
  <button class="filter-btn" data-filter="all">All</button>
</div>

<div class="list" id="list">
  <div class="loading">Loading results...</div>
</div>

<script>
let allResults = [];
let currentFilter = 'fail';

async function load() {
  try {
    const resp = await fetch('/results/summary.json');
    if (!resp.ok) {
      document.getElementById('list').innerHTML = '<div class="empty">No results found. Run tests first: bun run test</div>';
      return;
    }
    const data = await resp.json();

    document.getElementById('summary').innerHTML =
      '<span class="stat stat-pass">' + data.pass + ' pass</span>' +
      '<span class="stat stat-fail">' + data.fail + ' fail</span>' +
      '<span class="stat stat-error">' + data.error + ' error</span>' +
      '<span class="stat stat-skip">' + data.skip + ' skip</span>';

    allResults = data.results || [];
    render();
  } catch (e) {
    document.getElementById('list').innerHTML = '<div class="empty">Failed to load results: ' + e.message + '</div>';
  }
}

function render() {
  const filtered = currentFilter === 'all'
    ? allResults
    : allResults.filter(r => r.status === currentFilter);

  if (filtered.length === 0) {
    document.getElementById('list').innerHTML = '<div class="empty">No fixtures match this filter.</div>';
    return;
  }

  const html = filtered.map((r, i) => {
    const diffText = r.difference != null ? 'diff: ' + (r.difference * 100).toFixed(4) + '% / allowed: ' + ((r.allowed || 0) * 100).toFixed(4) + '%' : '';
    const errorHtml = r.error ? '<div class="error-msg">' + escapeHtml(r.error) + '</div>' : '';

    const compId = 'comp-' + i;
    const encodedId = r.id.replace(/#/g, '%23');

    let imagesHtml = '';
    if (r.status === 'fail' || r.status === 'error') {
      imagesHtml = '<div class="comparison" id="' + compId + '">' +
        '<div class="images">' +
          '<div class="img-col"><h3>Expected</h3><img src="/fixtures/' + encodedId + '/expected.png" onerror="this.outerHTML=\\'<div class=no-img>No expected image</div>\\'"></div>' +
          '<div class="img-col"><h3>Actual</h3><img src="/results/diffs/' + encodedId + '/actual.png" onerror="this.outerHTML=\\'<div class=no-img>No actual image</div>\\'"></div>' +
          '<div class="img-col"><h3>Diff</h3><img src="/results/diffs/' + encodedId + '/diff.png" onerror="this.outerHTML=\\'<div class=no-img>No diff image</div>\\'"></div>' +
        '</div></div>';
    } else if (r.status === 'pass' || r.status === 'skip') {
      imagesHtml = '<div class="comparison" id="' + compId + '">' +
        '<div class="images">' +
          '<div class="img-col"><h3>Expected</h3><img src="/fixtures/' + encodedId + '/expected.png" onerror="this.outerHTML=\\'<div class=no-img>No expected image</div>\\'"></div>' +
        '</div></div>';
    }

    return '<div class="fixture">' +
      '<div class="fixture-row" onclick="toggle(\\'' + compId + '\\')">' +
        '<span class="badge badge-' + r.status + '">' + r.status + '</span>' +
        '<span class="fixture-id">' + escapeHtml(r.id) + '</span>' +
        '<span class="fixture-diff">' + diffText + '</span>' +
      '</div>' +
      errorHtml +
      imagesHtml +
    '</div>';
  }).join('');

  document.getElementById('list').innerHTML = html;
}

function toggle(id) {
  const el = document.getElementById(id);
  if (el) el.classList.toggle('open');
}

function escapeHtml(s) {
  const d = document.createElement('div');
  d.textContent = s;
  return d.innerHTML;
}

// Filter buttons
document.getElementById('filters').addEventListener('click', (e) => {
  if (e.target.classList.contains('filter-btn')) {
    document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));
    e.target.classList.add('active');
    currentFilter = e.target.dataset.filter;
    render();
  }
});

load();
</script>
</body>
</html>`;
}
