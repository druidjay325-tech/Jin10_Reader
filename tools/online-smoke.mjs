import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

if (process.argv.length !== 3 || process.argv[2] !== '--allow-network' || process.env.JIN10_RUN_ONLINE !== '1' || !process.env.JIN10_TEST_TOKEN?.trim()) {
  console.error('在线测试未启动。需显式 --allow-network、JIN10_RUN_ONLINE=1 和专用 JIN10_TEST_TOKEN；不会读取本机应用配置。');
  process.exit(2);
}
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
console.log('开始真实服务 smoke；会消耗上游额度。凭据仅通过进程环境传递。');
const result = spawnSync('cargo', ['test', '--locked', '--test', 'online_smoke', '--', '--ignored', '--nocapture'], {
  cwd: root,
  env: process.env,
  stdio: 'inherit',
  windowsHide: true,
  shell: false,
});
if (result.error) console.error('无法启动 Cargo 在线测试，请确认 Rust 工具链可用。');
process.exitCode = result.status ?? 1;
