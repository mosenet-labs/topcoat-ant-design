## 1. 添加 Cargo 依赖

在应用的 `Cargo.toml` 中添加：

```toml
[dependencies]
topcoat-ant-design = { version = "0.1.1", features = ["router"] }
```

本地开发组件库时，可以使用路径依赖：

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

`module_router!()` 负责模块派生的 page、layout、layer 和 route。组件库的 Fontsource 字体由 `topcoat_ant_design()` 显式注册；应用的 procedure 和 shard 也应按需注册：

```rust,ignore
let router = topcoat::router::module_router!()
    .runtime()
    .discover_procedures()
    .discover_shards()
    .topcoat_ant_design()
    .assets(app_assets)
    .build();
```

需要一次性发现绝对路径处理器和字体的应用，也可以在 `module_router!()` 返回的 builder 上调用完整 `.discover()`。完整发现与 `topcoat_ant_design()` 不应同时注册同一字体路由。

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

图标常量从 `topcoat_ant_design::icons` 导入，并交给 Topcoat 原生 `icon` 组件渲染。完整示例见 [Icons 图标](icons.md)。

## 接入边界

| 层级 | 职责 |
| --- | --- |
| `topcoat-ant-design` | 构建 Tailwind CSS、声明 Asset、提供 JetBrains Mono Fontsource、Iconify 图标和组件 API。 |
| 宿主 Topcoat 应用 | 使用模块路由、加载 AssetBundle、配置静态资源托管、注册字体并调用 `head_assets()`。 |
| 业务页面 | 导入所需组件，通过 signal、绑定属性和事件传入状态与业务动作。 |
