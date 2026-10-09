import test from 'node:test';
import assert from 'node:assert/strict';
import { inspectBinary } from './check-binary.mjs';

test('发行物扫描识别两种路径分隔符、UTF-16 与匿名路径，不回显路径值', () => {
  const profile = ['C:', 'Users', 'synthetic'].join('\\');
  for (const separator of ['/', '\\']) {
    const value = profile.replaceAll('\\', separator) + separator + 'source.c';
    for (const encoding of ['utf8', 'utf16le']) assert.deepEqual(inspectBinary(Buffer.from(value, encoding), profile), ['build.personal-path']);
  }
  assert.deepEqual(inspectBinary(Buffer.from('/build-user/source.c /jin10-reader/src/main.rs'), profile), []);
});

test('发行物拒绝凭据形态、私用采集及debug测试入口，结果仅包含规则名', () => {
  const fixture = ['sk-' + 'x'.repeat(32), 'get_collect_stats', 'JIN10_READER_TEST_DIR'].join(' ');
  assert.deepEqual(inspectBinary(Buffer.from(fixture), ''), ['secret.api-key-shape', 'runtime.private-collection', 'runtime.debug-profile']);
});
