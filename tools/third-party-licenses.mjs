import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const target = 'x86_64-pc-windows-msvc';
const result = spawnSync('cargo', ['metadata', '--offline', '--locked', '--filter-platform', target, '--format-version', '1'], {
  cwd: root, encoding: 'utf8', maxBuffer: 32 * 1024 * 1024, windowsHide: true,
});
if (result.status !== 0) {
  console.error('离线 Cargo metadata 未完成；没有下载依赖或覆盖现有许可证文档。请在本地核对缓存条件。');
  process.exit(1);
}
const metadata = JSON.parse(result.stdout);
if (!metadata.resolve) throw new Error('Cargo 未提供目标解析图，无法生成可靠清单。');
const nodes = new Set(metadata.resolve.nodes.map((node) => node.id));
const packages = metadata.packages.filter((pkg) => pkg.source && nodes.has(pkg.id))
  .sort((a, b) => a.name.localeCompare(b.name) || a.version.localeCompare(b.version));
const texts = new Map();
const missing = [];
const missingEvidence = [];
const licenseCounts = new Map();
const hash = (value) => createHash('sha256').update(value).digest('hex');
const supplementalPath = path.join(root, 'licenses', 'license-supplements.json');
const supplements = fs.existsSync(supplementalPath) ? JSON.parse(fs.readFileSync(supplementalPath, 'utf8')) : [];
const textName = /^(?:licen[sc]e(?:[._-].*)?|copying(?:[._-].*)?|notice(?:[._-].*)?|copyright(?:[._-].*)?|unlicen[sc]e(?:[._-].*)?)$/i;
const licenseDirectory = /^(?:licen[sc]es?|legal|third.party.licen[sc]es?|aws-lc)$/i;

function licenseFiles(directory, declared) {
  const output = new Set();
  const consider = (file) => {
    const relative = path.relative(directory, file);
    if (!relative || relative.startsWith('..') || path.isAbsolute(relative)) return;
    if (fs.existsSync(file) && fs.lstatSync(file).isFile()) output.add(file);
  };
  if (declared) consider(path.resolve(directory, declared));
  const visit = (current, depth, isLicenseDirectory = false) => {
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const file = path.join(current, entry.name);
      if (entry.isSymbolicLink()) continue;
      if (entry.isFile() && (textName.test(entry.name) || (isLicenseDirectory && /\.(?:txt|md|rst)$/i.test(entry.name)))) consider(file);
      if (entry.isDirectory() && depth < 3 && licenseDirectory.test(entry.name)) visit(file, depth + 1, entry.name.toLowerCase() !== 'aws-lc');
    }
  };
  visit(directory, 0);
  return [...output].sort();
}

const rows = [];
for (const pkg of packages) {
  const identity = `${pkg.name} ${pkg.version}`;
  const expression = pkg.license || '未声明；需人工复核';
  licenseCounts.set(expression, (licenseCounts.get(expression) || 0) + 1);
  const directory = path.dirname(pkg.manifest_path);
  const files = licenseFiles(directory, pkg.license_file);
  const captured = [];
  for (const file of files) {
    const bytes = fs.readFileSync(file);
    if (bytes.includes(0) || bytes.length > 1024 * 1024) continue;
    const text = bytes.toString('utf8');
    const digest = hash(bytes);
    const relative = path.relative(directory, file).split(path.sep).join('/');
    if (!texts.has(digest)) texts.set(digest, { text, uses: [] });
    texts.get(digest).uses.push(`${identity} · ${relative}`);
    captured.push(relative);
  }
  if (!captured.length) {
    let commit = null;
    try { commit = JSON.parse(fs.readFileSync(path.join(directory, '.cargo_vcs_info.json'), 'utf8')).git?.sha1; } catch { /* 缺少来源提交时不猜测可用补充许可。 */ }
    for (const supplement of supplements.filter((entry) => entry.packages.includes(`${pkg.name}@${pkg.version}`))) {
      if (!commit || commit !== supplement.commit || !/^licenses\/upstream\/[a-z0-9.-]+\.txt$/.test(supplement.file)) throw new Error('补充许可与包发布物的精确提交不匹配。');
      const absolute = path.join(root, supplement.file);
      if (!fs.lstatSync(absolute).isFile()) throw new Error('补充许可必须是普通文本文件。');
      const bytes = fs.readFileSync(absolute);
      const digest = hash(bytes);
      if (digest !== supplement.sha256) throw new Error('补充许可哈希不匹配。');
      if (!texts.has(digest)) texts.set(digest, { text: bytes.toString('utf8'), uses: [] });
      texts.get(digest).uses.push(`${identity} · ${supplement.kind === 'canonical-license' ? '依声明收录的许可制定者标准文本' : '精确提交原许可'} ${supplement.source}`);
      captured.push(`上游补充：${path.basename(supplement.file)}`);
    }
  }
  if (!captured.length) {
    missing.push(identity);
    let vcs = null;
    try { vcs = JSON.parse(fs.readFileSync(path.join(directory, '.cargo_vcs_info.json'), 'utf8')); } catch { /* 发布物可能没有 VCS 元数据。 */ }
    missingEvidence.push({ package: identity, repository: pkg.repository, license: pkg.license, commit: vcs?.git?.sha1, pathInRepository: vcs?.path_in_vcs });
  }
  rows.push(`| [${pkg.name}](https://crates.io/crates/${encodeURIComponent(pkg.name)}/${encodeURIComponent(pkg.version)}) | ${pkg.version} | ${expression.replaceAll('|', '\\|')} | ${captured.length ? captured.map((name) => `\`${name}\``).join('、') : '未找到独立文本；需人工复核'} |`);
}
const lockHash = hash(fs.readFileSync(path.join(root, 'Cargo.lock')));
const licenseTable = [...licenseCounts.entries()].sort(([a], [b]) => a.localeCompare(b)).map(([license, count]) => `| ${license.replaceAll('|', '\\|')} | ${count} |`).join('\n');
const markdown = `# Windows 依赖许可证清单

