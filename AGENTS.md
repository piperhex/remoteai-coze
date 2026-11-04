# Coze agent instructions

All instructions in this file must remain in English.

- This is the independent `piperhex/remoteai-coze` repository, used as Remote AI's `apps/coze` submodule.
- Do not import files, dependencies, credentials, or build tools from the parent repository.
- Preserve the desktop bootstrap protocol, MCP contracts, language bridge and package layout.
- Keep all translation sources and language metadata in `packages/i18n`; use public package entries.
  Run `npm run generate:i18n` and `npm run check:i18n` after resource changes.
- Run `npm run check`, `npm run test:e2e`, `cargo fmt --check`, `cargo test --locked`, and
  `cargo clippy --locked --all-targets -- -D warnings` for applicable changes.
- Browser tests must use bundled Chromium and `scripts/test-browser.mjs`. Never use system Chrome or Edge
  on Windows. Follow `docs/browser-tests.md` and preserve the password-probe workaround.
- Preserve existing visual identity. Inspect screenshots from the running application for UI changes.
- Keep source files below 500 lines where practical; split by responsibility. Use typed errors,
  thin entry points, and tests for important behavior. Never silently ignore errors or expose secrets.
- Published packages are immutable. Keep Cargo.toml, Cargo.lock, package.json and runtime.json versions aligned.
- After successful verification, commit only task changes and push the configured upstream. Merge remote
  updates without rewriting history; abort a conflicting merge and report the issue.
- Commit and push this repository before updating the parent submodule pointer. Do not stage unrelated
  parent work. Review active conversations before final delivery when conversation tools are available.
