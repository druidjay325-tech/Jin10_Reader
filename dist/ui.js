import { normalizeError, safeUrl } from './model.js';

export function el(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = String(text);
  return node;
}

function gearPath() {
  const points = [];
  for (let tooth = 0; tooth < 8; tooth++) {
    for (const [offset, radius] of [[-.5,7.6],[-.24,7.6],[-.24,10.1],[.24,10.1],[.24,7.6],[.5,7.6]]) {
      const angle = -Math.PI / 2 + (tooth + offset) * Math.PI / 4;
      points.push(`${(12 + Math.cos(angle) * radius).toFixed(2)} ${(12 + Math.sin(angle) * radius).toFixed(2)}`);
    }
  }
  return 'M' + points.join('L') + 'Z';
}
const PATHS = {
  mark: ['M4 19V5l5 7 5-7v14', 'M19 5v14'],
  settings: [gearPath(), 'M12 8.5a3.5 3.5 0 1 0 0 7 3.5 3.5 0 0 0 0-7'],
  refresh: ['M20 7v5h-5', 'M4 17v-5h5', 'M6 7a7 7 0 0 1 12-1l2 3', 'M18 17a7 7 0 0 1-12 1l-2-3'],
  close: ['m6 6 12 12', 'M18 6 6 18'],
  minus: ['M5 12h14'],
  mini: ['M4 5h16v14H4z', 'M12 12h5v4h-5z'],
  expand: ['M8 3H3v5', 'm3 3 6 6', 'M16 21h5v-5', 'm21 21-6-6'],
  search: ['M10 3a7 7 0 1 0 0 14 7 7 0 0 0 0-14', 'm15 15 6 6'],
  arrow: ['M5 12h14', 'm14 7 5 5-5 5'],
  external: ['M14 3h7v7', 'm10 14 11-11', 'M10 3H3v18h18v-7'],
  chevron: ['m6 9 6 6 6-6'],
  shield: ['m12 3 8 3v6c0 5-8 9-8 9s-8-4-8-9V6z', 'm8 12 3 3 5-6'],
  copy: ['M8 8h12v13H8z', 'M16 8V3H3v13h5'],
};

export function icon(name) {
  const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
  for (const [key, value] of Object.entries({ viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.6', 'stroke-linecap': 'round', 'stroke-linejoin': 'round', 'aria-hidden': 'true' })) svg.setAttribute(key, value);
  for (const d of PATHS[name] || PATHS.arrow) {
    const path = document.createElementNS(svg.namespaceURI, 'path'); path.setAttribute('d', d); svg.append(path);
  }
  return svg;
}

export function button(label, callback, className = 'button', iconName) {
  const node = el('button', className);
  node.type = 'button';
  if (iconName) node.append(icon(iconName));
  node.append(el('span', '', label));
  if (callback) node.addEventListener('click', callback);
  return node;
}

export function iconButton(label, name, callback) {
  const node = button(label, callback, 'icon-button', name);
  node.title = label; node.setAttribute('aria-label', label);
  return node;
}

const formatter = new Intl.DateTimeFormat('zh-CN', { timeZone: 'Asia/Shanghai', hour: '2-digit', minute: '2-digit', hour12: false });
const dateFormatter = new Intl.DateTimeFormat('zh-CN', { timeZone: 'Asia/Shanghai', month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit', hour12: false });
export function parseDate(value) {
  if (!value) return null;
  // 上游不带时区的日期按其来源的北京时间解释，避免受本机时区影响。
  let normalized = typeof value === 'string' && /^\d{4}-\d\d-\d\d[ T]\d\d:\d\d(?::\d\d)?$/.test(value) ? value.replace(' ', 'T') + '+08:00' : value;
  if (typeof value === 'number' && value > 0 && value < 100000000000) normalized = value * 1000;
  const date = new Date(normalized);
  return Number.isNaN(date.valueOf()) ? null : date;
}
export function timeText(value, full = false) {
  const date = parseDate(value);
  return date ? (full ? dateFormatter : formatter).format(date) : '时间未提供';
}
export function applyTheme(theme, opacity = 0.95) {
  const resolved = theme === 'system' ? (matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light') : theme;
  document.documentElement.dataset.theme = resolved === 'glass' ? 'glass' : resolved === 'light' ? 'light' : 'dark';
  const value = Number(opacity);
  document.documentElement.style.setProperty('--glass-opacity', String(Number.isFinite(value) ? Math.max(0, Math.min(1, value)) : .95));
}
export function showError(node, error) {
  node.textContent = normalizeError(error).message;
  node.hidden = false;
  node.setAttribute('role', 'alert');
}
export function externalLink(label, url, open, onError) {
  const href = safeUrl(url);
  if (!href) return null;
  const link = el('a', 'text-link', label);
  link.href = href; link.target = '_blank'; link.rel = 'noopener noreferrer';
  link.addEventListener('click', event => { event.preventDefault(); void open(href).catch(onError); });
  return link;
}
