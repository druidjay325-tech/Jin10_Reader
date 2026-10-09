import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

export function inspectBinary(bytes, profile = process.env.USERPROFILE) {
  const issues = [];
  const ascii = bytes.toString('latin1');
  if (profile) {
    const variants = [profile.replaceAll('\\', '/'), profile.replaceAll('/', '\\')].map(value => value.toLowerCase());
    const representations = [ascii.toLowerCase(), bytes.toString('utf16le').toLowerCase(), bytes.subarray(1).toString('utf16le').toLowerCase()];
    if (representations.some(text => variants.some(value => text.includes(value)))) issues.push('build.personal-path');
  }
  if (/sk-[A-Za-z0-9_-]{20,}/.test(ascii)) issues.push('secret.api-key-shape');
  if (['get_collect_stats', 'news-rag-demo', '127.0.0.1:4357'].some(value => ascii.includes(value))) issues.push('runtime.private-collection');
  if (ascii.includes('JIN10_READER_TEST_DIR') || ascii.includes('JIN10_READER_DISABLE_ACRYLIC')) issues.push('runtime.debug-profile');
  return issues;
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  if (process.argv.length !== 3) { console.error('Usage: node tools/check-binary.mjs <release.exe>'); process.exit(2); }
  const bytes = fs.readFileSync(process.argv[2]);
  const issues = inspectBinary(bytes);
  console.log(JSON.stringify({ file: path.basename(process.argv[2]), bytes: bytes.length, issues, scope: 'Selected byte patterns; compressed payloads also require source and embedded executable review.' }));
  process.exitCode = issues.length ? 1 : 0;
}
