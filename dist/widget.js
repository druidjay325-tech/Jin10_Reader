import { commands, environment } from './commands.js';
import { Feed, Scheduler, INSTRUMENTS, INTERVALS, normalizeError, numberValue, itemKey } from './model.js';
import { el, icon, button, iconButton, applyTheme, timeText, parseDate, showError, externalLink } from './ui.js';

const app = document.getElementById('app');
const state = { config: null, selected: 'flash', mini: false, quotes: [], quotesAt: null, quotesError: null, expanded: new Set(), health: null };
const feeds = {};
let renderedFeedKey = '';
let dataGeneration = 0;
let readyGeneration = null;
const labels = { flash: '最新快讯', news: '深度资讯', calendar: '财经日历' };
const unlisten = [];

const header = el('header', 'titlebar');
header.setAttribute('data-tauri-drag-region', '');
const brand = el('div', 'brand'); brand.setAttribute('data-tauri-drag-region', '');
const appIcon = el('img', 'brand-icon'); appIcon.src = './app-icon.png'; appIcon.alt = ''; appIcon.draggable = false;
brand.append(appIcon, el('span', 'brand-name', 'JIN10'), el('span', 'brand-subtitle', 'READER'));
brand.querySelectorAll('*').forEach(node => node.setAttribute('data-tauri-drag-region', ''));
const controls = el('div', 'window-actions');
const miniButton = iconButton('进入迷你行情', 'mini', () => void toggleMini());
const maximizeButton = iconButton('最大化或还原窗口', 'expand', () => void commands.maximize().catch(topError));
controls.append(miniButton, maximizeButton, iconButton('打开设置', 'settings', () => void commands.openSettings().catch(topError)), iconButton('隐藏到托盘', 'minus', () => void commands.hide().catch(topError)));
header.addEventListener('mousedown', event => {
  if (event.button === 0 && event.detail === 2 && !event.target.closest('button')) {
    event.preventDefault(); event.stopImmediatePropagation();
    if (!state.mini) void commands.maximize().catch(topError);
  }
}, true);
header.append(brand, controls); app.append(header);
if (environment.demo) app.append(el('div', 'demo-banner', '演示数据 · 非真实行情 · 离线预览'));
const topMessage = el('div', 'global-message'); topMessage.hidden = true; app.append(topMessage);

const onboarding = el('main', 'onboarding'); onboarding.hidden = true;
const introMark = el('div', 'intro-mark'); introMark.append(icon('mark'));
const onboardingText = el('div', 'intro-copy');
onboardingText.append(el('p', 'eyebrow', '一扇安静的市场窗口'), el('h1', '', '让信息，回到阅读。'), el('p', 'intro-description', '行情快照、即时快讯与经济日历，在桌面一隅，保持清晰。'));
const connect = button('连接数据源', () => void commands.openSettings().catch(topError), 'button primary', 'arrow');
const privacy = el('div', 'privacy-note'); privacy.append(icon('shield'), el('span', '', '使用你自己的金十 MCP Token。凭据仅保存在本机；新闻仅在内存中展示。'));
onboarding.append(introMark, onboardingText, connect, privacy, el('p', 'source-note', '独立开发的非官方阅读器 · 数据服务需另行授权'));
app.append(onboarding);

const workspace = el('main', 'workspace'); workspace.hidden = true; app.append(workspace);
const market = el('section', 'market'); market.setAttribute('aria-labelledby', 'marketTitle');
const marketHeading = el('div', 'section-heading');
const marketTitle = el('h1', '', '市场一瞥'); marketTitle.id = 'marketTitle';
const quoteRefresh = iconButton('刷新行情快照', 'refresh', () => void scheduler.run('quotes', true));
marketHeading.append(marketTitle, el('span', 'section-caption', '行情快照 · 每 10 分钟'), quoteRefresh);
const quoteGrid = el('div', 'quote-grid');
const quoteStatus = el('div', 'module-status'); quoteStatus.setAttribute('aria-live', 'polite');
market.append(marketHeading, quoteGrid, quoteStatus); workspace.append(market);

