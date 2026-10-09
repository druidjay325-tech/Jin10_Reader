export const INTERVALS = Object.freeze({ quotes: 600000, flash: 300000, news: 3600000, calendar: 3600000 });
export const INSTRUMENTS = Object.freeze([
  { code: 'XAUUSD', name: '现货黄金', digits: 2 },
  { code: 'XAGUSD', name: '现货白银', digits: 3 },
  { code: 'USOIL', name: 'WTI 原油', digits: 2 },
  { code: 'UKOIL', name: '布伦特原油', digits: 2 },
  { code: 'USDCNH', name: '美元 / 离岸人民币', digits: 4 },
  { code: 'EURUSD', name: '欧元 / 美元', digits: 4 },
]);

export function normalizeError(error) {
  if (error && typeof error === 'object') {
    return { code: String(error.code || 'error'), message: String(error.message || '操作未完成，请重试'), retryable: error.retryable !== false };
  }
  const message = String(error || '操作未完成，请重试');
  try { const parsed = JSON.parse(message); if (parsed && typeof parsed === 'object') return normalizeError(parsed); } catch { /* 非 JSON 的 Rust 错误也保留原文。 */ }
  return { code: 'error', message, retryable: true };
}

export function numberValue(value) {
  if (!['number', 'string'].includes(typeof value) || (typeof value === 'string' && !value.trim())) return null;
  const result = Number(value);
  return Number.isFinite(result) ? result : null;
}

export function safeUrl(value) {
  try {
    const url = new URL(String(value));
    return ['https:', 'http:'].includes(url.protocol) && !url.username && !url.password ? url.href : null;
  } catch { return null; }
}

export function itemKey(item) {
  return String(item.id ?? item.url ?? `${item.time || item.pub_time || ''}\u0000${item.title || item.content || ''}`);
}

