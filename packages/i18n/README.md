# Coze localization

This repository's `@remoteai/i18n` workspace owns all Coze translations and language metadata.
It was extracted with its engine from Remote AI; it has no dependency on the parent checkout.
Coze resources are maintained only here. Desktop integration copy stays in Remote AI.

Use public package entries. Keep the four released languages complete. Platform copy belongs in
`platform/` and is generated through `platform.json`. Run `npm run generate:i18n` and
`npm run check:i18n` at this repository root. Never edit generated resources directly.
