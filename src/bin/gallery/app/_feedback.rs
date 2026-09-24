mod dialog;
mod drawer;
mod notification;
mod popconfirm;

pub(in crate::app) use dialog::dialog_page;
pub(in crate::app) use drawer::drawer_page;
pub(in crate::app) use notification::notification_page;
pub(in crate::app) use popconfirm::popconfirm_page;
mod tooltip;
pub(in crate::app) use tooltip::tooltip_page;

mod tag;
pub(in crate::app) use tag::tag_page;
