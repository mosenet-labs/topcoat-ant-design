use crate::locale::Locale;
use crate::locale::text;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::signal,
    view::{View, view},
};
use topcoat_ant_design::{NotificationTone, notification};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const NOTIFICATION_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/notification.md"
));
const NOTIFICATION_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/notification.md"
));

#[page]
pub(in crate::app) async fn notification_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(NOTIFICATION_DOC_EN, NOTIFICATION_DOC_ZH);
    let success = signal(cx, String::new);
    let info = signal(cx, String::new);
    let warning = signal(cx, String::new);
    let error = signal(cx, String::new);
    let example_source = rust_code_block(document, 0);
    let success_message = text(locale, "GitLab 实例“研发主站”已验证。");
    let info_message = text(locale, "项目列表已经同步到最新状态。");
    let warning_message = text(locale, "Token 将在 7 天后到期，请及时更新。");
    let error_message = text(locale, "无法连接 GitLab，请检查地址和 Token。");

    Ok(view! {
        page_header(
            eyebrow: "FEEDBACK",
            title: text(locale, "Notification 通知提醒框"),
            description: text(locale, "用于保存、验证和同步等操作反馈。下方说明与 notification 方法的 Rustdoc 来自同一份 Markdown。"),
        )
        <div class="grid gap-6">
            component_example(id: "notification-preview", title: text(locale, "组件预览"), description: text(locale, "点击按钮查看不同语义的通知。"), source: example_source,
                <div class="grid grid-cols-4 gap-4 p-6 max-[780px]:grid-cols-2 max-[520px]:grid-cols-1">
                    <article class="rounded-lg border border-[#edf0f4] bg-[#fafafa] p-5"><span class="mb-4 block h-1 w-8 rounded bg-[#52c41a]"></span><h3 class="m-0 text-base font-semibold">"Success"</h3><p class="mb-5 mt-2 min-h-11 text-sm leading-6 text-[#595959]">(text(locale, "保存、验证和启用成功。"))</p><button class="h-8 cursor-pointer rounded-md border border-[#91caff] bg-white px-3 font-mono text-sm text-[#1677ff] hover:border-[#1677ff] hover:text-[#0958d9]" type="button" @click=$(|_e| { info.set("".to_owned()); warning.set("".to_owned()); error.set("".to_owned()); success.set(success_message.to_owned()); })>(text(locale, "显示通知"))</button></article>
                    <article class="rounded-lg border border-[#edf0f4] bg-[#fafafa] p-5"><span class="mb-4 block h-1 w-8 rounded bg-[#1677ff]"></span><h3 class="m-0 text-base font-semibold">"Info"</h3><p class="mb-5 mt-2 min-h-11 text-sm leading-6 text-[#595959]">(text(locale, "普通状态和操作说明。"))</p><button class="h-8 cursor-pointer rounded-md border border-[#91caff] bg-white px-3 font-mono text-sm text-[#1677ff] hover:border-[#1677ff] hover:text-[#0958d9]" type="button" @click=$(|_e| { success.set("".to_owned()); warning.set("".to_owned()); error.set("".to_owned()); info.set(info_message.to_owned()); })>(text(locale, "显示通知"))</button></article>
                    <article class="rounded-lg border border-[#edf0f4] bg-[#fafafa] p-5"><span class="mb-4 block h-1 w-8 rounded bg-[#faad14]"></span><h3 class="m-0 text-base font-semibold">"Warning"</h3><p class="mb-5 mt-2 min-h-11 text-sm leading-6 text-[#595959]">(text(locale, "需要注意但仍可继续。"))</p><button class="h-8 cursor-pointer rounded-md border border-[#91caff] bg-white px-3 font-mono text-sm text-[#1677ff] hover:border-[#1677ff] hover:text-[#0958d9]" type="button" @click=$(|_e| { success.set("".to_owned()); info.set("".to_owned()); error.set("".to_owned()); warning.set(warning_message.to_owned()); })>(text(locale, "显示通知"))</button></article>
                    <article class="rounded-lg border border-[#edf0f4] bg-[#fafafa] p-5"><span class="mb-4 block h-1 w-8 rounded bg-[#ff4d4f]"></span><h3 class="m-0 text-base font-semibold">"Error"</h3><p class="mb-5 mt-2 min-h-11 text-sm leading-6 text-[#595959]">(text(locale, "需要立即关注的操作失败。"))</p><button class="h-8 cursor-pointer rounded-md border border-[#91caff] bg-white px-3 font-mono text-sm text-[#1677ff] hover:border-[#1677ff] hover:text-[#0958d9]" type="button" @click=$(|_e| { success.set("".to_owned()); info.set("".to_owned()); warning.set("".to_owned()); error.set(error_message.to_owned()); })>(text(locale, "显示通知"))</button></article>
                </div>
            )
            markdown_document(source: document)
            notification(language: locale.ui(), message: &success, title: text(locale, "操作成功"), tone: NotificationTone::Success)
            notification(language: locale.ui(), message: &info, title: text(locale, "操作提示"), tone: NotificationTone::Info)
            notification(language: locale.ui(), message: &warning, title: text(locale, "请注意"), tone: NotificationTone::Warning)
            notification(language: locale.ui(), message: &error, title: text(locale, "操作失败"), tone: NotificationTone::Error)
        </div>
    })
}
