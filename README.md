# Topcoat Ant Design

[English](README.md) · [简体中文](README.zh-CN.md)

`topcoat-ant-design` provides a reusable Rust library and a component Gallery binary behind the `gallery` feature. Components handle presentation, browser state, and interaction. They do not depend on a domain model, database, or external service.

Available components: [Icons](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/icons/index.html), [Notification](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.notification.html), [Popconfirm](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.popconfirm.html), [Dropdown Menu](docs/en/components/dropdown-menu.md), [Dialog](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.dialog.html), [Tag](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.tag.html), [Tooltip](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.tooltip.html), [Collapse](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.collapse.html), [Accordion](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.accordion_item.html), [Tabs](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.tabs.html), [Drawer](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.drawer.html), [Table](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.data_table.html), [FormField](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.form_field.html), and [DateTimeRange](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.date_time_range_filter.html).

The AI component set includes messages, sending, conversation navigation, Markdown, process details, sources, actions, attachments, and prompts. The Gallery's **AI Components** section shows individual examples and a composed Chat interface.

The current scope and next steps are recorded in [AI component requirements](docs/en/ai-components.md).
See the [Chat component guide](docs/en/components/chat.md) for the public API and host integration boundary.

See the [complete integration guide](docs/en/getting-started.md) or its [Chinese version](docs/getting-started.md).

This library targets Topcoat 0.9.0. Interactive hosts must enable `.runtime()` on the router and include `topcoat::runtime::script()` in the document head.

## Add the dependency

For a published version:

```toml
[dependencies]
topcoat-ant-design = "=0.2.0"
```

For local component development:

```toml
[dependencies]
topcoat-ant-design = { path = "../topcoat-ant-design" }
```

The default feature is sufficient to render components. Enable `router` only when the host uses selective route discovery and must explicitly register font routes:

```toml
topcoat-ant-design = { version = "=0.2.0", features = ["router"] }
```

## CSS, fonts, and assets

The library builds its own Tailwind component CSS and exposes it as a Topcoat `Asset`. The host does not need a build dependency on this crate or a path to its CSS. Call `head_assets()` in the root layout, then load and serve the Topcoat asset bundle from the host Router:

```rust,ignore
use topcoat_ant_design::head_assets;

Ok(view! {
    <head>
        head_assets()
    </head>
})
```

For selective route discovery, enable `router` and register the library's Fontsource routes:

```rust,ignore
use topcoat_ant_design::RouterBuilderUiExt;
use topcoat::{
    asset::{AssetBundle, RouterBuilderAssetExt},
    runtime::RouterBuilderRuntimeExt,
};

let app_assets = AssetBundle::load().expect("load Topcoat asset bundle");
let router = topcoat::router::module_router!()
    .runtime()
    .topcoat_ant_design()
    .assets(app_assets)
    .build();
```

The host creates `app_assets`; it is not a global variable supplied by the library. Full `.discover()` already discovers the font routes, so use one registration strategy. JetBrains Mono is the default font at weights 400, 500, 600, and 700; Chinese glyphs fall back to the system font. Fontsource currently uses its default jsDelivr source. A fresh build may download the pinned Tailwind CLI through Topcoat. Component CSS does not include Tailwind Preflight.

## Icons and components

The build stages the Ant Design Iconify set from the committed `icons/ant-design.json` cache. Import a validated constant and render it with Topcoat's native `icon` component:

```rust,ignore
use topcoat_ant_design::icons::PROJECT_OUTLINED;
use topcoat::icon::icon;

Ok(view! { icon(data: PROJECT_OUTLINED, label: "GitLab projects") })
```

The caller owns browser state through Topcoat signals. For example:

```rust,ignore
use topcoat_ant_design::{NotificationTone, notification};
use topcoat::runtime::signal;

let notice = signal(cx, String::new);

Ok(view! {
    <button @click=$(|_e| notice.set("Saved successfully.".to_owned()))>"Save"</button>
    notification(
        message: &notice,
        title: "Saved",
        tone: NotificationTone::Success,
    )
})
```

Popconfirm uses a trusted stable ID to associate its trigger and bubble; provide the business action in the child confirm button. The official [Dropdown Menu](docs/en/components/dropdown-menu.md) groups actions in a native `<details>` control. Collapse shares one signal between its trigger attributes and content. Official Tabs use real links and a host-provided `active` state. Drawer composes the official Sheet, accepts a `Signal<bool>`, and can navigate to a close URL through `DrawerConfig::with_close_href`. Each component has an [English guide](docs/en/components/) and a [Chinese guide](docs/components/).

Built-in component labels default to English. Pass `language: UiLanguage::ChineseSimplified` to composed components that provide their own controls, such as Notification, Popconfirm, Drawer, and DateTimeRange, when the host page is in Chinese.

## Topcoat native UI

All 31 Topcoat 0.9.0 registry components are re-exported directly from `topcoat_ant_design`; their modules also remain available under `topcoat_ant_design::ui`. For example, `use topcoat_ant_design::{button, ButtonVariant, dialog, dialog_content};`. The upstream registry state is recorded in [components.toml](components.toml).

This root-level API is available in version `0.2.0` and later.

The official implementations replace the former local Accordion, Dialog, Dropdown Menu, Tabs, and Tooltip components. Use their official root-level names and composition APIs. Distinct composites such as `data_table`, `drawer`, `popconfirm`, and `notification` remain in the library; Drawer is built on the official Sheet.

The regular `head_assets()` includes all component styles and the project font. Official and local components share the Ant Design color tokens in the same stylesheet. No wrapper class or extra Cargo feature is needed. Put `class="dark"` on an ancestor to use dark colors.

The Gallery includes an interactive [Topcoat native UI showcase](http://127.0.0.1:3100/topcoat-ui) with light and dark themes, sidebar, fields, dialogs, tables, and other controls.

The sources were copied from the official Topcoat `v0.9.0` registry at commit `96e8f9e0932ea883ced2859d462e9d6d3f52ea59`; see [upstream license](assets/topcoat-upstream-LICENSE). This project vendors the registry sources directly and does not require Topcoat's `ui` Cargo feature.

## Browse the Gallery

```bash
cargo run -p topcoat-ant-design \
  --bin topcoat-ant-design-gallery \
  --features gallery
```

Open `http://127.0.0.1:3100/` for the English Quick Start page, or `http://127.0.0.1:3100/overview` for the component overview. Use the **中文** switch for Chinese, or open `http://127.0.0.1:3100/?lang=zh` directly. `HOST` and `PORT` override the listening address. Component pages render interactive previews and examples. Most interactions use browser-side Topcoat signals; Chat also demonstrates validated Gallery-only procedures.

Open `http://127.0.0.1:3100/chat` for the Chat interface preview. `/chat/new` demonstrates multiple turns, request states, and a validated demo procedure; it does not contact a model.

Open `/bubble`, `/message-list`, and `/sender` for individual Chat component examples and expandable source code.

Open `http://127.0.0.1:3100/dropdown-menu` for the interactive Dropdown Menu example.

## License

The project is MIT licensed. See [LICENSE](LICENSE). The Ant Design icon data in `icons/ant-design.json` retains its MIT notice in [LICENSES/ant-design-icons-MIT.txt](LICENSES/ant-design-icons-MIT.txt).
