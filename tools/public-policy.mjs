import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';

export const rootFiles = new Set([
  'README.md', 'README.en.md', 'LICENSE', 'CHANGELOG.md', 'CONTRIBUTING.md',
  'SECURITY.md', 'Cargo.toml', 'Cargo.lock', 'build.rs', 'tauri.conf.json',
  '.gitignore', 'package.json', 'package-lock.json', 'rust-toolchain.toml',
]);
const extensions = {
  src: new Set(['.rs']),
  dist: new Set(['.js', '.mjs', '.css', '.html', '.svg']),
  capabilities: new Set(['.json']),
  permissions: new Set(['.toml', '.json']),
  icons: new Set(['.svg', '.png', '.ico']),
  licenses: new Set(['.md', '.txt', '.json']),
  tests: new Set(['.rs', '.js', '.mjs', '.cjs', '.json']),
  tools: new Set(['.js', '.mjs', '.cjs', '.py', '.ps1']),
  native: new Set(['.cpp', '.h', '.md', '.json']),
  resources: new Set(),
  '.github': new Set(['.yml', '.yaml', '.md', '.json']),
};
const ignoredRoot = new Set(['target', 'node_modules', 'gen', 'artifacts', 'release', '.git']);
const publicLicenses = new Set(['licenses/ASSETS.md', 'licenses/third-party-licenses.md', 'licenses/third-party-notices.txt', 'licenses/license-supplements.json']);
const forbiddenName = /(?:^|\/)(?:\.env(?:\..*)?|settings(?:\.[^/]*)?\.json|credentials[^/]*|[^/]*\.(?:pem|key|p12|pfx|log|db|sqlite3?|bak)|collector\.rs)(?:$|\/)/i;
const forbiddenSegment = /(?:^|\/)(?:news-rag-demo|tauri-spike|backups|raw|data|cache|logs|\.git)(?:\/|$)/i;

export function sha256(buffer) {
  return createHash('sha256').update(buffer).digest('hex');
}

export function isPublicPath(relative) {
  if (relative.includes('\\') || relative.startsWith('/') || relative.split('/').includes('..')) return false;
  if ((forbiddenName.test(relative) && relative !== 'capabilities/settings.json') || forbiddenSegment.test(relative)) return false;
  if (rootFiles.has(relative)) return true;
  if (relative === 'dist/app-icon.png') return true;
  if (/^resources\/acrylic\/(?:official-acrylic\.dll|Microsoft\.WindowsAppRuntime\.Bootstrap\.dll|SDK-LICENSE\.txt|SDK-NOTICE\.txt|MANIFEST\.json)$/.test(relative)) return true;
  const [top, ...remaining] = relative.split('/');
  if (top === 'licenses') return publicLicenses.has(relative) || /^licenses\/upstream\/[a-z0-9.-]+\.txt$/.test(relative);
  return remaining.length > 0 && extensions[top]?.has(path.posix.extname(relative).toLowerCase()) === true;
}

export function collectPublicFiles(root) {
  const files = [];
  const issues = [];
  function walk(directory, prefix = '') {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
      if (!prefix && ignoredRoot.has(entry.name)) continue;
      if (relative === 'EXPORT-MANIFEST.json') continue;
      const absolute = path.join(directory, entry.name);
      if (entry.isSymbolicLink()) {
        issues.push({ file: relative, rule: 'path.symlink' });
      } else if (entry.isDirectory()) {
        if ((!prefix && !extensions[entry.name]) || forbiddenSegment.test(relative)) {
          issues.push({ file: relative, rule: 'path.unlisted-directory' });
        } else {
          walk(absolute, relative);
        }
      } else if (entry.isFile() && isPublicPath(relative)) {
        files.push(relative);
      } else {
        issues.push({ file: relative, rule: 'path.unlisted-file' });
      }
    }
  }
  walk(root);
  return { files: files.sort(), issues };
}

