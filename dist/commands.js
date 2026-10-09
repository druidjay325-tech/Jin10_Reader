import { normalizeError, safeUrl } from './model.js';

const native = globalThis.__TAURI__;
const demo = !native && new URLSearchParams(location.search).get('demo') === '1';
let demoApi;
export const environment = Object.freeze({ native: Boolean(native), demo });

/** 所有 IPC 都从此处进入，页面不会接触或读取已存凭据。 */
async function call(command, args = {}) {
  try {
    if (native) return await native.core.invoke(command, args);
    if (demo) { demoApi ||= await import('./demo.js'); return await demoApi.call(command, args); }
    if (command === 'get_config') return defaultConfig();
    if (command === 'health') return { process: 'ok', state: 'unconfigured', configured: false, lastSuccessAt: null, lastError: null };
    throw { code: 'preview_only', message: '当前为浏览器预览。请在桌面应用中连接数据源。', retryable: false };
  } catch (error) { throw normalizeError(error); }
}

function defaultConfig() {
  return { theme: 'dark', opacity: 0.95, autoStart: false, minimizeOnStart: false, shortcuts: { toggleWindow: 'Ctrl+Alt+J', refresh: 'Ctrl+Alt+R' }, mcp: { serverUrl: 'https://mcp.jin10.com/mcp', hasToken: false }, configWarning: null };
}

export const commands = Object.freeze({
  getConfig: () => call('get_config'),
  saveConfig: config => call('save_config', { config }),
  health: () => call('health'),
  testConnection: (serverUrl, bearerToken) => call('test_mcp', { serverUrl, bearerToken }),
  reconnect: () => call('restart_mcp'),
  quotes: () => call('get_quotes', { codes: ['XAUUSD', 'XAGUSD', 'USOIL', 'UKOIL', 'USDCNH', 'EURUSD'] }),
  flash: ({ keyword, cursor }) => call(keyword ? 'search_flash' : 'get_flash', keyword ? { keyword, cursor } : { cursor }),
  news: ({ keyword, cursor }) => call(keyword ? 'search_news' : 'get_news', keyword ? { keyword, cursor, limit: 20 } : { cursor }),
  calendar: () => call('get_calendar'),
  detail: id => call('get_news_detail', { id: String(id) }),
  async openExternal(url) {
    const href = safeUrl(url);
    if (!href) throw { code: 'invalid_url', message: '该链接使用不受支持的地址格式。', retryable: false };
    if (native) return call('open_external', { url: href });
    window.open(href, '_blank', 'noopener,noreferrer');
  },
  async openSettings() {
    if (native) return call('open_settings');
    location.href = `settings.html${demo ? '?demo=1' : ''}`;
  },
  async closeSettings() {
    if (native) return native.window.getCurrentWindow().hide();
    location.href = `index.html${demo ? '?demo=1' : ''}`;
  },
  async hide() { if (native) return native.window.getCurrentWindow().hide(); },
  async mini(enabled) { if (native) return call('set_mini_mode', { enabled }); },
  async maximize() { if (native) return call('toggle_maximize'); },
  async copy(text) {
    if (!navigator.clipboard?.writeText) throw { code: 'clipboard', message: '当前环境无法访问剪贴板。', retryable: false };
    await navigator.clipboard.writeText(text);
  },
  async listen(name, callback) {
    if (!native) return () => {};
    return native.event.listen(name, event => callback(event.payload));
  },
});
