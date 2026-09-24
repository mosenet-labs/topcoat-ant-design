use topcoat::{
    Result,
    asset::{Asset, asset},
    font::{Font, fontsource::fontsource_font},
    view::{View, component, view},
};
#[cfg(feature = "router")]
use topcoat::{font::RouterBuilderFontExt, router::RouterBuilder};

/// Component stylesheet built by this crate and bundled by the final application.
pub const STYLESHEET: Asset = asset!(
    concat!(env!("OUT_DIR"), "/topcoat-ant-design.css"),
    rename: "topcoat-ant-design-css",
);

/// Use this to build an asset manifest when embedding assets in a single binary.
#[doc(hidden)]
pub const STYLESHEET_SHA256: &str =
    include_str!(concat!(env!("OUT_DIR"), "/component-tailwind.sha256"));

/// Read the built stylesheet; regular Topcoat applications should use [`STYLESHEET`].
#[doc(hidden)]
#[must_use]
pub fn embedded_stylesheet() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/topcoat-ant-design.css"))
}

/// Default UI font; Chinese glyphs fall back to the host system font stack.
pub const DEFAULT_FONT: Font = fontsource_font!(
    JETBRAINS_MONO,
    weight: [400, 500, 600, 700],
    style: Normal,
    subset: Latin,
);

/// Register the Topcoat server assets required by these UI components.
#[cfg(feature = "router")]
pub trait RouterBuilderUiExt {
    /// Register the default UI fonts. The application still configures Topcoat static asset hosting.
    #[must_use]
    fn topcoat_ant_design(self) -> Self;
}

#[cfg(feature = "router")]
impl RouterBuilderUiExt for RouterBuilder {
    fn topcoat_ant_design(self) -> Self {
        self.font(DEFAULT_FONT)
    }
}

/// Render the shared font and stylesheet assets for UI components.
#[component]
pub async fn head_assets() -> Result<impl View> {
    Ok(view! {
        topcoat::font::link(font: DEFAULT_FONT)
        <link rel="stylesheet" href=(STYLESHEET)>
    })
}
