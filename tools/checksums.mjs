import fs from 'node:fs';
import path from 'node:path';
import { sha256 } from './public-policy.mjs';

const argument = process.argv[2];
if (!argument || process.argv.length !== 3) {
  console.error('用法：node tools/checksums.mjs <安装包目录>');
  process.exit(2);
}
const directory = path.resolve(argument);
const files = fs.readdirSync(directory, { withFileTypes: true })
  .filter((entry) => entry.isFile() && /\.(?:exe|msi|zip)$/i.test(entry.name))
  .map((entry) => entry.name).sort();
if (!files.length) {
  console.error('目录中没有安装包；不会写入空校验文件。');
  process.exit(1);
}
const lines = files.map((file) => `${sha256(fs.readFileSync(path.join(directory, file)))}  ${file}`);
fs.writeFileSync(path.join(directory, 'SHA256SUMS.txt'), `${lines.join('\n')}\n`, 'utf8');
console.log(`已为 ${files.length} 个发行物生成 SHA256SUMS.txt。校验值不代替代码签名。`);
