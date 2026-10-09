import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { auditPublic, printIssues } from './public-policy.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const { files, issues } = auditPublic(root);
if (issues.length) {
  printIssues(issues);
  console.error(`公开目录检查失败：${issues.length} 项；命中内容已隐去。`);
  process.exitCode = 1;
} else {
  console.log(`公开目录检查通过：${files.length} 个白名单文件；未扫描 Git 历史或依赖漏洞数据库。`);
}