此清单由 \`node tools/third-party-licenses.mjs\` 离线生成。目标 \`${target}\`，使用当前默认功能的 Cargo 解析节点，包括进入该目标图的运行、构建和开发依赖；不把锁文件内所有其他平台包都算进 Windows 发行物。清单列的是依赖声明，不是对二进制实际链接代码的逐字节鉴定。

- Cargo.lock SHA-256：\`${lockHash}\`
- 目标图中的第三方包版本：**${packages.length} 个**。
- 取得的独立许可/NOTICE 文本：**${texts.size} 份不同内容**，相同文本按哈希去重，保留全部包与文件归属。
- 尚未覆盖许可文本的包版本：**${missing.length} 个**${missing.length ? '，见下方限制。' : '。'}

原始文本保存在 [third-party-notices.txt](third-party-notices.txt)。生成器只读取本机已缓存包的许可、COPYING、COPYRIGHT、NOTICE 等文本，以及已经核对的上游补充许可，没有导出第三方源码或本机绝对路径。源码包附 \`license-supplements.json\`，原文出处也写入汇编。补充来源绑定包发布物的 VCS 提交并校验 SHA-256；其中 selectors 发布物没有独立许可文件，依其 MPL-2.0 声明及源文件许可头，收录 [Mozilla 官方 MPL 2.0 标准文本](https://www.mozilla.org/media/MPL/2.0/index.txt)，不伪称来自该 crate 的独立 LICENSE。生成器本身仍然全离线。发行源码与二进制附件应同时携带此清单和许可汇编；项目 MIT 不替代第三方许可。

## 许可表达式汇总

以下保留 Cargo 元数据中的原始 SPDX 表达式，不擅自替贡献者选择多许可中的某一个分支。

| 声明 | 包版本数 |
|---|---:|
${licenseTable}

## 逐包清单

| 包 | 版本 | Cargo 声明 | 收录的包内许可文件 |
|---|---|---|---|
${rows.join('\n')}

## MPL 2.0 组件的源码获取

下列组件使用发布物原样依赖，本项目未修改其源码。接收二进制的用户可以从以下精确版本的 crates.io 发布物取得对应源码；许可全文见汇编，并遵循 [MPL 2.0 第 3 节](https://www.mozilla.org/en-US/MPL/2.0/#responsibilities)的适用要求。正式发布时应核对这些来源仍可获得；未来若修改依赖源码，需提供与修改后二进制对应的源码获取方式。

${packages.filter((pkg) => pkg.license?.includes('MPL-2.0')).map((pkg) => `- [${pkg.name} ${pkg.version} 原始源码发布物](https://crates.io/api/v1/crates/${encodeURIComponent(pkg.name)}/${encodeURIComponent(pkg.version)}/download)`).join('\n')}

## 限制与维护

${missing.length ? `尚未收录许可文本：${missing.map((name) => `\`${name}\``).join('、')}。这些包的声明已保留，但许可文本仍需从其相同版本发布物或权威仓库补齐，不能把该缺口写成已审核完成。` : '本次目标图中每个包版本均已关联至少一份包内原文、同提交上游原文或按明确声明取得的许可制定者标准文本。'}

自动收录范围是包声明的 license_file、包根目录许可/NOTICE，以及常见许可子目录；复杂 vendored 组件可能还有专属声明，需要根据最终分发内容人工复核。Rust 标准库、MSVC 工具链、Windows/WebView2、NSIS 和构建器自身的分发条款不在本 Cargo 项目依赖图内，仍遵循其发行者规则。本文件不证明完整法律审查已经完成。

改变锁文件、功能开关或目标平台后重跑生成器，并复核新增/变更的表达式和 NOTICE。默认全离线，缓存不足即停止，不自动下载其他平台依赖。
`;
const notices = ['Jin10 Reader · Windows 第三方许可与 NOTICE 汇编', `Cargo.lock SHA-256: ${lockHash}`, '以下保留包内原始许可文本，按完整文本哈希去重。'];
for (const [digest, entry] of [...texts.entries()].sort(([a], [b]) => a.localeCompare(b))) {
  notices.push(`\n${'='.repeat(78)}\nSHA-256: ${digest}\n适用包与原文件：\n${entry.uses.map((use) => `- ${use}`).join('\n')}\n${'-'.repeat(78)}\n${entry.text}`);
}
fs.writeFileSync(path.join(root, 'licenses', 'third-party-licenses.md'), markdown, 'utf8');
fs.writeFileSync(path.join(root, 'licenses', 'third-party-notices.txt'), `${notices.join('\n')}\n`, 'utf8');
console.log(JSON.stringify({ target, packages: packages.length, licenseExpressions: licenseCounts.size, distinctTexts: texts.size, missing, missingEvidence }, null, 2));