const reader = el('section', 'reader'); workspace.append(reader);
const tablist = el('div', 'tabs'); tablist.setAttribute('role', 'tablist'); tablist.setAttribute('aria-label', '阅读内容');
const tabButtons = {};
for (const name of Object.keys(labels)) {
  const tab = button(labels[name], () => selectTab(name), 'tab');
  tab.id = `tab-${name}`; tab.setAttribute('role', 'tab'); tab.setAttribute('aria-controls', 'feedPanel');
  tab.addEventListener('keydown', event => {
    if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(event.key)) return;
    event.preventDefault();
    const names = Object.keys(labels); const current = names.indexOf(name);
    const next = event.key === 'Home' ? 0 : event.key === 'End' ? names.length - 1 : (current + (event.key === 'ArrowRight' ? 1 : -1) + names.length) % names.length;
    selectTab(names[next]); tabButtons[names[next]].focus();
  });
  tabButtons[name] = tab; tablist.append(tab);
}
reader.append(tablist);
const searchForm = el('form', 'search-form'); searchForm.setAttribute('role', 'search');
const searchLabel = el('label', 'visually-hidden', '搜索当前栏目'); searchLabel.htmlFor = 'searchInput';
const searchIcon = icon('search');
const searchInput = el('input', 'search-input'); searchInput.id = 'searchInput'; searchInput.type = 'search'; searchInput.placeholder = '搜索快讯'; searchInput.maxLength = 120; searchInput.autocomplete = 'off';
const searchClear = iconButton('清除搜索，恢复最新内容', 'close', () => { searchInput.value = ''; void feeds[state.selected].query(''); }); searchClear.hidden = true;
const searchSubmit = button('搜索', null, 'search-submit'); searchSubmit.type = 'submit';
const feedRefresh = iconButton('刷新当前栏目', 'refresh', () => void scheduler.run(state.selected, true));
searchForm.append(searchLabel, searchIcon, searchInput, searchClear, searchSubmit, feedRefresh);
searchForm.addEventListener('submit', event => { event.preventDefault(); void feeds[state.selected].query(searchInput.value); });
reader.append(searchForm);
const feedPanel = el('div', 'feed-panel'); feedPanel.id = 'feedPanel'; feedPanel.setAttribute('role', 'tabpanel');
const feedStatus = el('div', 'module-status feed-status'); feedStatus.setAttribute('aria-live', 'polite');
const pendingButton = button('有新内容，点击查看', () => { feeds[state.selected].acceptPending(); list.scrollTop = 0; }, 'pending-button'); pendingButton.hidden = true;
const list = el('div', 'feed-list'); list.tabIndex = 0; list.setAttribute('aria-label', '内容列表');
const moreButton = button('加载更多', () => void feeds[state.selected].more(), 'more-button');
feedPanel.append(feedStatus, pendingButton, list, moreButton); reader.append(feedPanel);
const footer = el('footer', 'app-footer');
const connection = button('未连接', () => void commands.openSettings().catch(topError), 'connection-state');
const footerNote = el('span', '', '非官方 · 北京时间');
footer.append(connection, footerNote); app.append(footer);

for (const name of ['flash', 'news', 'calendar']) {
  const fetcher = name === 'calendar' ? async ({ keyword }) => {
    const value = await commands.calendar();
    const items = Array.isArray(value) ? value : value?.important;
    if (!Array.isArray(items)) throw new Error('日历数据格式暂不受支持');
    return items.filter(item => !keyword || `${item.title || ''} ${item.pub_time || ''}`.toLowerCase().includes(keyword.toLowerCase()));
  } : commands[name];
  feeds[name] = new Feed(fetcher, snapshot => {
    if (snapshot.error?.code === 'rate_limited' || snapshot.error?.code === 'auth_failed') scheduler.pause(name);
    if (name === state.selected) renderFeed();
  });
}

