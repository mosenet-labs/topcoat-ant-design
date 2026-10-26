## Rust 工具链

Topcoat 0.11.0 和本库 0.4.0 最低需要 Rust 1.99；本仓库通过根目录的 `rust-toolchain.toml` 固定使用 1.99.1：

```toml
[toolchain]
channel = "1.99.1"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

在使用本库的应用项目中也应配置兼容的工具链；Cargo 不会继承依赖库的 toolchain 文件。使用 rustup 时，可先安装上述版本，再进入项目目录确认实际生效的编译器：

```sh
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
rustup show active-toolchain
rustc --version
```

`Cargo.toml` 的 `rust-version` 只声明最低版本，不会自动切换编译器。如果构建仍使用 1.98.1，请检查显式的 `cargo +1.98.1`、`RUSTUP_TOOLCHAIN` 环境变量或 rustup 目录 override；CI 的 toolchain 配置和 Docker 的 Rust 基础镜像也需要使用兼容版本。

## 1. 添加 Cargo 依赖

在应用的 `Cargo.toml` 中添加：

```toml
[dependencies]
topcoat-ant-design = { version = "=0.4.0", features = ["router"] }
```

0.4.0 使用 Topcoat 0.11.0，需要 Rust 1.99+。从 0.3.x 升级时，请同步迁移官方 Dropdown Menu 的 `target`/`id`/`close` API。本地开发组件库时，也可以使用路径依赖：

```toml
[dependencies]
topcoat-ant-design = { path = "../topcoat-ant-design", features = ["router"] }
```

只渲染组件而不负责构建 Router 的 crate 可以省略 `router` feature。

## 2. 建立 Topcoat 模块路由

推荐用 `app.rs` 作为路由树根。Rust 的 `mod` 声明决定哪些路由模块参与编译，`module_router!()` 根据模块路径生成 URL：

```text
src/
├── main.rs          # mod app;
├── app.rs           # mod dashboard; module_router!() 的根
└── app/
    └── dashboard.rs # #[page] 对应 /dashboard
```

页面使用无路径的 `#[page]`，共享文档外壳使用 `#[layout]`。根布局会自动包裹 `app` 下的页面：

```rust,ignore
// src/app.rs
mod dashboard;

use topcoat_ant_design::head_assets;
use topcoat::{
    Result,
    router::{Slot, layout},
    view::{View, view},
};

#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="zh-CN">
            <head>
                <meta charset="utf-8">
                head_assets()
                topcoat::runtime::script()
            </head>
            <body>(slot)</body>
        </html>
    })
}
```

```rust,ignore
// src/app/dashboard.rs
use topcoat::{Result, router::page, view::{View, view}};

#[page]
async fn dashboard() -> Result<impl View> {
    Ok(view! { <h1>"Dashboard"</h1> })
}
```

## 3. 配置 Topcoat Router 与静态资源

`head_assets()` 声明页面需要的组件样式与字体。宿主还需要加载 Topcoat 构建生成的资源包，并把它注册到 Router。`app_assets` 是宿主创建的变量：

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

如果应用把资源编入单一二进制，可以像 Gallery 一样在自己的 `assets` 模块中构造 `AssetConfig`，再使用 `let app_assets = assets::config()?;`。资源配置是样式、字体和 Runtime 正常访问的必要条件。

## 4. 注册额外的链接期资源

Topcoat 0.9 将 procedure 和 shard 作为 route 注册。`module_router!()` 发现模块派生的 page、layout、layer 和 route；procedure 与 shard 需要用 `.route(name)` 显式注册。组件库的 Fontsource 字体由 `topcoat_ant_design()` 注册：

```rust,ignore
let router = topcoat::router::module_router!()
    .route(gallery_reply)
    .route(message_region)
    .runtime()
    .topcoat_ant_design()
    .assets(app_assets)
    .build();
```

只有使用 `Router::builder()` 从空路由开始时，才适合用完整 `.discover()` 统一发现全部路由。`module_router!()` 已发现模块派生的 route，再调用 `.discover_routes()` 可能重复注册。完整发现与 `topcoat_ant_design()` 也不应同时注册同一字体路由。

## 5. 使用组件

组件从 `topcoat_ant_design` 导入，浏览器状态由调用方的 Topcoat signal 管理：

```rust,ignore
use topcoat_ant_design::{NotificationTone, notification};
use topcoat::runtime::signal;

let notice = signal(cx, String::new);

<button @click=$(|_e| notice.set("保存成功。".to_owned()))>
    "保存"
</button>

notification(
    message: &notice,
    title: "操作成功",
    tone: NotificationTone::Success,
)
```

图标常量从 `topcoat_ant_design::icons` 导入，并交给 Topcoat 原生 `icon` 组件渲染。完整示例见 [Icons 图标](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/icons/index.html)。

## 接入边界

| 层级 | 职责 |
| --- | --- |
| `topcoat-ant-design` | 构建 Tailwind CSS、声明 Asset、提供 JetBrains Mono Fontsource、Iconify 图标和组件 API。 |
| 宿主 Topcoat 应用 | 使用模块路由、加载 AssetBundle、配置静态资源托管、注册字体并调用 `head_assets()`。 |
| 业务页面 | 导入所需组件，通过 signal、绑定属性和事件传入状态与业务动作。 |

## Topcoat 0.11 本地化

组件内置标签使用 Fluent 翻译。现有 `language` 属性仍默认英文；要跟随宿主请求语言，请传入 `UiLanguage::current(cx)`。在路由 `#[layer]` 中使用 `cx.with(topcoat::l10n::locale!("zh"))` 设置语言。不要只在 layout 内派生上下文，因为它不会影响页面。

```rust,ignore
use topcoat_ant_design::{UiLanguage, json_viewer};

view! {
    json_viewer(value: &data, language: UiLanguage::current(cx))
}
```

本库最低需要 Rust 1.99。新增官方组件示例：

- [Popover](components/popover.md)
- [Navigation Menu](components/navigation-menu.md)
- [Floating](components/floating.md)
