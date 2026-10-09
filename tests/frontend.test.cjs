const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { pathToFileURL } = require('node:url');
const modelPromise = import(pathToFileURL(path.join(__dirname, '../dist/model.js')).href);
const letters = (start, count) => Array.from({ length: count }, (_, i) => ({ id: String.fromCharCode(65 + start + i), content: `内容 ${start + i}` }));

test('历史分页先消耗本地缓冲，再请求下个游标，没有漏项或重复', async () => {
  const { Feed } = await modelPromise; const calls = [];
  const feed = new Feed(async ({ cursor }) => {
    calls.push(cursor);
    return cursor ? { items: letters(10, 10), next_offset: null, has_more: false } : { items: letters(0, 10), next_offset: 'C2', has_more: true };
  });
  await feed.refresh(); assert.deepEqual(feed.snapshot.items.map(x => x.id), ['A','B','C','D','E']);
  await feed.more(); assert.equal(calls.length, 1); assert.equal(feed.snapshot.items.length, 10);
  await feed.more(); assert.deepEqual(calls, [null, 'C2']); assert.equal(feed.snapshot.items.length, 15);
  await feed.more(); assert.equal(feed.snapshot.items.length, 20); assert.equal(feed.snapshot.hasMore, false);
  await feed.more(); assert.equal(calls.length, 2);
  assert.equal(new Set(feed.snapshot.items.map(x => x.id)).size, 20);
});

test('最新刷新使用空游标，新内容等待用户接收且后续分页从新游标开始', async () => {
  const { Feed } = await modelPromise; let round = 0; const calls = [];
  const feed = new Feed(async ({ cursor }) => { calls.push(cursor); return { items: letters(round++ ? 20 : 0, 10), next_cursor: 'next', has_more: true }; });
  await feed.refresh(); await feed.more(); await feed.refresh();
  assert.deepEqual(calls, [null, null]); assert.equal(feed.snapshot.items[0].id, 'A'); assert.ok(feed.snapshot.pending);
  feed.acceptPending(); assert.equal(feed.snapshot.items[0].id, 'U'); assert.equal(feed.snapshot.items.length, 5);
});

test('相同 ID 的正文修订更新当前内容，保留已加载历史', async () => {
  const { Feed } = await modelPromise; let revised = false;
  const feed = new Feed(async ({ cursor }) => cursor ? { items: letters(10, 5), has_more: false } : { items: letters(0, 10).map(x => ({ ...x, content: revised ? '修订内容' : x.content })), next_cursor: 'next', has_more: true });
  await feed.refresh(); await feed.more(); await feed.more(); revised = true; await feed.refresh();
  assert.equal(feed.snapshot.items[0].content, '修订内容'); assert.equal(feed.snapshot.items.length, 15); assert.equal(feed.snapshot.pending, null);
});

test('慢 A 与快 B 搜索乱序时只展示 B，清除后恢复正常流', async () => {
  const { Feed } = await modelPromise; let resolveA;
  const feed = new Feed(({ keyword }) => keyword === 'A' ? new Promise(resolve => { resolveA = resolve; }) : Promise.resolve({ items: [{ id: keyword || 'normal', content: keyword || '正常内容' }], has_more: false }));
  await feed.refresh(); const a = feed.query('A'); await Promise.resolve(); await feed.query('B');
  resolveA({ items: [{ id: 'A', content: '过期响应' }], has_more: false }); await a;
  assert.equal(feed.snapshot.items[0].id, 'B'); await feed.query(''); assert.equal(feed.snapshot.items[0].id, 'normal');
});

test('清除搜索后过期响应不能覆盖正常流', async () => {
  const { Feed } = await modelPromise; let resolveSearch;
  const feed = new Feed(({ keyword }) => keyword ? new Promise(resolve => { resolveSearch = resolve; }) : Promise.resolve({ items: [{ id: 'normal' }], has_more: false }));
  await feed.refresh(); const search = feed.query('delay'); await Promise.resolve(); await feed.query('');
  resolveSearch({ items: [{ id: 'search' }], has_more: false }); await search;
  assert.equal(feed.snapshot.items[0].id, 'normal');
});

