#![doc = include_str!("../README.md")]

mod components;
pub mod icons;
mod language;
/// Topcoat 0.9.0 native UI components, vendored from its official registry.
#[cfg(feature = "native-ui")]
pub mod native_ui;
mod theme;

pub use components::accordion::{AccordionItemConfig, accordion_item};
pub use components::chat::{
    ChatBubbleRole, ChatMessage, ChatMessageStatus, ChatThoughtStatus, chat_actions,
    chat_attachment_tray, chat_bubble, chat_conversation_item, chat_conversation_list, chat_file,
    chat_markdown, chat_message_list, chat_prompt, chat_prompts, chat_sender, chat_source,
    chat_sources, chat_think, chat_thought_chain, chat_thought_step, render_chat_markdown,
};
pub use components::collapse::{collapse, collapse_trigger_attributes};
pub use components::date_time_range::{DateTimeRangeConfig, date_time_range_filter};
pub use components::dialog::{
    DialogConfig, dialog, dialog_close_attributes, dialog_trigger_attributes,
};
pub use components::drawer::{DrawerConfig, drawer};
pub use components::dropdown_menu::{dropdown_menu, dropdown_menu_trigger_attributes};
pub use components::form_field::{FormFieldConfig, form_field};
pub use components::notification::{NotificationTone, notification};
pub use components::popconfirm::{popconfirm, popconfirm_trigger_attributes};
pub use components::table::{
    DataTableDensity, data_table, table_page_size_select, table_pagination,
};
pub use components::tabs::{tab_link, tabs};
pub use components::tooltip::tooltip;
pub use language::UiLanguage;
#[cfg(feature = "router")]
pub use theme::RouterBuilderUiExt;
pub use theme::{DEFAULT_FONT, STYLESHEET, head_assets};
#[doc(hidden)]
pub use theme::{STYLESHEET_SHA256, embedded_stylesheet};

pub use components::tag::{TagTone, tag};
