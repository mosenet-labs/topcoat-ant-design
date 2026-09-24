use topcoat::{
    Result,
    asset::{Asset, asset},
    font::{Font, fontsource::fontsource_font},
    view::{View, component, view},
};
#[cfg(feature = "router")]
use topcoat::{font::RouterBuilderFontExt, router::RouterBuilder};

/// UI crate 构建并交给最终应用打包的组件样式。
pub const STYLESHEET: Asset = asset!(
    concat!(env!("OUT_DIR"), "/topcoat-ant-design.css"),
    rename: "topcoat-ant-design-css",
);

/// 供需要把资源内嵌进单一二进制的宿主构造资源清单。
#[doc(hidden)]
pub const STYLESHEET_SHA256: &str =
    include_str!(concat!(env!("OUT_DIR"), "/component-tailwind.sha256"));

/// 读取构建后的样式；普通 Topcoat 应用应使用 [`STYLESHEET`]。
#[doc(hidden)]
#[must_use]
pub fn embedded_stylesheet() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/topcoat-ant-design.css"))
}

/// UI 组件的默认字体；中文字符由调用方系统字体栈回退显示。
pub const DEFAULT_FONT: Font = fontsource_font!(
    JETBRAINS_MONO,
    weight: [400, 500, 600, 700],
    style: Normal,
    subset: Latin,
);

/// 注册 UI 组件依赖的 Topcoat 服务端资源。
#[cfg(feature = "router")]
pub trait RouterBuilderUiExt {
    /// 注册 UI 默认字体；应用自身仍统一配置 Topcoat 静态资源托管。
    #[must_use]
    fn topcoat_ant_design(self) -> Self;
}

#[cfg(feature = "router")]
impl RouterBuilderUiExt for RouterBuilder {
    fn topcoat_ant_design(self) -> Self {
        self.font(DEFAULT_FONT)
    }
}

/// 渲染 UI 组件共用的字体与样式资源。
#[component]
pub async fn head_assets() -> Result<impl View> {
    Ok(view! {
        topcoat::font::link(font: DEFAULT_FONT)
        <link rel="stylesheet" href=(STYLESHEET)>
    })
}