export function inspectText(file, text) {
  const issues = [];
  const rules = [
    ['secret.private-key', /-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----/g],
    ['secret.provider-token', /\b(?:gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{30,}|sk-[A-Za-z0-9_-]{24,})\b/g],
    ['secret.bearer-literal', /\bBearer\s+[A-Za-z0-9_.-]{24,}/g],
    ['secret.jwt', /\beyJ[A-Za-z0-9_-]{12,}\.[A-Za-z0-9_-]{12,}\.[A-Za-z0-9_-]{12,}\b/g],
    ['secret.config-literal', /["'](?:bearerToken|bearer_token|api_key|apiKey|access_token)["']\s*:\s*["'][A-Za-z0-9_.-]{16,}["']/g],
    ['privacy.personal-path', /\b[A-Za-z]:[\\/]Users[\\/][^\s"'<>/\\]+[\\/]/g],
  ];
  if (file.startsWith('src/') || file.startsWith('dist/') || file === 'Cargo.toml') {
    rules.push(['scope.collector', /\b(?:ingest_url|ingest_cmd|start_collector|mod\s+collector|news-rag-demo|collector::|rusqlite|lancedb)\b|127\.0\.0\.1:4357/g]);
  }
  if (file.startsWith('dist/')) {
    rules.push(['security.dynamic-code', /\beval\s*\(|\bnew\s+Function\s*\(|\.innerHTML\s*=/g]);
    if (file.endsWith('.html')) {
      rules.push(['security.inline-event', /\son[a-z]+\s*=/gi]);
      rules.push(['security.inline-script', /<script(?![^>]*\bsrc\s*=)[^>]*>[\s\S]*?\S[\s\S]*?<\/script\s*>/gi]);
    }
  }
  for (const [rule, regex] of rules) {
    for (const match of text.matchAll(regex)) {
      issues.push({ file, rule, line: text.slice(0, match.index).split('\n').length });
    }
  }
  return issues;
}

export function inspectConfig(config) {
  const issues = [];
  const add = (rule) => issues.push({ file: 'tauri.conf.json', rule });
  if (config.identifier !== 'io.github.jin10reader.desktop') add('identity.identifier');
  if (config.version !== '0.1.5') add('identity.version');
  if (!config.bundle?.active) add('bundle.disabled');
  const csp = config.app?.security?.csp;
  if (typeof csp !== 'string' || !csp.trim()) {
    add('csp.missing');
    return issues;
  }
  if (/unsafe-eval|\*/.test(csp)) add('csp.unsafe-source');
  const directives = new Map(csp.split(';').map((entry) => {
    const [name, ...values] = entry.trim().split(/\s+/);
    return [name, values];
  }));
  if (!directives.get('default-src')?.includes("'self'")) add('csp.default-src');
  const script = directives.get('script-src');
  if (!script?.includes("'self'") || script.some((value) => value !== "'self'" && !/^'sha(?:256|384|512)-/.test(value))) add('csp.script-src');
  if (directives.get('object-src')?.join(' ') !== "'none'") add('csp.object-src');
  if (directives.get('base-uri')?.join(' ') !== "'none'") add('csp.base-uri');
  return issues;
}

export function auditPublic(root) {
  const { files, issues } = collectPublicFiles(root);
  const nativeManifest=path.join(root,'resources/acrylic/MANIFEST.json');
  if(fs.existsSync(nativeManifest)){
    try{for(const entry of JSON.parse(fs.readFileSync(nativeManifest,'utf8')).files){const file='resources/acrylic/'+entry.file;if(!isPublicPath(file)||!fs.existsSync(path.join(root,file))||sha256(fs.readFileSync(path.join(root,file)))!==entry.sha256)issues.push({file,rule:'native.resource-integrity'});}}
    catch{issues.push({file:'resources/acrylic/MANIFEST.json',rule:'native.manifest-invalid'});}
  }
  for (const required of ['README.md', 'README.en.md', 'LICENSE', 'Cargo.toml', 'Cargo.lock', 'build.rs', 'tauri.conf.json', 'SECURITY.md', 'licenses/ASSETS.md']) {
    if (!files.includes(required)) issues.push({ file: required, rule: 'path.required-missing' });
  }
  for (const file of files) {
    if (/\.(?:png|ico|dll)$/.test(file)) continue;
    const text = fs.readFileSync(path.join(root, file), 'utf8');
    issues.push(...inspectText(file, text));
  }
  const configFile = path.join(root, 'tauri.conf.json');
  if (fs.existsSync(configFile)) {
    try {
      const config = JSON.parse(fs.readFileSync(configFile, 'utf8'));
      issues.push(...inspectConfig(config));
      const cargo = fs.readFileSync(path.join(root, 'Cargo.toml'), 'utf8');
      const version = cargo.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
      if (version !== config.version) issues.push({ file: 'Cargo.toml', rule: 'identity.version-mismatch' });
      if (!/^name\s*=\s*"jin10-reader"/m.test(cargo)) issues.push({ file: 'Cargo.toml', rule: 'identity.crate-name' });
      if (!/^license\s*=\s*"MIT"/m.test(cargo)) issues.push({ file: 'Cargo.toml', rule: 'license.manifest' });
    } catch {
      issues.push({ file: 'tauri.conf.json', rule: 'config.invalid' });
    }
  }
  return { files, issues };
}

export function printIssues(issues) {
  // 只暴露位置和规则编号；绝不把可能是秘密的原文打印出来。
  for (const issue of issues) console.error(`${issue.file}${issue.line ? `:${issue.line}` : ''} [${issue.rule}]`);
}