let scheduler = createScheduler();

function createScheduler() {
  const generation = dataGeneration;
  return new Scheduler({
    quotes: () => refreshQuotes(generation),
    flash: () => generation === dataGeneration ? feeds.flash.refresh() : Promise.resolve(),
    news: () => generation === dataGeneration ? feeds.news.refresh() : Promise.resolve(),
    calendar: () => generation === dataGeneration ? feeds.calendar.refresh() : Promise.resolve(),
  });
}

function changeDataSource(config) {
  dataGeneration += 1;
  readyGeneration = null;
  scheduler.stop();
  for (const feed of Object.values(feeds)) feed.reset();
  state.quotes = []; state.quotesAt = null; state.quotesError = null; state.expanded.clear();
  searchInput.value = ''; renderedFeedKey = ''; topMessage.hidden = true; quoteRefresh.disabled = false;
  for (const dialog of app.querySelectorAll('.detail-dialog')) dialog.remove();
  // 新调度器不等待旧账户的在途请求；旧调度器已停止，旧响应由世代校验丢弃。
  scheduler = createScheduler();
  renderQuotes(); renderFeed();
  renderConnection({ process: 'ok', state: config.mcp?.hasToken ? 'connecting' : 'unconfigured', configured: Boolean(config.mcp?.hasToken), lastSuccessAt: null, lastError: null });
  if (state.mini && !config.mcp?.hasToken) void toggleMini();
  applyConfig(config);
}

function topError(error) { showError(topMessage, error); }

function recoverReadyConnection() {
  if (readyGeneration !== dataGeneration) return;
  const generation = dataGeneration;
  scheduler.recoverWhenSettled(
    name => name === 'quotes' ? Boolean(state.quotesError) : Boolean(feeds[name].snapshot.error),
    () => generation === dataGeneration && !document.hidden && Boolean(state.config?.mcp?.hasToken),
  );
}

async function toggleMini() {
  const enabled = !state.mini;
  try {
    await commands.mini(enabled); state.mini = enabled; maximizeButton.disabled = enabled; app.classList.toggle('is-mini', enabled);
    miniButton.replaceChildren(icon(enabled ? 'expand' : 'mini'), el('span', '', enabled ? '恢复阅读窗口' : '进入迷你行情'));
    miniButton.setAttribute('aria-label', enabled ? '恢复阅读窗口' : '进入迷你行情');
    miniButton.title = enabled ? '恢复阅读窗口' : '进入迷你行情';
    footerNote.textContent = enabled && environment.demo ? '演示数据 · 非真实行情' : '非官方 · 北京时间';
  } catch (error) { topError(error); }
}

function applyConfig(config) {
  state.config = config; applyTheme(config.theme, config.opacity);
  const configured = Boolean(config.mcp?.hasToken);
  onboarding.hidden = configured; workspace.hidden = !configured; miniButton.disabled = !configured;
  if (config.configWarning) topError(config.configWarning);
  if (configured && !document.hidden) scheduler.start(); else scheduler.stop();
}

function renderConnection(health) {
  state.health = health;
  if (health.configWarning) topError(health.configWarning);
  const text = { unconfigured: '连接数据源', connecting: '正在连接', connected: '已连接', offline: '连接中断', auth_failed: '认证失败', rate_limited: '配额受限', error: '连接异常' };
  connection.textContent = text[health.state] || '状态未知';
  connection.dataset.state = health.state;
  connection.title = health.lastError?.message || (health.lastSuccessAt ? `最近业务成功 ${timeText(health.lastSuccessAt, true)}` : '尚无成功业务请求');
}

