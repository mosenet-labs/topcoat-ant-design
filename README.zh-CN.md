# Topcoat Ant Design

[English](README.md) · [简体中文](README.zh-CN.md)

`topcoat-ant-design` 同时提供可供第三方引用的 Rust lib，以及受 `gallery` feature 控制的组件展示网站 bin。组件只处理通用展示、浏览器状态和交互，不依赖领域模型、数据库或外部服务。

当前组件：

- [Icons](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/icons/index.html)：由 Topcoat Iconify 编译期生成的 Ant Design 图标目录；
- [Notification](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.notification.html)：页面右上角的操作结果通知；
- [Popconfirm](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.popconfirm.html)：按钮附近的轻量二次确认气泡；
- [Dropdown Menu](docs/components/dropdown-menu.md)：按钮附近的紧凑操作菜单；
- [Dialog](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.dialog.html)：承载表单与集中操作的官方对话框结构；
- [Tag](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.tag.html)：展示状态和分类；
- [Tooltip](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.tooltip.html)：提供悬停和键盘聚焦时的文字提示；
- [Collapse](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.collapse.html)：未知高度内容的展开与收起动效；
- [Accordion](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.accordion_item.html)：在多个面板间切换内容；
- [Tabs](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.tabs.html)：使用真实链接的路由型页签；
- [Drawer](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.drawer.html)：由 signal 或关闭路由控制的右侧详情面板；
- [Table](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.data_table.html)：支持显示密度和分页组合的数据表格；
- [FormField](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.form_field.html)：统一表单字段的标签、说明和错误展示；
- [DateTimeRange](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/struct.date_time_range_filter.html)：选择起止日期时间。

AI 组件覆盖消息、输入、会话导航、Markdown、过程详情、来源、操作、附件和建议输入。Gallery 的「AI 组件」分类同时展示独立示例与完整 Chat 界面。

在两个标签页打开 `http://127.0.0.1:3100/chat/live?lang=zh`，可以看到 Topcoat 0.9 的服务端推送如何实时更新现有 Chat 组件。此页面是共享的 Gallery 示例会话，不连接模型，也不持久化消息。

当前范围与验收结果见 [AI 组件需求记录](docs/ai-components.md)。
分阶段开发任务的完成状态见 [Chat 组件开发待办清单](docs/chat-todo.md)。
公共 API 和宿主接入边界见 [Chat 组件文档](docs/components/chat.md)。

