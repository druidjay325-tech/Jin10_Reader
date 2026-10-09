import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { auditPublic, printIssues, sha256 } from './public-policy.mjs';

const root = fs.realpathSync(path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..'));
const destinationArgument = process.argv[2];
if (!destinationArgument || process.argv.length !== 3) {
  console.error('用法：node tools/export-public.mjs <不存在的新目录>');
  process.exit(2);
}
const destination = path.resolve(destinationArgument);
const relative = path.relative(root, destination);
if (!relative || (!relative.startsWith(`..${path.sep}`) && relative !== '..' && !path.isAbsolute(relative))) {
  console.error('导出目录必须位于源码目录之外，避免递归复制。');
  process.exit(2);
}
if (fs.existsSync(destination)) {
  console.error('目标目录已存在；为保留已有内容，本工具拒绝覆盖。');
  process.exit(2);
}
const parent = path.dirname(destination);
if (!fs.existsSync(parent) || !fs.statSync(parent).isDirectory() || fs.realpathSync(parent).toLowerCase() !== parent.toLowerCase()) {
  console.error('目标父目录必须已存在且不能经过符号链接。');
  process.exit(2);
}
const { files, issues } = auditPublic(root);
if (issues.length) {
  printIssues(issues);
  console.error('检查未通过，没有创建导出目录。');
  process.exit(1);
}

// 先缓存通过检查的内容，避免复制过程中读取其他进程新写入的文件。
const entries = files.map((file) => {
  const absolute = path.join(root, file);
  if (!fs.lstatSync(absolute).isFile()) throw new Error('导出源文件类型在检查后变化。');
  const content = fs.readFileSync(absolute);
  return { path: file, content, bytes: content.length, sha256: sha256(content) };
});
// 对实际复制的内容再跑文本检查，防止审核与读取之间的修改漏过。
const { inspectText } = await import('./public-policy.mjs');
const copiedIssues = entries.flatMap((entry) => /\.(?:png|ico|dll)$/.test(entry.path) ? [] : inspectText(entry.path, entry.content.toString('utf8')));
if (copiedIssues.length) {
  printIssues(copiedIssues);
  console.error('检查后源文件发生不安全变化，没有创建导出目录。');
  process.exit(1);
}
fs.mkdirSync(destination);
for (const entry of entries) {
  const target = path.join(destination, ...entry.path.split('/'));
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, entry.content, { flag: 'wx' });
}
const manifest = {
  format: 1,
  product: 'Jin10 Reader',
  version: '0.1.5',
  createdAt: new Date().toISOString(),
  scope: '白名单源码快照；不包含 Git 历史、运行数据或构建产物。',
  files: entries.map(({ path: file, bytes, sha256: hash }) => ({ path: file, bytes, sha256: hash })),
};
fs.writeFileSync(path.join(destination, 'EXPORT-MANIFEST.json'), `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
console.log(`已导出 ${entries.length} 个文件；源目录与旧 Git 历史均未修改。`);
console.log(`目标：${destination}`);