async function refreshQuotes(generation = dataGeneration) {
  if (generation !== dataGeneration) return;
  quoteRefresh.disabled = true;
  try {
    const result = await commands.quotes();
    if (generation !== dataGeneration) return;
    if (!Array.isArray(result)) throw new Error('行情数据格式暂不受支持');
    state.quotes = result; state.quotesAt = Date.now(); state.quotesError = null;
  } catch (error) {
    if (generation !== dataGeneration) return;
    state.quotesError = normalizeError(error);
    if (['rate_limited', 'auth_failed'].includes(state.quotesError.code)) scheduler.pause('quotes');
  } finally { if (generation === dataGeneration) { quoteRefresh.disabled = false; renderQuotes(); } }
}

function renderQuotes() {
  const map = new Map(state.quotes.filter(Boolean).map(q => [q.code, q]));
  quoteGrid.replaceChildren();
  for (const instrument of INSTRUMENTS) {
    const quote = map.get(instrument.code);
    const price = numberValue(quote?.close); const change = numberValue(quote?.ups_percent);
    const card = el('article', 'quote-card');
    const label = el('div', 'quote-label', instrument.name); label.title = instrument.name;
    const priceNode = el('div', 'quote-price', price === null ? '—' : price.toLocaleString('en-US', { minimumFractionDigits: instrument.digits, maximumFractionDigits: instrument.digits }));
    const changeNode = el('span', `quote-change ${change === null || change === 0 ? 'flat' : change > 0 ? 'up' : 'down'}`, change === null ? '暂无涨跌' : `${change > 0 ? '↑ +' : change < 0 ? '↓ ' : ''}${change.toFixed(2)}%`);
    const at = parseDate(quote?.time);
    const stale = Boolean(at && Date.now() - at.valueOf() > INTERVALS.quotes * 2);
    if (stale) card.classList.add('is-stale');
    const time = el('span', 'quote-time', at ? `${timeText(at)}${stale ? ' · 旧快照' : ''}` : '时间未提供');
    card.append(label, priceNode, el('span', 'quote-code', instrument.code), changeNode, time);
    quoteGrid.append(card);
  }
  const missing = INSTRUMENTS.filter(i => numberValue(map.get(i.code)?.close) === null).length;
  quoteStatus.classList.toggle('is-error', Boolean(state.quotesError));
  market.classList.toggle('has-error', Boolean(state.quotesError));
  quoteStatus.textContent = state.quotesError ? `${state.quotesAt ? '保留旧快照 · ' : ''}${state.quotesError.message}` : state.quotesAt ? `获取于 ${timeText(state.quotesAt)}${missing ? ` · ${missing} 个品种暂无数据` : ' · 价格旁为数据时间'}` : '等待获取行情快照';
}

function selectTab(name) {
  state.selected = name;
  for (const [key, tab] of Object.entries(tabButtons)) { tab.setAttribute('aria-selected', String(key === name)); tab.tabIndex = key === name ? 0 : -1; }
  feedPanel.setAttribute('aria-labelledby', `tab-${name}`);
  searchInput.value = feeds[name].keyword; searchInput.placeholder = `搜索${labels[name].replace('最新', '').replace('深度', '')}`;
  list.scrollTop = 0; renderFeed();
}