test('切换账户清空正常流、搜索和待接收内容，旧搜索响应不能进入新账户', async () => {
  const { Feed } = await modelPromise; let account = 'A'; let resolveOldSearch;
  const feed = new Feed(({ keyword }) => keyword ? new Promise(resolve => { resolveOldSearch = resolve; }) : Promise.resolve({ items: [{ id: account, content: `账户 ${account}` }], has_more: false }));
  await feed.refresh(); account = 'A-new'; await feed.refresh(); assert.ok(feed.snapshot.pending);
  const oldSearch = feed.query('旧账户关键词'); await Promise.resolve();
  feed.reset();
  assert.equal(feed.snapshot.keyword, ''); assert.equal(feed.snapshot.items.length, 0); assert.equal(feed.snapshot.pending, null); assert.equal(feed.snapshot.updatedAt, null); assert.equal(feed.snapshot.error, null); assert.equal(feed.snapshot.loading, false);
  account = 'B'; await feed.refresh(); resolveOldSearch({ items: [{ id: 'old-search' }], has_more: false }); await oldSearch;
  assert.equal(feed.snapshot.items[0].id, 'B'); assert.equal(feed.snapshot.keyword, ''); assert.equal(feed.normal.items.length, 1); assert.equal(feed.search.items.length, 0);
});

test('账户新调度器不被旧在途请求阻塞，旧完成不复活timer，重复ready不多拉', async () => {
  const { Scheduler } = await modelPromise; const timers = new Map(); let id = 0; let oldCalls = 0; let newCalls = 0; const release = [];
  const clock = { setTimeout(fn) { timers.set(++id, fn); return id; }, clearTimeout(n) { timers.delete(n); } };
  const names = ['quotes', 'flash', 'news', 'calendar'];
  const old = new Scheduler(Object.fromEntries(names.map(name => [name, () => { oldCalls++; return new Promise(resolve => release.push(resolve)); }])), clock);
  old.start(); await Promise.resolve(); assert.equal(oldCalls, 4); old.stop();
  const current = new Scheduler(Object.fromEntries(names.map(name => [name, async () => { newCalls++; }])), clock);
  current.start(); current.start(); await Promise.all(names.map(name => current.run(name)));
  assert.equal(newCalls, 4); assert.equal(timers.size, 4);
  release.forEach(resolve => resolve()); await Promise.all(names.map(name => old.entries[name].flight));
  assert.equal(timers.size, 4); current.stop(); assert.equal(timers.size, 0);
});

test('重复游标停止网络翻页，重叠项去重', async () => {
  const { Feed } = await modelPromise; let calls = 0;
  const feed = new Feed(async () => { calls++; return { items: letters(0, 5), next_cursor: 'same', has_more: true }; });
  await feed.refresh(); await feed.more(); await feed.more();
  assert.equal(feed.snapshot.items.length, 5); assert.equal(feed.snapshot.hasMore, false); assert.equal(calls, 2);
});

test('失败保留内容与真实成功时间，首次空数据可辨识', async () => {
  const { Feed } = await modelPromise; let fail = false;
  const feed = new Feed(async () => { if (fail) throw { code: 'offline', message: '连接已断开' }; return { items: letters(0, 5), has_more: false }; });
  await feed.refresh(); const time = feed.snapshot.updatedAt; fail = true; await feed.refresh();
  assert.equal(feed.snapshot.items.length, 5); assert.equal(feed.snapshot.updatedAt, time); assert.equal(feed.snapshot.error.code, 'offline');
  const empty = new Feed(async () => []); await empty.refresh(); assert.equal(empty.snapshot.loaded, true); assert.equal(empty.snapshot.hasMore, false);
});

