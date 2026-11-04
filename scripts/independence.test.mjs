import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, relative, resolve } from 'node:path';
import test from 'node:test';

const root = fileURLToPath(new URL('../', import.meta.url));
test('build and release inputs belong to this repository', () => {
  const files = execFileSync('git', ['ls-files', '--cached', '--others', '--exclude-standard'],
    { cwd: root, encoding: 'utf8' }).trim().split('\n');
  for (const file of files.filter(file => /\.(?:mjs|ts|tsx|json|yml|toml)$/.test(file)
    && !file.endsWith('package-lock.json') && !file.startsWith('docs/'))) {
    const text = readFileSync(new URL('../' + file, import.meta.url), 'utf8');
    for (const match of text.matchAll(/(?:from\s+|import\s*\()['"](\.[^'"]+)['"]/g)) {
      const target = resolve(root, dirname(file), match[1]);
      assert.ok(!relative(root, target).startsWith('..'), `${file} imports outside this repository: ${match[1]}`);
    }
  }
});
test('CI runs tests on changes and publishes only explicit releases', () => {
  const workflow = readFileSync(new URL('../.github/workflows/ci.yml', import.meta.url), 'utf8');
  assert.match(workflow, /branches: \[main\]/);
  assert.match(workflow, /npm run check/);
  assert.match(workflow, /COZE_SIGNING_PRIVATE_KEY/);
  assert.doesNotMatch(workflow, /workflow_call:|apps\/coze\//);
});