function renderFeed() {
  const snapshot = feeds[state.selected].snapshot;
  searchClear.hidden = !snapshot.keyword && !searchInput.value;
  feedRefresh.disabled = snapshot.loading;
  feedStatus.classList.toggle('is-error', Boolean(snapshot.error));
  const stale = snapshot.updatedAt && Date.now() - snapshot.updatedAt > INTERVALS[state.selected] * 2;
  feedStatus.textContent = snapshot.error ? `${snapshot.items.length ? '显示上次内容 · ' : ''}${snapshot.error.message}` : snapshot.loading ? '正在获取内容…' : snapshot.keyword ? `“${snapshot.keyword}”的搜索结果 · 已显示 ${snapshot.items.length} 条` : snapshot.updatedAt ? `${stale ? '内容可能已过期 · ' : ''}获取于 ${timeText(snapshot.updatedAt)} · ${state.selected === 'flash' ? '每 5 分钟' : '每小时'}刷新` : '等待获取内容';
  pendingButton.hidden = !snapshot.pending;
  pendingButton.textContent = snapshot.pending ? `最新内容已就绪 · 点击查看` : '';
  const renderKey = JSON.stringify([state.selected, snapshot.keyword, snapshot.items, snapshot.items.length ? null : [snapshot.loading, snapshot.error]]);
  if (renderKey !== renderedFeedKey) {
    renderedFeedKey = renderKey;
    const scrollTop = list.scrollTop;
    list.replaceChildren();
    if (!snapshot.items.length) {
      const empty = el('div', 'empty-state');
      empty.append(icon(snapshot.keyword ? 'search' : 'mark'), el('h2', '', snapshot.loading ? '稍候，内容正在路上' : snapshot.error ? '暂时无法获取内容' : snapshot.keyword ? '没有找到匹配内容' : '暂时没有新内容'), el('p', '', snapshot.error ? '你可以重试，或到设置中检查数据源。' : snapshot.keyword ? '换一个关键词，或清除搜索继续阅读。' : '新的内容将在下次刷新时出现。'));
      if (snapshot.error) empty.append(button('重试', () => void scheduler.run(state.selected, true), 'button secondary', 'refresh'));
      list.append(empty);
    } else for (const item of snapshot.items) list.append(state.selected === 'calendar' ? calendarItem(item) : articleItem(item));
    list.scrollTop = scrollTop;
    requestAnimationFrame(updateExpansion);
  }
  moreButton.disabled = snapshot.loading || !snapshot.loaded || !snapshot.hasMore;
  moreButton.hidden = !snapshot.items.length;
  moreButton.textContent = snapshot.loading ? '正在加载…' : snapshot.hasMore ? '加载更多' : '已读到这里 · 暂无更多';
}

function articleItem(item) {
  const article = el('article', `feed-item ${state.selected === 'news' ? 'news-item' : 'flash-item'}`);
  const meta = el('div', 'item-meta');
  const time = el('time', '', timeText(item.time, true)); const parsed = parseDate(item.time); if (parsed) time.dateTime = parsed.toISOString();
  const actions = el('div', 'item-actions');
  const link = externalLink('原文', item.url, commands.openExternal, topError); if (link) actions.append(link);
  const content = String(state.selected === 'news' ? item.introduction || '' : item.content || item.title || '正文未提供');
  const copy = button('复制', async () => {
    try { await commands.copy(`${item.title || ''}${item.title ? '\n' : ''}${content}`); copy.textContent = '已复制'; setTimeout(() => { copy.textContent = '复制'; }, 1500); } catch (error) { topError(error); }
  }, 'text-button'); actions.append(copy); meta.append(time, actions); article.append(meta);
  if (state.selected === 'news') article.append(el('h2', 'article-title', item.title || '未命名资讯'));
  const text = el('p', 'article-content', content); const key = `${state.selected}:${itemKey(item)}`;
  const expanded = state.expanded.has(key); text.classList.toggle('clamped', !expanded);
  article.append(text);
  if (content) {
    const expand = button(expanded ? '收起全文' : '展开全文', () => {
      const isExpanded = state.expanded.has(key);
      if (isExpanded) state.expanded.delete(key); else state.expanded.add(key);
      text.classList.toggle('clamped', isExpanded); expand.textContent = isExpanded ? '展开全文' : '收起全文'; expand.setAttribute('aria-expanded', String(!isExpanded));
    }, 'text-button expand-button');
    expand.setAttribute('aria-expanded', String(expanded)); expand.hidden = !expanded; article.append(expand);
  }
  if (state.selected === 'news' && item.id && !item.url) {
    const detail = button('阅读详情', async () => {
      const generation = dataGeneration;
      detail.disabled = true;
      try { const result = await commands.detail(item.id); if (generation === dataGeneration) showDetail(result); } catch (error) { if (generation === dataGeneration) topError(error); } finally { detail.disabled = false; }
    }, 'text-button'); article.append(detail);
  }
  return article;
}

