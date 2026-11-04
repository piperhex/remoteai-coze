import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";

export default defineConfig({
  root: fileURLToPath(new URL(".", import.meta.url)),
  plugins: [react()],
  // Resolve this repository's public package entries even inside the parent submodule checkout.
  resolve: { alias: {
    '@remoteai/i18n/coze': fileURLToPath(new URL('../packages/i18n/src/catalogs/coze/index.ts', import.meta.url)),
    '@remoteai/i18n/errors': fileURLToPath(new URL('../packages/i18n/src/errors.ts', import.meta.url)),
    '@remoteai/i18n/copy': fileURLToPath(new URL('../packages/i18n/src/copy', import.meta.url)),
    '@remoteai/i18n': fileURLToPath(new URL('../packages/i18n/src/index.ts', import.meta.url)),
  } },
  build: { outDir: "../dist/ui", emptyOutDir: true, target: "es2022" },
  // Unit tests assert Chinese copy: pin the zh-CN locale the desktop tests use.
  test: { setupFiles: [fileURLToPath(new URL("../shared/localization/testLocale.ts", import.meta.url))] },
});
