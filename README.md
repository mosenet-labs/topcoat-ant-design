# Topcoat Ant Design

[English](README.md) · [简体中文](README.zh-CN.md)

`topcoat-ant-design` provides a reusable Rust library and a component Gallery binary behind the `gallery` feature. Components handle presentation, browser state, and interaction. They do not depend on a domain model, database, or external service.

Available components: [Icons](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/icons/index.html), [Notification](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.notification.html), [Popconfirm](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.popconfirm.html), [Dialog](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.dialog.html), [Tag](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.tag.html), [Tooltip](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.tooltip.html), [Collapse](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.collapse.html), [Accordion](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.accordion_item.html), [Tabs](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.tabs.html), [Drawer](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.drawer.html), [Table](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.data_table.html), [FormField](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.form_field.html), and [DateTimeRange](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.date_time_range_filter.html).

The AI component set starts with `chat_bubble`, `chat_message_list`, and `chat_sender`. The Gallery's **AI Components** section shows them together in a Chat interface preview.

See the [complete integration guide](docs/en/getting-started.md) or its [Chinese version](docs/getting-started.md).

## Add the dependency

For a published version:

```toml
[dependencies]
topcoat-ant-design = "0.1.2"
```

For local component development:

```toml
[dependencies]
topcoat-ant-design = { path = "../topcoat-ant-design" }
```

The default feature is sufficient to render components. Enable `router` only when the host uses selective route discovery and must explicitly register font routes:

```toml
topcoat-ant-design = { version = "0.1.2", features = ["router"] }
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

Popconfirm uses a trusted stable ID to associate its trigger and bubble; provide the business action in the child confirm button. Collapse shares one signal between its trigger attributes and content. Tabs use real links and a host-provided `active` state. Drawer accepts a `Signal<bool>` and can navigate to a close URL through `DrawerConfig::with_close_href`. Each component has an [English guide](docs/en/components/) and a [Chinese guide](docs/components/).

Built-in component labels default to English. Pass `language: UiLanguage::ChineseSimplified` to components that provide their own controls, such as Notification, Popconfirm, Dialog, Drawer, Accordion, and DateTimeRange, when the host page is in Chinese.

## Browse the Gallery

```bash
cargo run -p topcoat-ant-design \
  --bin topcoat-ant-design-gallery \
  --features gallery
```

Open `http://127.0.0.1:3100/` for the English Quick Start page, or `http://127.0.0.1:3100/overview` for the component overview. Use the **中文** switch for Chinese, or open `http://127.0.0.1:3100/?lang=zh` directly. `HOST` and `PORT` override the listening address. Existing component pages render real interactions alongside the same Markdown used by their public API docs. Gallery interactions change browser-side Topcoat signals only and do not call a business service.

Open `http://127.0.0.1:3100/chat` for the Chat interface preview. Its input updates a local message; it does not contact a model.

## License

The project is MIT licensed. See [LICENSE](LICENSE). The Ant Design icon data in `icons/ant-design.json` retains its MIT notice in [LICENSES/ant-design-icons-MIT.txt](LICENSES/ant-design-icons-MIT.txt).
