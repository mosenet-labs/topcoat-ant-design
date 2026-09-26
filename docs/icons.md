# Icons 图标

UI 库使用 Topcoat 的 `icon` 与 `icon-iconify` 能力提供统一图标。图标来源于 Iconify 的 Ant Design 图标集，构建时校验名称并生成 `IconData`，最终以内联 SVG 输出。

## 使用

应用需要直接依赖 Topcoat，并启用图标与视图能力：

```toml
[dependencies]
topcoat-ant-design = "0.1.2"
topcoat = { version = "=0.9.0", default-features = false, features = ["icon", "view"] }
```

从 UI 库取得经过筛选的图标常量，再交给 Topcoat 原生组件渲染：

```rust
use topcoat_ant_design::icons::PROJECT_OUTLINED;
use topcoat::{
    Result,
    icon::icon,
    view::{View, component, view},
};

#[component]
async fn project_link() -> Result<impl View> {
    Ok(view! {
        <a href="/projects">
            icon(data: PROJECT_OUTLINED)
            <span>"GitLab 项目"</span>
        </a>
    })
}
```

## 尺寸与语义

图标默认是 `1em`，会跟随周围文字大小和 `currentColor`。需要固定尺寸时传入 `size`：

```rust,ignore
icon(data: PROJECT_OUTLINED, size: 20)
```

装饰图标不传 `label`，Topcoat 会自动设置 `aria-hidden="true"`。独立表达操作含义的图标应传入可访问名称：

```rust,ignore
icon(data: PROJECT_OUTLINED, label: "GitLab 项目")
```

## 构建方式

`build.rs` 使用 `BuildConfig` 暂存 `ant-design` 图标集。缓存文件保存在 `icons/ant-design.json` 并纳入版本控制，因此 CI 和 Docker 构建不需要临时请求 Iconify。新增图标时，在 `src/icons.rs` 中增加对应的 `iconify::include!` 声明；无效名称会直接导致编译失败。
