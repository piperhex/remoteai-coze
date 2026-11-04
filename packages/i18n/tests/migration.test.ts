import { createHash } from 'node:crypto';
import { expect, it } from 'vitest';
import { resources } from '../src/catalogs/coze';
import kinds from '../src/copy/coze/kinds';
import { createI18n, applications } from '../src';

function sorted(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(sorted);
  if (value && typeof value === 'object') {
    return Object.fromEntries(Object.entries(value).sort(([a], [b]) => a.localeCompare(b, 'en'))
      .map(([key, item]) => [key, sorted(item)]));
  }
  return value;
}

it('preserves all pre-extraction Chinese, English and Russian translations', () => {
  const dictionaries = resources as Record<string, Record<string, string>>;
  const entries = Object.keys(dictionaries.zh).filter(key => key !== 'coze.application.title')
    .map(key => [dictionaries.zh[key], dictionaries.en[key], dictionaries.ru[key]])
    .sort((a, b) => a[0].localeCompare(b[0], 'en'));
  expect(createHash('sha256').update(JSON.stringify(entries)).digest('hex'))
    .toBe('54efc3f47447847456222d9c0470eca8ea5ed35b5afe24104da1e27471db06de');
});

it('preserves structured resource labels and every released language', () => {
  expect(createHash('sha256').update(JSON.stringify(sorted(kinds))).digest('hex'))
    .toBe('0be94343d155dcb7cbc40ba4a9a0fe120f220792eee7b170be01befbc03834b6');
  expect(applications.coze).toEqual(['zh', 'en', 'ru', 'ja']);
  for (const language of applications.coze) {
    expect(Object.keys(resources[language]).sort()).toEqual(Object.keys(resources.en).sort());
    expect(Object.keys(kinds[language]).sort()).toEqual(Object.keys(kinds.en).sort());
  }
  expect(createI18n({ locale: 'ja', resources }).t('coze.application.title')).toBe('アプリケーション');
});