test('刷新为空时不伪装成当前旧内容仍是最新', async () => {
  const { Feed } = await modelPromise; let empty = false;
  const feed = new Feed(async () => empty ? [] : letters(0, 5)); await feed.refresh(); empty = true; await feed.refresh();
  assert.ok(feed.snapshot.pending); feed.acceptPending(); assert.equal(feed.snapshot.items.length, 0); assert.equal(feed.snapshot.hasMore, false);
});

test('重连十次仍只有四个调度器，每模块一轮首批请求', async () => {
  const { Scheduler } = await modelPromise; const timers = new Map(); let id = 0; const calls = {};
  const clock = { setTimeout(fn, ms) { timers.set(++id, { fn, ms }); return id; }, clearTimeout(n) { timers.delete(n); } };
  const scheduler = new Scheduler(Object.fromEntries(['quotes','flash','news','calendar'].map(name => [name, async () => { calls[name] = (calls[name] || 0) + 1; }])), clock);
  for (let i = 0; i < 10; i++) scheduler.start();
  await Promise.all(Object.keys(scheduler.entries).map(name => scheduler.run(name)));
  assert.deepEqual(Object.values(calls), [1,1,1,1]); assert.equal(timers.size, 4);
  scheduler.stop(); assert.equal(timers.size, 0);
  scheduler.start(false); assert.equal(timers.size, 4); scheduler.stop(); assert.equal(timers.size, 0);
});

test('停止时在途请求完成不会复活定时器，配额暂停需手动恢复', async () => {
  const { Scheduler } = await modelPromise; const timers = new Map(); let id = 0; let resolve; let calls = 0;
  const clock = { setTimeout(fn) { timers.set(++id, fn); return id; }, clearTimeout(n) { timers.delete(n); } };
  const scheduler = new Scheduler({ quotes: () => { calls++; return new Promise(r => { resolve = r; }); } }, clock);
  scheduler.start(); await Promise.resolve(); const flight = scheduler.run('quotes'); scheduler.stop(); resolve(); await flight; assert.equal(timers.size, 0);
  scheduler.start(false); scheduler.pause('quotes'); assert.equal(timers.size, 0); await scheduler.run('quotes'); assert.equal(calls, 1);
  const retry = scheduler.run('quotes', true); await Promise.resolve(); resolve(); await retry; assert.equal(calls, 2); assert.equal(timers.size, 1); scheduler.stop();
});

test('外链拒绝脚本、本地文件及含凭据地址，仅支持 HTTP(S)', async () => {
  const { safeUrl } = await modelPromise;
  for (const value of ['javascript:alert(1)', 'data:text/html,test', 'file:///C:/secret', 'https://user:pass@example.com', '\" onclick=bad']) assert.equal(safeUrl(value), null);
  assert.equal(safeUrl('https://example.com/a?q=1'), 'https://example.com/a?q=1');
});

test('显式重连只恢复失败模块，不重拉健康模块，不新增重复调度器', async () => {
  const { Scheduler } = await modelPromise; const timers = new Map(); const calls = { quotes: 0, flash: 0 }; let id = 0;
  const clock = { setTimeout(fn) { timers.set(++id, fn); return id; }, clearTimeout(n) { timers.delete(n); } };
  const scheduler = new Scheduler({ quotes: async () => { calls.quotes++; }, flash: async () => { calls.flash++; } }, clock);
  scheduler.start(); await Promise.all(['quotes', 'flash'].map(name => scheduler.run(name)));
  scheduler.pause('quotes'); scheduler.recover(['quotes']); await scheduler.run('quotes');
  assert.equal(calls.quotes, 2); assert.equal(calls.flash, 1); assert.equal(timers.size, 2); scheduler.stop();
  scheduler.pause('quotes'); scheduler.recover(['quotes']); assert.equal(calls.quotes, 2); assert.equal(timers.size, 0);
  scheduler.start(false); assert.equal(timers.size, 2); scheduler.stop();
});