function updateExpansion() {
  for (const article of list.querySelectorAll('.feed-item')) {
    const text = article.querySelector('.article-content');
    const expand = article.querySelector('.expand-button');
    if (text && expand) expand.hidden = text.classList.contains('clamped') && text.scrollHeight <= text.clientHeight + 1;
  }
}

function showDetail(data) {
  const dialog = el('dialog', 'detail-dialog');
  dialog.append(button('关闭详情', () => { dialog.close(); dialog.remove(); }, 'button secondary', 'close'), el('h2', '', data.title || '资讯详情'), el('div', 'detail-content', data.content || data.introduction || '正文暂未提供'));
  app.append(dialog); dialog.showModal(); dialog.addEventListener('close', () => dialog.remove());
}

function calendarItem(item) {
  const article = el('article', 'feed-item calendar-item');
  const meta = el('div', 'item-meta'); const stars = Math.max(0, Math.min(5, Math.round(numberValue(item.star) || 0)));
  const importance = el('span', 'importance', '●'.repeat(stars)); importance.setAttribute('aria-label', `重要程度 ${stars} 星`);
  meta.append(el('time', '', timeText(item.pub_time, true)), importance);
  const values = el('dl', 'calendar-values');
  for (const [label, key] of [['前值', 'previous'], ['预期', 'consensus'], ['实际', 'actual']]) {
    const group = el('div'); group.append(el('dt', '', label), el('dd', key === 'actual' ? 'actual' : '', item[key] === null || item[key] === undefined || item[key] === '' ? '—' : item[key])); values.append(group);
  }
  article.append(meta, el('h2', 'article-title', item.title || '未命名事件'), values);
  if (item.revised !== null && item.revised !== undefined && item.revised !== '') article.append(el('p', 'revised-value', `前值修正：${item.revised}`));
  return article;
}

searchInput.addEventListener('input', () => { searchClear.hidden = !searchInput.value && !feeds[state.selected].keyword; });
document.addEventListener('visibilitychange', () => {
  if (document.hidden) scheduler.stop();
  else if (state.config?.mcp?.hasToken) {
    const needsInitial = !state.quotesAt && Object.values(feeds).every(feed => !feed.normal.loaded);
    scheduler.start(needsInitial); recoverReadyConnection(); renderQuotes(); renderFeed();
  }
});
matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => { if (state.config) applyTheme(state.config.theme, state.config.opacity); });
window.addEventListener('beforeunload', () => { scheduler.stop(); unlisten.forEach(stop => stop()); });
window.addEventListener('resize', updateExpansion);

async function init() {
  renderQuotes(); selectTab('flash');
  try {
    unlisten.push(await commands.listen('data-source-changed', changeDataSource));
    unlisten.push(await commands.listen('config-updated', applyConfig));
    unlisten.push(await commands.listen('connection-state', renderConnection));
    unlisten.push(await commands.listen('mcp-ready', () => {
      readyGeneration = dataGeneration;
      if (state.config?.mcp?.hasToken && !document.hidden) scheduler.start();
      recoverReadyConnection();
    }));
    unlisten.push(await commands.listen('refresh-all', () => { if (state.config?.mcp?.hasToken) Object.keys(INTERVALS).forEach(name => void scheduler.run(name, true)); }));
    const generation = dataGeneration;
    const [config, health] = await Promise.all([commands.getConfig(), commands.health()]);
    if (generation === dataGeneration) { applyConfig(config); renderConnection(health); }
  } catch (error) { onboarding.hidden = false; topError(error); }
}
void init();
