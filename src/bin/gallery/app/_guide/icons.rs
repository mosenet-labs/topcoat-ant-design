use crate::locale::Locale;
use crate::locale::text;
use topcoat::{
    Result,
    icon::{IconData, icon},
    router::page,
    view::{View, component, view},
};
use topcoat_ant_design::icons::{
    APARTMENT_OUTLINED, APPSTORE_OUTLINED, ARROW_RIGHT_OUTLINED, AUDIT_OUTLINED,
    CHECK_CIRCLE_FILLED, FILE_DONE_OUTLINED, GITLAB_OUTLINED, INFO_CIRCLE_FILLED, MENU_OUTLINED,
    PROJECT_OUTLINED, RELOAD_OUTLINED, SEARCH_OUTLINED, TEAM_OUTLINED, WARNING_FILLED,
};

use crate::{app::page_header, markdown::markdown_document};

const DOCUMENTATION_EN: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/en/icons.md"));
const DOCUMENTATION_ZH: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/icons.md"));

#[component]
async fn icon_sample(data: IconData, name: &str, description: &str) -> Result<impl View> {
    Ok(view! {
        <article class="flex min-h-32 flex-col justify-between rounded-lg border border-[#e8eaee] bg-white p-4 transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-sm">
            <span class="text-[28px] text-[#1677ff]">icon(data: data)</span>
            <div><strong class="mt-5 block text-sm font-semibold">(name)</strong><small class="mt-1 block text-xs leading-5 text-[#8c8c8c]">(description)</small></div>
        </article>
    })
}

#[page]
pub(in crate::app) async fn icons_page(cx: &topcoat::context::Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(DOCUMENTATION_EN, DOCUMENTATION_ZH);
    Ok(view! {
        page_header(
            eyebrow: "FOUNDATION",
            title: text(locale, "Icons 图标"),
            description: text(locale, "由 Topcoat Iconify 在编译期校验并生成的 Ant Design 内联 SVG 图标。"),
        )
        <section class="mb-8 rounded-xl border border-[#e8eaee] bg-white p-6 shadow-sm" aria-labelledby="icon-set-title">
            <h2 class="mb-2 mt-0 text-lg font-semibold" id="icon-set-title">(text(locale, "控制台图标集"))</h2>
            <p class="mb-5 mt-0 text-sm leading-6 text-[#595959]">(text(locale, "图标继承文字颜色，默认跟随字号缩放；下列项目均直接使用 Topcoat icon 组件渲染。"))</p>
            <div class="grid grid-cols-4 gap-3 max-[760px]:grid-cols-2 max-[440px]:grid-cols-1">
                icon_sample(data: APPSTORE_OUTLINED, name: "Appstore", description: text(locale, "运行总览"))
                icon_sample(data: FILE_DONE_OUTLINED, name: "FileDone", description: text(locale, "审查任务"))
                icon_sample(data: PROJECT_OUTLINED, name: "Project", description: text(locale, "GitLab 项目"))
                icon_sample(data: GITLAB_OUTLINED, name: "GitLab", description: text(locale, "GitLab 实例"))
                icon_sample(data: APARTMENT_OUTLINED, name: "Apartment", description: text(locale, "LDAP 配置"))
                icon_sample(data: TEAM_OUTLINED, name: "Team", description: text(locale, "用户与权限"))
                icon_sample(data: AUDIT_OUTLINED, name: "Audit", description: text(locale, "审计日志"))
                icon_sample(data: SEARCH_OUTLINED, name: "Search", description: text(locale, "搜索"))
                icon_sample(data: RELOAD_OUTLINED, name: "Reload", description: text(locale, "刷新"))
                icon_sample(data: MENU_OUTLINED, name: "Menu", description: text(locale, "菜单"))
                icon_sample(data: ARROW_RIGHT_OUTLINED, name: "ArrowRight", description: text(locale, "跳转"))
                icon_sample(data: CHECK_CIRCLE_FILLED, name: "CheckCircle", description: text(locale, "成功状态"))
                icon_sample(data: INFO_CIRCLE_FILLED, name: "InfoCircle", description: text(locale, "信息状态"))
                icon_sample(data: WARNING_FILLED, name: "Warning", description: text(locale, "警告状态"))
            </div>
        </section>
        markdown_document(source: document)
    })
}
