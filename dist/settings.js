import { commands, environment } from './commands.js';
import { normalizeError } from './model.js';
import { el, icon, button, iconButton, applyTheme, timeText, showError } from './ui.js';

const app = document.getElementById('app');
let original = null;
let saving = false;
let dirty = false;
let clearTokenPending = false;
let materialStatus = null;
let connectionGeneration = 0;
let loadGeneration = 0;
const inputs = {};
const header = el('header', 'titlebar'); header.setAttribute('data-tauri-drag-region', '');
const title = el('div', 'settings-window-title', '偏好设置'); title.setAttribute('data-tauri-drag-region', '');
header.append(title, iconButton('取消修改并关闭设置', 'close', () => void cancel())); app.append(header);
if (environment.demo) app.append(el('div', 'demo-banner', '演示设置 · 仅本页内存有效'));
const form = el('form', 'settings-form'); app.append(form);
const content = el('main', 'settings-content'); form.append(content);
const banner = el('div', 'settings-message'); banner.hidden = true; banner.setAttribute('aria-live', 'polite'); content.append(banner);

function section(titleText, caption) {
  const node = el('section', 'settings-section');
  const heading = el('div', 'settings-heading'); heading.append(el('h2', '', titleText)); if (caption) heading.append(el('span', 'section-caption', caption));
  node.append(heading); content.append(node); return node;
}
function field(parent, id, labelText, type = 'text', hint) {
  const row = el('div', 'field'); const label = el('label', '', labelText); label.htmlFor = id;
  const input = el('input', 'form-input'); input.id = id; input.name = id; input.type = type; inputs[id] = input;
  input.autocomplete = 'off'; row.append(label, input);
  if (hint) { const description = el('p', 'field-help', hint); description.id = `${id}-help`; input.setAttribute('aria-describedby', description.id); row.append(description); }
  parent.append(row); return input;
}
function checkbox(parent, id, labelText, hint) {
  const row = el('label', 'check-row'); row.htmlFor = id;
  const input = el('input'); input.id = id; input.type = 'checkbox'; inputs[id] = input;
  const copy = el('span', 'check-copy'); copy.append(el('span', '', labelText)); if (hint) copy.append(el('span', 'field-help', hint));
  row.append(input, copy); parent.append(row); return input;
}

const account = section('数据源', '使用自己的访问凭据');
const accountHeading = account.querySelector('.settings-heading'); accountHeading.classList.add('account-heading');
accountHeading.append(button('获取 Token / 官方教程', () => void commands.openExternal('https://mcp.jin10.com/app/').catch(error => message(normalizeError(error).message, true)), 'button secondary official-link', 'external'));
const server = field(account, 'serverUrl', 'MCP 服务地址', 'url', '使用 HTTPS 地址。更换服务地址时，请同时输入新 Token。'); server.required = true; server.spellcheck = false;
const token = field(account, 'bearerToken', '新 Token', 'password', '留空保留现有凭据。保存后不会再次显示 Token。'); token.spellcheck = false; token.maxLength = 8192;
const tokenState = el('p', 'token-state'); account.append(tokenState);
const clearToken = button('清除已存 Token', () => {
  clearTokenPending = !clearTokenPending; updateTokenClear();
  if (clearTokenPending) token.value = '';
  clearToken.dispatchEvent(new Event('input', { bubbles: true }));
}, 'button secondary token-clear'); clearToken.id = 'clearToken';
const accountActions = el('div', 'button-row account-actions');
const test = button('测试连接', () => void testConnection(), 'button secondary', 'shield');
const reconnect = button('重连已存数据源', () => void reconnectSaved(), 'button secondary', 'refresh');
accountActions.append(test, reconnect, clearToken); account.append(accountActions);
account.append(el('p', 'field-help token-clear-help', '清除 Token 仅标记待清除，保存设置后生效；可再次点击撤销。'));
const connectionResult = el('p', 'inline-result'); connectionResult.setAttribute('aria-live', 'polite'); account.append(connectionResult);