test('ready早于旧请求结束时等待superseded结果再单次恢复，重复ready保持幂等', async () => {
  const { Scheduler } = await modelPromise; const timers = new Map(); let id = 0; let finishOld; let failed = false; let quotes = 0; let flash = 0;
  const clock = { setTimeout(fn) { timers.set(++id, fn); return id; }, clearTimeout(n) { timers.delete(n); } };
  const scheduler = new Scheduler({
    quotes: async () => { quotes++; if (quotes === 1) { await new Promise(resolve => { finishOld = resolve; }); failed = true; } else failed = false; },
    flash: async () => { flash++; },
  }, clock);
  scheduler.start(); await Promise.resolve();
  for (let i = 0; i < 10; i++) scheduler.recoverWhenSettled(name => name === 'quotes' && failed);
  finishOld(); await Promise.all(Object.values(scheduler.entries).map(entry => entry.recovery));
  assert.equal(quotes, 2); assert.equal(flash, 1); assert.equal(failed, false); assert.equal(timers.size, 2); scheduler.stop();
});

test('待旧请求结束的恢复在隐藏或账户切换后失效', async () => {
  const { Scheduler } = await modelPromise; let finishOld; let calls = 0; let current = true;
  const scheduler = new Scheduler({ quotes: async () => { calls++; await new Promise(resolve => { finishOld = resolve; }); } });
  scheduler.start(); await Promise.resolve(); scheduler.recoverWhenSettled(() => true, () => current);
  current = false; scheduler.stop(); finishOld(); await scheduler.entries.quotes.recovery;
  assert.equal(calls, 1); assert.equal(scheduler.entries.quotes.timer, null);
});

test('空值、非法数值不伪造零价格，真实零仍保留', async () => {
  const { numberValue } = await modelPromise;
  for (const value of [null, undefined, '', '   ', [], {}, false, 'abc', Infinity]) assert.equal(numberValue(value), null);
  assert.equal(numberValue('0'), 0); assert.equal(numberValue('7.0825'), 7.0825);
});

test('普通外观恢复保留账户、提醒和其他字段', async () => {
  const { appearanceDefaults } = await modelPromise;
  const cfg = { theme: 'light', opacity: .6, mcp: { hasToken: true }, reminders: [{ id: 'keep' }], autoStart: true };
  const result = appearanceDefaults(cfg); assert.deepEqual(result.mcp, cfg.mcp); assert.deepEqual(result.reminders, cfg.reminders); assert.equal(result.autoStart, true); assert.equal(result.theme, 'dark'); assert.equal(cfg.theme, 'light');
});

test('字符串和结构化 IPC 错误均保留可读信息', async () => {
  const { normalizeError, normalizePage } = await modelPromise;
  assert.equal(normalizeError('连接失败').message, '连接失败'); assert.equal(normalizeError('42').message, '42');
  assert.equal(normalizeError('{"code":"auth_failed","message":"凭据无效","retryable":false}').retryable, false);
  assert.equal(normalizePage({ items: [], next_offset: 20, has_more: true }).cursor, '20');
});

test('公开前端不持久化新闻、无内联执行入口且 IPC 集中', () => {
  const dir = path.join(__dirname, '../dist');
  for (const filename of fs.readdirSync(dir).filter(file => /\.(js|html)$/.test(file))) {
    const source = fs.readFileSync(path.join(dir, filename), 'utf8');
    assert.doesNotMatch(source, /\b(?:localStorage|sessionStorage|indexedDB)\b/);
    assert.doesNotMatch(source, /\.innerHTML\s*=|\binsertAdjacentHTML\s*\(|\beval\s*\(|\bnew\s+Function\s*\(/);
    if (filename.endsWith('.html')) assert.doesNotMatch(source, /\son[a-z]+\s*=/i);
    if (filename !== 'commands.js') assert.doesNotMatch(source, /\.invoke\s*\(/);
  }
});
