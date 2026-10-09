import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { collectPublicFiles, inspectConfig, inspectText, isPublicPath } from './public-policy.mjs';

test('白名单不会因扩展名允许而泄漏运行配置或旧历史', () => {
  for (const candidate of ['.git/config', 'src/collector.rs', 'docs/settings.json', 'tests/.env', 'docs/data/news.json', 'docs/open-source-readiness-private.md', '../README.md', 'docs/private.key', 'icons/setup.exe', 'dist/private.png','resources/arbitrary.dll','resources/acrylic/private.dll']) {
    assert.equal(isPublicPath(candidate), false, candidate);
  }
  for (const candidate of ['dist/widget.js', 'dist/app-icon.png', 'Cargo.lock', 'icons/mark.svg', 'licenses/ASSETS.md', '.github/workflows/windows.yml','native/acrylic.cpp','resources/acrylic/official-acrylic.dll']) {
    assert.equal(isPublicPath(candidate), true, candidate);
  }
});

test('危险内容只报告规则和位置，不回显匹配值', () => {
  const fake = 'sk-' + 'x'.repeat(32);
  const result = inspectText('src/example.rs', `// fixture\nconst VALUE = "${fake}";`);
  assert.equal(result[0].rule, 'secret.provider-token');
  assert.equal(result[0].line, 2);
  assert.equal(JSON.stringify(result).includes(fake), false);
  assert.equal(inspectText('dist/index.html', '<button onclick="run()">').some((issue) => issue.rule === 'security.inline-event'), true);
  assert.equal(inspectText('src/example.rs', 'let ingest_url = value;').some((issue) => issue.rule === 'scope.collector'), true);
});

test('CSP 空值、动态脚本与远端脚本会阻止公开', () => {
  const config = {
    identifier: 'io.github.jin10reader.desktop', version: '0.1.5', bundle: { active: true },
    app: { security: { csp: "default-src 'self'; script-src 'self'; object-src 'none'; base-uri 'none'" } },
  };
  assert.deepEqual(inspectConfig(config), []);
  config.app.security.csp += "; style-src 'self' 'unsafe-inline'";
  assert.deepEqual(inspectConfig(config), []);
  config.app.security.csp = null;
  assert.equal(inspectConfig(config).some((issue) => issue.rule === 'csp.missing'), true);
  config.app.security.csp = "default-src 'self'; script-src 'self' https:; object-src 'none'; base-uri 'none'";
  assert.equal(inspectConfig(config).some((issue) => issue.rule === 'csp.script-src'), true);
});

test('导出命令生成可核对清单，拒绝覆盖，失败时不创建目标', () => {
  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), 'jin10-export-test-'));
  try {
    const source = path.join(temporary, 'source');
    fs.mkdirSync(path.join(source, 'licenses'), { recursive: true });
    fs.mkdirSync(path.join(source, 'tools'));
    const toolsDirectory = path.dirname(fileURLToPath(import.meta.url));
    for (const script of ['public-policy.mjs', 'export-public.mjs']) {
      fs.copyFileSync(path.join(toolsDirectory, script), path.join(source, 'tools', script));
    }
    for (const file of ['README.md', 'README.en.md', 'LICENSE', 'SECURITY.md', 'Cargo.lock', 'build.rs', 'licenses/ASSETS.md']) {
      fs.writeFileSync(path.join(source, file), '合成测试内容\n');
    }
    fs.writeFileSync(path.join(source, 'Cargo.toml'), '[package]\nname = "jin10-reader"\nversion = "0.1.5"\nlicense = "MIT"\n');
    fs.writeFileSync(path.join(source, 'tauri.conf.json'), JSON.stringify({
      identifier: 'io.github.jin10reader.desktop', version: '0.1.5', bundle: { active: true },
      app: { security: { csp: "default-src 'self'; script-src 'self'; object-src 'none'; base-uri 'none'" } },
    }));
    fs.mkdirSync(path.join(source, '.git'));
    fs.writeFileSync(path.join(source, '.git', 'config'), '不能导出旧历史');
    const destination = path.join(temporary, 'export');
    const run = (target) => spawnSync(process.execPath, [path.join(source, 'tools', 'export-public.mjs'), target], { encoding: 'utf8', windowsHide: true });
    const first = run(destination);
    assert.equal(first.status, 0, first.stderr);
    assert.equal(fs.existsSync(path.join(destination, '.git')), false);
    const manifest = JSON.parse(fs.readFileSync(path.join(destination, 'EXPORT-MANIFEST.json'), 'utf8'));
    for (const entry of manifest.files) {
      const data = fs.readFileSync(path.join(destination, entry.path));
      assert.equal(entry.bytes, data.length);
      assert.equal(entry.sha256, createHash('sha256').update(data).digest('hex'));
    }
    assert.equal(run(destination).status, 2);
    fs.mkdirSync(path.join(source, 'docs'));
    fs.writeFileSync(path.join(source, 'docs', 'settings.json'), '{}');
    const blocked = path.join(temporary, 'blocked');
    assert.equal(run(blocked).status, 1);
    assert.equal(fs.existsSync(blocked), false);
  } finally {
    assert.equal(path.dirname(temporary), fs.realpathSync(os.tmpdir()));
    fs.rmSync(temporary, { recursive: true });
  }
});

test('收集器拒绝不在白名单内的文件，并且不进入 Git 与构建缓存', () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'jin10-public-policy-'));
  try {
    fs.mkdirSync(path.join(root, '.git'));
    fs.mkdirSync(path.join(root, 'target'));
    fs.mkdirSync(path.join(root, 'docs'));
    fs.writeFileSync(path.join(root, '.git', 'config'), 'private');
    fs.writeFileSync(path.join(root, 'target', 'private.txt'), 'private');
    fs.writeFileSync(path.join(root, 'docs', 'settings.json'), '{}');
    fs.writeFileSync(path.join(root, 'README.md'), '# test');
    const result = collectPublicFiles(root);
    assert.deepEqual(result.files, ['README.md']);
    assert.equal(result.issues.length, 1);
    assert.equal(result.issues[0].file, 'docs');
    assert.equal(result.issues[0].rule, 'path.unlisted-directory');
  } finally {
    // 只清理由本测试创建且已确认位于系统临时目录的专属 fixture。
    assert.equal(path.dirname(root), fs.realpathSync(os.tmpdir()));
    fs.rmSync(root, { recursive: true });
  }
});