const appearance = section('外观', '轻盈、清晰、少打扰');
const themeRow = el('div', 'theme-row'); const themeLabel = el('span', 'theme-label', '主题'); themeLabel.id = 'theme-label';
const theme = { value: 'dark' };
const themeGroup = el('div', 'theme-options'); themeGroup.id = 'theme'; themeGroup.setAttribute('role', 'radiogroup'); themeGroup.setAttribute('aria-labelledby', 'theme-label');
const themeChoices = [['light', '浅色'], ['dark', '墨色'], ['system', '跟随系统'], ['glass', '毛玻璃']];
for (const [value, label] of themeChoices) {
  const choice = button(label, () => selectTheme(value), 'theme-choice'); choice.dataset.theme = value;
  choice.setAttribute('role', 'radio'); choice.setAttribute('aria-checked', 'false'); themeGroup.append(choice);
}
themeGroup.addEventListener('keydown', event => {
  if (!['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Home', 'End'].includes(event.key)) return;
  event.preventDefault(); const index = themeChoices.findIndex(([value]) => value === theme.value);
  const next = event.key === 'Home' ? 0 : event.key === 'End' ? 3 : (index + (['ArrowLeft', 'ArrowUp'].includes(event.key) ? -1 : 1) + 4) % 4;
  selectTheme(themeChoices[next][0]); themeGroup.children[next].focus();
});
themeRow.append(themeLabel, themeGroup); appearance.append(themeRow);
const opacityRow = el('div', 'field'); const opacityLabel = el('label', '', '毛玻璃模式不透明度'); opacityLabel.htmlFor = 'opacity';
const opacity = el('input', 'range-input'); opacity.type = 'range'; opacity.min = '0'; opacity.max = '100'; opacity.step = '1'; opacity.id = 'opacity'; inputs.opacity = opacity;
const opacityOutput = el('output', 'range-value', '95%'); opacityOutput.htmlFor = 'opacity';
const range = el('div', 'range-row'); range.append(opacity, opacityOutput); opacityRow.append(opacityLabel, range, el('p', 'field-help', '仅在毛玻璃模式下可调，保存后生效。文字与按钮保持清晰；墨色、浅色始终完全不透明。')); appearance.append(opacityRow);
const luminosityRow = el('div', 'field');
const luminosityLabel = el('label', '', '亮度层不透明度'); luminosityLabel.htmlFor = 'glassLuminosity';
const luminosity = el('input', 'range-input'); luminosity.id = 'glassLuminosity'; luminosity.type = 'range'; luminosity.min = '0'; luminosity.max = '100'; luminosity.step = '1';
const luminosityOutput = el('output', 'range-value', '0%'); luminosityOutput.htmlFor = 'glassLuminosity';
const luminosityRange = el('div', 'range-row'); luminosityRange.append(luminosity, luminosityOutput);
luminosityRow.append(luminosityLabel, luminosityRange, el('p', 'field-help', '控制原生毛玻璃的明暗底材。降低可减少灰底，让背景更清透；仅毛玻璃模式可调，保存后生效。')); appearance.append(luminosityRow);
const materialInfo = el('p', 'field-help'); materialInfo.setAttribute('aria-live', 'polite'); appearance.append(materialInfo);
const runtimeLink = button('毛玻璃运行环境 / 官方下载', () => void commands.openExternal('https://learn.microsoft.com/en-us/windows/apps/windows-app-sdk/downloads').catch(error => message(normalizeError(error).message, true)), 'text-button', 'external'); runtimeLink.hidden = true; appearance.append(runtimeLink);

const general = section('启动与快捷键');
checkbox(general, 'autoStart', '登录 Windows 后自动启动');
checkbox(general, 'minimizeOnStart', '启动时隐藏到托盘');
const shortcutHint = '例如 Ctrl+Alt+J。可分别修改；与旧版并行时请使用不同组合。Windows 保留 F12 供调试使用。';
field(general, 'toggleWindow', '显示 / 隐藏阅读器', 'text', shortcutHint);
field(general, 'refresh', '刷新所有栏目', 'text', '例如 Ctrl+Alt+R。快捷键刷新会发起数据请求。');
const shortcutStatus = el('p', 'field-help'); general.append(shortcutStatus);

const status = section('连接状态', '如实记录最近结果');
const statusList = el('dl', 'status-list'); status.append(statusList);
const statusFields = {};
for (const [key, label] of [['process', '应用进程'], ['state', '数据源连接'], ['lastSuccessAt', '最近业务成功'], ['lastError', '最近错误']]) {
  const row = el('div'); const value = el('dd', '', '—'); row.append(el('dt', '', label), value); statusList.append(row); statusFields[key] = value;
}
status.append(button('更新状态', () => void refreshHealth(), 'text-button', 'refresh'));
const about = el('div', 'settings-about'); about.append(el('strong', '', 'Jin10 Reader · 金十阅读器'), el('p', '', '独立开发的非官方阅读器。新闻仅保留在当前会话内存中；关闭后即释放。数据内容和服务授权归原提供方。')); content.append(about);

const footer = el('footer', 'settings-footer');
const draftState = el('span', 'draft-state', '修改后，保存才会生效');
const footerActions = el('div', 'button-row');
const cancelButton = button('取消', () => void cancel(), 'button secondary');
const save = button('保存设置', null, 'button primary'); save.type = 'submit';
footerActions.append(cancelButton, save); footer.append(draftState, footerActions); form.append(footer);