[完整快速开始](https://github.com/mosenet-labs/topcoat-ant-design/blob/main/docs/getting-started.md)说明了依赖、页面资源、AssetBundle 和 Router 的接入关系。

组件库当前面向 Topcoat 0.9.0。交互式宿主需要在 Router 上启用 `.runtime()`，并在文档 `<head>` 中加入 `topcoat::runtime::script()`。

## 添加依赖

在应用中添加发布版本：

```toml
[dependencies]
topcoat-ant-design = "=0.2.0"
```

在发布前或开发组件库时，可以使用本地路径：

```toml
[dependencies]
topcoat-ant-design = { path = "../topcoat-ant-design" }
```

默认 feature 已足够渲染组件。只有宿主采用选择性路由发现并需要显式注册字体时，才启用 `router`：

```toml
topcoat-ant-design = { version = "=0.2.0", features = ["router"] }
```

## 接入样式与字体

组件使用 Tailwind 工具类，组件 CSS 由 UI crate 自己在构建期生成，并通过 `STYLESHEET` 声明为 Topcoat `Asset`。宿主不需要把 UI crate 加入构建依赖，不需要读取组件 CSS 路径，也不需要为组件启用 Tailwind。

宿主仍需像其他 Topcoat 应用一样统一配置一次静态资源托管。在根布局调用 `head_assets()` 后，Topcoat 的资源打包器会发现组件样式并把它放进最终应用的资源包：

```rust,ignore
Ok(view! {
    <head>
        topcoat_ant_design::head_assets()
    </head>
})
```

使用完整 `.discover()` 的应用会同时发现 Fontsource 字体路由，不需要额外注册。只发现部分路由的应用可启用 `router` feature，并调用一次扩展：

```toml
topcoat-ant-design = { version = "=0.2.0", features = ["router"] }
```

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

`app_assets` 是宿主从 Topcoat 构建产物加载的资源包，不是组件库提供的全局变量。不能只靠 Cargo 依赖自动修改宿主的 Router 和 HTML，因此静态资源托管与 `head_assets()` 是显式接入点；组件 CSS 的生成、定位和打包元数据均由 UI crate 维护。

默认字体为 JetBrains Mono，包含界面使用的 400、500、600 和 700 字重。它不包含中文字形，中文会按字体栈回退到系统中文字体。

当前 Fontsource 使用默认的 jsDelivr 字体来源。UI crate 在全新构建环境中会通过 Topcoat 下载锁定的 Tailwind CLI；离线构建和字体自托管需要另外提供对应资源。组件样式不包含 Tailwind Preflight，不会重置宿主页面的全局元素样式。

## Topcoat 原生 UI 组件

Topcoat 0.9.0 官方 registry 的全部 31 个组件直接从 `topcoat_ant_design` 导出，也可通过 `topcoat_ant_design::ui` 下的对应模块访问。例如 `use topcoat_ant_design::{button, ButtonVariant, dialog, dialog_content};`。官方组件的来源记录在 [components.toml](components.toml)。

根目录直接导出的 API 自 `0.2.0` 版本起可用。

原有的 Accordion、Dialog、Dropdown Menu、Tabs、Tooltip 自定义实现已由官方组件替换，直接使用根目录导出的官方名称及组合 API。`data_table`、`drawer`、`popconfirm`、`notification` 等具有独立功能的复合组件继续保留；Drawer 内部使用官方 Sheet。

常规 `head_assets()` 同时加载官方及自定义组件样式和项目字体。它们共用当前库的 Ant Design 主题变量，不需要额外包裹类或 Cargo feature。祖先元素设置 `class="dark"` 可切换深色主题。

运行 Gallery 后打开 [Topcoat 原生组件展示页](http://127.0.0.1:3100/topcoat-ui)，可以直接试用明暗主题、Sidebar、表单字段、弹层、表格等组件。

原始源码来自 Topcoat 官方 `v0.9.0` 标签对应的提交 `96e8f9e0932ea883ced2859d462e9d6d3f52ea59`，见[上游许可证](assets/topcoat-upstream-LICENSE)。项目直接导入官方 registry 源码，不依赖 Topcoat 的 `ui` Cargo feature。

## 接入图标

UI 库在 `build.rs` 中通过 Topcoat `icon-iconify` 暂存 Ant Design 图标集，并只公开项目实际使用的 `IconData` 常量。宿主直接调用 Topcoat 原生 `icon` 组件，不需要图标字体或浏览器端 CDN：

```rust,ignore
use topcoat_ant_design::icons::PROJECT_OUTLINED;
use topcoat::icon::icon;

Ok(view! {
    icon(data: PROJECT_OUTLINED, label: "GitLab 项目")
})
```

图标集缓存位于 `icons/ant-design.json` 并纳入版本控制，CI 和 Docker 构建可以离线完成图标生成。完整语义和尺寸说明见 [Icons 文档](https://docs.rs/topcoat-ant-design/latest/topcoat_ant_design/icons/index.html)。

## 接入组件

Notification 接收调用方创建的消息 signal：

```rust,ignore
use topcoat_ant_design::{NotificationTone, notification};
use topcoat::runtime::signal;

let notice = signal(cx, String::new);

Ok(view! {
    <button @click=$(|_e| notice.set("保存成功。".to_owned()))>"保存"</button>
    notification(
        message: &notice,
        title: "操作成功",
        tone: NotificationTone::Success,
    )
})
```

Popconfirm 使用一个稳定且可信的 DOM `id` 关联触发按钮和气泡。业务事件写在调用方提供的确认按钮中；组件会在确认点击后立即关闭并重置自身状态：

```rust,ignore
use topcoat_ant_design::{popconfirm, popconfirm_trigger_attributes};

let trigger = popconfirm_trigger_attributes(cx, "disable-instance");

Ok(view! {
    <button type="button" (trigger)>"停用"</button>
    popconfirm(
        id: "disable-instance",
        title: "确认停用此实例？",
        <button type="button" @click=$(|_e| {
            notice.set("实例已停用。".to_owned());
        })>"确认"</button>
    )
})
```

组件 `id` 用于 DOM 和 CSS 锚点名称，应由代码生成，不应直接使用未经校验的用户输入。

Popconfirm 默认显示在触发按钮下方；靠近视口左右边缘时会平移，底部空间不足时会翻转到上方。箭头位置在浏览器本地通过 Topcoat `@toggle` 事件计算，始终限制在气泡圆角以内并指向触发按钮。

Collapse 是接收 Topcoat signal 的组件，触发按钮属性也由组件库通过 `attributes!` 生成：

```rust,ignore
use topcoat_ant_design::{collapse, collapse_trigger_attributes};
use topcoat::{runtime::signal, view::view};

// 根布局已经调用 topcoat_ant_design::head_assets()。
let open = signal(cx, || true);
let trigger = collapse_trigger_attributes(cx, "advanced-options", &open);

Ok(view! {
    <button type="button" (trigger)>"展开/收起"</button>
    collapse(
        id: "advanced-options",
        open: &open,
        <div>"任意高度的内容"</div>
    )
})
```

组件样式使用 `0fr`/`1fr` 网格轨道完成双向高度过渡，并同步处理透明度、可见性和 `prefers-reduced-motion`。

Tabs 由官方 `tabs`、`tabs_list`、`tabs_trigger` 和 `tabs_content` 组成。宿主根据当前 Topcoat 路由传入 `active`，组件输出真实链接与 `aria-current`。Drawer 接收 `DrawerConfig` 和 `Signal<bool>`；由查询参数控制详情时，可以通过 `DrawerConfig::with_close_href` 设置不含详情参数的关闭地址。

## 公开接口

| 接口 | 用途 | 额外 feature |
| --- | --- | --- |
| `icons` | 提供编译期校验的 Ant Design `IconData` 常量 | 无 |
| `notification`、`NotificationTone` | 展示页面级操作反馈 | 无 |
| `popconfirm`、`popconfirm_trigger_attributes` | 建立确认气泡及其触发关系 | 无 |
| `dropdown_menu`、`dropdown_menu_trigger`、`dropdown_menu_content` | 组合官方操作菜单 | 无 |
| `collapse`、`collapse_trigger_attributes` | 建立可访问的折叠内容及触发关系 | 无 |
| `tabs`、`tabs_list`、`tabs_trigger`、`tabs_content` | 建立路由型详情页签 | 无 |
| `drawer` | 展示可关闭的右侧详情面板 | 无 |
| `head_assets` | 在根布局加载组件 CSS 和默认字体 | 无 |
| `RouterBuilderUiExt` | 为选择性发现的 Router 注册字体路由 | `router` |

## 浏览组件

复合组件的内置按钮和无障碍标签默认使用英文。中文页面可向 Notification、Popconfirm、Drawer 和 DateTimeRange 传入 `language: UiLanguage::ChineseSimplified`。

在仓库根目录运行：

```bash
cargo run -p topcoat-ant-design \
  --bin topcoat-ant-design-gallery \
  --features gallery
```

默认入口和组件页面为：

- `http://127.0.0.1:3100/`：快速开始与完整接入说明；
- `http://127.0.0.1:3100/overview`：组件概览；
- `http://127.0.0.1:3100/icons`：Topcoat Iconify 图标目录；
- `http://127.0.0.1:3100/chat`：Chat 聊天界面，含真实会话路由和服务端示例过程函数，不连接模型；
- `http://127.0.0.1:3100/chat/new`：空会话、多轮消息和请求状态演示；
- `http://127.0.0.1:3100/bubble`：ChatBubble 的用户与助手消息示例；
- `http://127.0.0.1:3100/message-list`：ChatMessageList 的会话区域示例；
- `http://127.0.0.1:3100/sender`：ChatSender 的本地发送示例；
- `http://127.0.0.1:3100/notification`：Notification；
- `http://127.0.0.1:3100/popconfirm`：Popconfirm；
- `http://127.0.0.1:3100/dropdown-menu`：Dropdown 下拉菜单；
- `http://127.0.0.1:3100/collapse`：Collapse 动画；
- `http://127.0.0.1:3100/tabs`：Tabs 路由页签；
- `http://127.0.0.1:3100/drawer`：Drawer。

可以使用 `HOST` 和 `PORT` 环境变量覆盖监听地址。Gallery 使用 `app.rs` 根路由、下划线逻辑分组、无路径 `#[page]`、根 `#[layout]` 和类型安全的 `href!`；现有基础组件页同时展示真实交互效果与公开 API 共用的 Markdown 文档。

Gallery 只修改浏览器中的 Topcoat signal，不连接业务服务。

## 许可

本项目代码采用 MIT 许可证，正文见根目录的 `LICENSE`。`icons/ant-design.json` 来源于 Ant Design Icons，其 MIT 许可声明保存在 `LICENSES/ant-design-icons-MIT.txt`。
