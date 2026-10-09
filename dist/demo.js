// 仅在浏览器显式打开 ?demo=1 时载入。所有内容均为合成展示，不连接外部服务。
let config = { theme: 'dark', opacity: 0.95, autoStart: false, minimizeOnStart: false, shortcuts: { toggleWindow: 'Ctrl+Alt+J', refresh: 'Ctrl+Alt+R' }, mcp: { serverUrl: 'https://mcp.jin10.com/mcp', hasToken: true }, configWarning: null };
const now = Date.now();
const headlines = [
  '全球市场观察：投资者等待新一轮经济数据，贵金属价格窄幅波动。',
  '能源市场观察：原油供需预期保持分歧，市场关注库存变化与下一阶段需求。',
  '外汇市场观察：美元指数维持区间整理，主要货币走势分化。',
  '宏观观察：多项经济指标即将公布，市场重新评估增长与通胀路径。',
  '盘前阅读：从债券收益率、汇率与商品价格理解市场情绪变化。',
  '欧洲市场开盘观察：交易者关注经济景气数据与企业财报。',
  '亚太市场收盘观察：主要指数涨跌不一，成交保持平稳。',
  '市场日历提醒：本周重点数据集中公布，请留意实际发布时间。',
  '长期视角：短期价格波动并不代表基本面已经发生改变。',
  '风险观察：跨市场相关性可能变化，阅读时请核对数据来源与时间。',
];
function items(kind) {
  return Array.from({ length: 24 }, (_, i) => ({ id: `${kind}-${i}`, title: headlines[i % headlines.length], content: headlines[i % headlines.length], introduction: '这是一条离线合成示例，用于展示阅读排版、搜索与分页。它不描述真实市场事件，也不构成投资建议。', time: new Date(now - i * 180000).toISOString(), url: 'https://www.jin10.com/' }));
}
function page(kind, { cursor, keyword }) {
  const all = items(kind).filter(item => !keyword || `${item.title}${item.introduction}`.includes(keyword));
  const start = Number(cursor || 0); const next = start + 10;
  return { items: all.slice(start, next), has_more: next < all.length, next_offset: next < all.length ? String(next) : null };
}
export async function call(command, args) {
  await new Promise(resolve => setTimeout(resolve, 90));
  if (command === 'get_config') return structuredClone(config);
  if (command === 'save_config') {
    const patch = structuredClone(args.config); const connectionChanged = Boolean(patch.mcp);
    if (patch.mcp) { config.mcp = { serverUrl: patch.mcp.serverUrl || config.mcp.serverUrl, hasToken: patch.mcp.clearToken ? false : Boolean(patch.mcp.bearerToken || config.mcp.hasToken) }; delete patch.mcp; }
    config = { ...config, ...patch };
    return { saved: true, connectionChanged, config: structuredClone(config) };
  }
  if (command === 'health' || command === 'restart_mcp') return { process: 'ok', state: config.mcp.hasToken ? 'connected' : 'unconfigured', configured: config.mcp.hasToken, lastSuccessAt: now, lastError: null };
  if (command === 'test_mcp') return { ok: true, tools: ['get_quote', 'list_flash', 'list_news', 'list_calendar'] };
  if (command === 'get_quotes') return args.codes.map((code, i) => ({ code, close: [2635.72, 31.428, 72.56, 76.11, 7.0834, 1.1028][i], ups_percent: [0.42, 0.68, -0.37, -0.29, 0.06, -0.14][i], time: new Date(now).toISOString() }));
  if (command === 'get_flash' || command === 'search_flash') return page('flash', args);
  if (command === 'get_news' || command === 'search_news') return page('news', args);
  if (command === 'get_news_detail') return { title: '市场阅读示例', content: '这是一段合成文章详情。所有示例数据仅用于离线演示，不连接真实数据源。' };
  if (command === 'get_calendar') return Array.from({ length: 12 }, (_, i) => ({ id: `cal-${i}`, title: ['美国初请失业金人数', '欧元区零售销售月率', '中国外汇储备', '美国消费者信心指数'][i % 4], pub_time: new Date(now + i * 3600000).toISOString(), star: 3 + i % 3, previous: '示例 4.2', consensus: '示例 4.1', actual: i === 0 ? '示例 4.3' : null }));
  throw new Error('演示环境不支持此操作');
}
