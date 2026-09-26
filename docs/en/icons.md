# Icons

The UI library uses Topcoat's `icon` and `icon-iconify` support for a consistent icon set. Icons come from Iconify's Ant Design collection. Names are validated at build time and converted to `IconData`, then rendered as inline SVG.

## Usage

Applications depend directly on Topcoat with icon and view support:

```toml
[dependencies]
topcoat-ant-design = "=0.2.0-dev.1"
topcoat = { version = "=0.9.0", default-features = false, features = ["icon", "view"] }
```

Import a selected icon constant from the UI library and render it with Topcoat's native component:

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
            <span>"GitLab projects"</span>
        </a>
    })
}
```

## Size and meaning

Icons default to `1em`, following the surrounding text size and `currentColor`. Pass `size` for fixed dimensions:

```rust,ignore
icon(data: PROJECT_OUTLINED, size: 20)
```

Omit `label` for decorative icons; Topcoat automatically sets `aria-hidden="true"`. Give a standalone action icon an accessible name:

```rust,ignore
icon(data: PROJECT_OUTLINED, label: "GitLab projects")
```

## Build process

`build.rs` stages the `ant-design` icon set through `BuildConfig`. The cache is committed at `icons/ant-design.json`, so CI and Docker builds do not need a live Iconify request. Add new `iconify::include!` declarations in `src/icons.rs`; an invalid icon name fails compilation.