function unique(items) {
  const seen = new Set();
  return items.filter(item => {
    const key = itemKey(item);
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

export function normalizePage(value) {
  if (Array.isArray(value)) return { items: value, cursor: null, hasMore: false };
  if (value && Array.isArray(value.important)) return { items: value.important, cursor: null, hasMore: false };
  if (!value || !Array.isArray(value.items)) throw new Error('数据格式暂不受支持，请稍后重试');
  const next = value.next_cursor ?? value.next_offset;
  const cursor = next !== null && next !== undefined && next !== '' ? String(next) : null;
  return { items: value.items.filter(item => item && typeof item === 'object'), cursor, hasMore: value.has_more !== false && Boolean(cursor) };
}

function emptyStore() {
  return { items: [], visible: 5, cursor: null, hasRemote: true, loaded: false, updatedAt: null, error: null, pending: null };
}

/** 本地未显示的条目先用完，再向服务端翻页；搜索结果与正常流各自持有状态。 */
export class Feed {
  constructor(fetchPage, onChange = () => {}) {
    this.fetchPage = fetchPage;
    this.onChange = onChange;
    this.normal = emptyStore();
    this.search = emptyStore();
    this.keyword = '';
    this.generation = 0;
    this.inFlight = null;
  }
  get store() { return this.keyword ? this.search : this.normal; }
  get snapshot() {
    const s = this.store;
    return { ...s, items: s.items.slice(0, s.visible), total: s.items.length, keyword: this.keyword, loading: Boolean(this.inFlight), hasMore: s.visible < s.items.length || s.hasRemote };
  }
  reset() {
    this.generation += 1;
    this.inFlight = null;
    this.normal = emptyStore();
    this.search = emptyStore();
    this.keyword = '';
    this.onChange(this.snapshot);
  }
  async query(keyword) {
    this.keyword = keyword.trim();
    this.generation += 1;
    this.inFlight = null;
    if (this.keyword) this.search = emptyStore();
    this.onChange(this.snapshot);
    return this.refresh({ preserve: !this.keyword && this.normal.loaded });
  }
  refresh({ preserve = true } = {}) { return this.request(false, preserve); }
  async more() {
    if (this.inFlight) return this.inFlight;
    if (this.store.visible < this.store.items.length) {
      this.store.visible += 5;
      this.onChange(this.snapshot);
      return;
    }
    if (!this.store.hasRemote) return;
    return this.request(true, false);
  }
  acceptPending() {
    const s = this.store;
    if (!s.pending) return;
    Object.assign(s, s.pending, { visible: 5, pending: null, error: null });
    this.onChange(this.snapshot);
  }
  request(more, preserve) {
    if (this.inFlight) return this.inFlight;
    const generation = this.generation;
    const keyword = this.keyword;
    const s = this.store;
    const cursor = more ? s.cursor : null;
    const task = Promise.resolve().then(() => generation === this.generation ? this.fetchPage({ keyword, cursor }) : null).then(value => {
      if (generation !== this.generation) return;
      const page = normalizePage(value);
      const update = { items: unique(page.items), cursor: page.cursor, hasRemote: page.hasMore, loaded: true, updatedAt: Date.now() };
      if (more) {
        s.items = unique([...s.items, ...page.items]);
        s.visible += 5;
        s.cursor = page.cursor;
        s.hasRemote = page.hasMore && page.cursor !== cursor;
        s.updatedAt = update.updatedAt;
      } else if (preserve && s.loaded && s.items.length) {
        const currentKeys = s.items.slice(0, page.items.length).map(itemKey).join('\u0001');
        const nextKeys = update.items.map(itemKey).join('\u0001');
        if (!update.items.length || currentKeys !== nextKeys) s.pending = update;
        else {
          const revised = new Map(update.items.map(item => [itemKey(item), item]));
          s.items = s.items.map(item => revised.get(itemKey(item)) || item);
          s.updatedAt = update.updatedAt;
          s.pending = null;
        }
      } else Object.assign(s, update, { visible: 5, pending: null });
      s.error = null;
    }).catch(error => {
      if (generation === this.generation) s.error = normalizeError(error);
    }).finally(() => {
      if (generation === this.generation) { this.inFlight = null; this.onChange(this.snapshot); }
    });
    this.inFlight = task;
    this.onChange(this.snapshot);
    return task;
  }
}

/** 每个模块只有一个定时器和一个在途请求；隐藏、重连不会积累定时器。 */
export class Scheduler {
  constructor(tasks, clock = globalThis) {
    this.clock = clock;
    this.active = false;
    this.entries = Object.fromEntries(Object.entries(tasks).map(([name, task]) => [name, { task, timer: null, flight: null, recovery: null, paused: false }]));
  }
  start(immediate = true) {
    if (this.active) return;
    this.active = true;
    for (const name of Object.keys(this.entries)) {
      if (immediate) void this.run(name);
      else this.schedule(name);
    }
  }
  stop() {
    this.active = false;
    for (const entry of Object.values(this.entries)) { this.clock.clearTimeout(entry.timer); entry.timer = null; }
  }
  schedule(name) {
    const entry = this.entries[name];
    if (!this.active || entry.paused || entry.timer !== null) return;
    entry.timer = this.clock.setTimeout(() => { entry.timer = null; void this.run(name); }, INTERVALS[name]);
  }
  run(name, manual = false) {
    const entry = this.entries[name];
    if (entry.flight) return entry.flight;
    if (manual) entry.paused = false;
    if (entry.paused) return Promise.resolve();
    this.clock.clearTimeout(entry.timer);
    entry.timer = null;
    entry.flight = Promise.resolve().then(entry.task).finally(() => { entry.flight = null; this.schedule(name); });
    return entry.flight;
  }
  pause(name) {
    const entry = this.entries[name];
    entry.paused = true;
    this.clock.clearTimeout(entry.timer);
    entry.timer = null;
  }
  recover(names) {
    for (const name of names) {
      this.entries[name].paused = false;
      if (this.active) void this.run(name);
    }
  }
  recoverWhenSettled(hasError, isCurrent = () => true) {
    for (const [name, entry] of Object.entries(this.entries)) {
      if (entry.recovery) continue;
      // 就绪事件可能先于被替换连接的失败响应；等旧请求结束再判定，并覆盖整个重试周期去重。
      entry.recovery = Promise.resolve(entry.flight).catch(() => {}).then(() => {
        if (this.active && isCurrent() && hasError(name)) return this.run(name, true);
      }).finally(() => { entry.recovery = null; });
    }
  }
}

export function appearanceDefaults(config) {
  return { ...config, theme: 'dark', opacity: 0.95 };
}
