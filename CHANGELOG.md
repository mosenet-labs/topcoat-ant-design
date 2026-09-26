# Changelog

## 0.2.0

Official release built on Topcoat 0.9.0.

- Export all 31 official Topcoat UI components from the crate root and replace the local Accordion, Dialog, Dropdown Menu, Tabs, and Tooltip implementations.
- Rebuild the Gallery shell with official sidebar controls and shared light and dark theme tokens.
- Apply the Ant Design light and dark tokens to official components through the shared stylesheet without a wrapper class or separate feature.
- Align custom component attribute forwarding and reactive active state with native UI conventions.
- Theme custom components with light and dark design tokens.
- Keep date-range input observers in sync when shortcuts or Clear set values.
- Show official components alongside custom components in the bilingual Gallery, with interactive previews and code examples.
- Preserve the selected theme across Gallery navigation and update example Tabs without a page reload.

## 0.2.0-dev.1

Development release from the `test` branch.

- Upgrade the Topcoat dependency to 0.9.0 and use its reactive UI APIs.
- Include all 31 native Topcoat UI registry components and their neutral theme.
- Add reusable Chat components and interactive Gallery examples.
- Reuse native Topcoat primitives in existing components while retaining their public interfaces.