function clearMessage() { banner.hidden = true; banner.classList.remove('is-error'); }
function selectTheme(value) {
  theme.value = value; dirty = true; draftState.textContent = '有未保存的修改'; clearMessage(); updateAppearance();
}
function updateAppearance() {
  themeGroup.dataset.value = theme.value;
  for (const choice of themeGroup.children) {
    const selected = choice.dataset.theme === theme.value;
    choice.setAttribute('aria-checked', String(selected)); choice.tabIndex = selected ? 0 : -1; choice.classList.toggle('is-selected', selected);
  }
  opacity.disabled = theme.value !== 'glass';
  luminosity.disabled = theme.value !== 'glass' || (original?.theme === 'glass' && materialStatus?.engine === 'preset');
  luminosityOutput.textContent = `${luminosity.value}%`;
  luminosityRow.classList.toggle('is-disabled', luminosity.disabled);
  opacityOutput.textContent = `${opacity.value}%`;
  opacityRow.classList.toggle('is-disabled', opacity.disabled);
  applyTheme(theme.value, Number(opacity.value) / 100);
  materialInfo.textContent = theme.value === 'glass' ? (materialStatus?.warning || (materialStatus?.engine === 'official' ? '已使用官方可调毛玻璃。' : '')) : '';
  runtimeLink.hidden = theme.value !== 'glass' || materialStatus?.engine !== 'preset';
}
function updateTokenClear() {
  clearToken.setAttribute('aria-pressed', String(clearTokenPending));
  clearToken.classList.toggle('is-selected', clearTokenPending);
  clearToken.querySelector('span').textContent = clearTokenPending ? '撤销清除 Token' : '清除已存 Token';
  token.disabled = clearTokenPending;
}
function message(text, failed = false) {
  banner.textContent = text; banner.hidden = false; banner.classList.toggle('is-error', failed); banner.setAttribute('role', failed ? 'alert' : 'status');
}
function fill(config) {
  connectionGeneration += 1;
  original = structuredClone(config); dirty = false;
  server.value = config.mcp?.serverUrl || 'https://mcp.jin10.com/mcp'; token.value = ''; clearTokenPending = false; updateTokenClear();
  theme.value = config.theme || 'dark'; opacity.value = String(Math.round((config.opacity ?? 0.95) * 100)); opacityOutput.textContent = `${opacity.value}%`;
  luminosity.value=String(Math.round((config.glassLuminosity??0)*100));
  inputs.autoStart.checked = Boolean(config.autoStart); inputs.minimizeOnStart.checked = Boolean(config.minimizeOnStart);
  inputs.toggleWindow.value = config.shortcuts?.toggleWindow || 'Ctrl+Alt+J'; inputs.refresh.value = config.shortcuts?.refresh || 'Ctrl+Alt+R';
  tokenState.textContent = config.mcp?.hasToken ? '已保存 Token · 输入新值可替换' : '尚未配置 Token';
  token.placeholder = config.mcp?.hasToken ? '留空保留现有 Token' : '填写你的金十 MCP Token';
  reconnect.disabled = !config.mcp?.hasToken; clearToken.disabled = !config.mcp?.hasToken;
  test.disabled = false;
  save.disabled = saving || Boolean(config.configReadOnly); draftState.textContent = '修改后，保存才会生效'; updateAppearance();
  if (config.configWarning) message(config.configWarning, Boolean(config.configReadOnly));
}
async function load() {
  const generation = ++loadGeneration;
  connectionGeneration += 1;
  try { const config = await commands.getConfig(); if (generation !== loadGeneration) return; fill(config); await refreshHealth(); } catch (error) { if (generation === loadGeneration) { showError(banner, error); save.disabled = true; } }
}
async function refreshHealth() {
  try {
    const health = await commands.health();
    materialStatus=health.material||null;updateAppearance();
    if (health.shortcutStatus) {
      shortcutStatus.textContent = `显示/隐藏：${health.shortcutStatus.toggleWindow ? '已生效' : '未注册'} · 刷新：${health.shortcutStatus.refresh ? '已生效' : '未注册'}`;
    }
    if (health.configWarning) {
      if (health.shortcutStatus && (!health.shortcutStatus.toggleWindow || !health.shortcutStatus.refresh)) shortcutStatus.textContent += `。${health.configWarning}`;
      else message(health.configWarning, true);
    }
    statusFields.process.textContent = health.process === 'ok' ? '运行中' : '状态未知';
    statusFields.state.textContent = ({ unconfigured: '尚未配置', connecting: '正在连接', connected: '已连接', offline: '离线', auth_failed: '认证失败', rate_limited: '配额受限', error: '连接异常' })[health.state] || '未知';
    statusFields.lastSuccessAt.textContent = health.lastSuccessAt ? timeText(health.lastSuccessAt, true) : '尚无成功请求';
    statusFields.lastError.textContent = health.lastError?.message || '无';
  } catch (error) { statusFields.lastError.textContent = normalizeError(error).message; }
}
async function testConnection() {
  if (clearTokenPending) return;
  const generation = ++connectionGeneration;
  const endpoint = server.value.trim();
  const newToken = token.value.trim();
  test.disabled = true; connectionResult.textContent = '正在验证连接…'; connectionResult.classList.remove('is-error');
  try {
    await commands.testConnection(endpoint, newToken);
    if (generation !== connectionGeneration) return;
    connectionResult.textContent = '连接测试通过。尚未保存，也未改变当前数据源。';
  } catch (error) { if (generation === connectionGeneration) { connectionResult.textContent = normalizeError(error).message; connectionResult.classList.add('is-error'); } }
  finally { if (generation === connectionGeneration) test.disabled = clearTokenPending; }
}
async function reconnectSaved() {
  const generation = ++connectionGeneration;
  reconnect.disabled = true; connectionResult.textContent = '正在连接已保存的数据源…';
  try { await commands.reconnect(); if (generation !== connectionGeneration) return; connectionResult.textContent = '已连接到保存的数据源。'; connectionResult.classList.remove('is-error'); }
  catch (error) { if (generation === connectionGeneration) { connectionResult.textContent = normalizeError(error).message; connectionResult.classList.add('is-error'); } }
  finally { if (generation === connectionGeneration) reconnect.disabled = !original?.mcp?.hasToken; await refreshHealth(); }
}
async function cancel() {
  if (saving) return;
  connectionGeneration += 1; loadGeneration += 1;
  if (original) fill(original);
  clearMessage(); connectionResult.textContent = '';
  await commands.closeSettings().catch(error => message(normalizeError(error).message, true));
}
function makePatch() {
  const patch = { theme: theme.value, opacity: Number(opacity.value) / 100, glassLuminosity:Number(luminosity.value)/100, autoStart: inputs.autoStart.checked, minimizeOnStart: inputs.minimizeOnStart.checked, shortcuts: { toggleWindow: inputs.toggleWindow.value.trim(), refresh: inputs.refresh.value.trim() } };
  if (server.value.trim() !== original.mcp.serverUrl || token.value.trim() || clearTokenPending) {
    patch.mcp = { serverUrl: server.value.trim() };
    if (clearTokenPending) patch.mcp.clearToken = true;
    else if (token.value.trim()) patch.mcp.bearerToken = token.value.trim();
  }
  return patch;
}
form.addEventListener('submit', async event => {
  event.preventDefault(); if (saving || !original) return;
  if (clearTokenPending && token.value.trim()) { message('请在“输入新 Token”和“清除已存 Token”中选择一种操作。', true); return; }
  saving = true; content.inert = true; save.disabled = true; cancelButton.disabled = true; clearMessage(); draftState.textContent = '正在保存…';
  try {
    const result = await commands.saveConfig(makePatch());
    fill(result.config); token.value = ''; message('设置已保存。');
    if (result.connectionChanged && result.config.mcp?.hasToken) {
      message('设置已保存，阅读器会自动连接新数据源。连接状态见下方。');
    } else if (result.connectionChanged) message('设置已保存，Token 已清除。');
    await refreshHealth();
  } catch (error) { message(normalizeError(error).message, true); draftState.textContent = '未保存，请检查提示'; }
  finally { saving = false; content.inert = false; save.disabled = Boolean(original?.configReadOnly); cancelButton.disabled = false; }
});
form.addEventListener('input', () => { dirty = true; draftState.textContent = '有未保存的修改'; clearMessage(); });
opacity.addEventListener('input', updateAppearance);
luminosity.addEventListener('input', updateAppearance);
for (const input of [server, token, clearToken]) input.addEventListener('input', () => {
  connectionGeneration += 1;
  connectionResult.textContent = clearTokenPending ? '保存后将清除 Token。点击撤销后可测试数据源。' : '';
  connectionResult.classList.remove('is-error');
  test.disabled = clearTokenPending; reconnect.disabled = !original?.mcp?.hasToken;
});
document.addEventListener('keydown', event => { if (event.key === 'Escape') { event.preventDefault(); void cancel(); } });
matchMedia('(prefers-color-scheme: dark)').addEventListener('change', updateAppearance);
void commands.listen('config-updated', config => { if (!dirty && !saving) fill(config); });
void commands.listen('connection-state', () => void refreshHealth());
void commands.listen('settings-opened', () => { if (!saving) { clearMessage(); connectionResult.textContent = ''; token.disabled = false; void load(); } });
void load();
