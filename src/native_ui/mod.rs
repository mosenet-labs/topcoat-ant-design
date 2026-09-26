pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod dialog;
pub mod dropdown_menu;
pub mod field;
pub mod hover_card;
pub mod input;
pub mod kbd;
pub mod label;
pub mod pagination;
pub mod progress;
pub mod radio_group;
pub mod select;
pub mod separator;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod toggle;
pub mod tooltip;

use topcoat::{
    Result,
    asset::{Asset, asset},
    view::{View, component, view},
};

/// Standalone stylesheet for the vendored Topcoat components.
/// Wrap native components in an element with the `native-ui` class to apply the neutral theme.
pub const STYLESHEET: Asset = asset!(
    concat!(env!("OUT_DIR"), "/topcoat-native-ui.css"),
    rename: "topcoat-native-ui-css",
);

/// Add the native UI stylesheet to a Topcoat document head.
#[component]
pub async fn head_assets() -> Result<impl View> {
    Ok(view! { <link rel="stylesheet" href=(STYLESHEET)> })
}
