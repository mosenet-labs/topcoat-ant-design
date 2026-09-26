use crate::locale::Locale;
use crate::locale::text;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, expr, signal},
    view::{View, attributes, view},
};
use topcoat_ant_design::{DataTableDensity, data_table, table_page_size_select, table_pagination};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const TABLE_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/table.md"
));
const TABLE_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/table.md"
));

#[page]
pub(in crate::app) async fn table_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(TABLE_DOC_EN, TABLE_DOC_ZH);
    let page = signal(cx, || "1".to_owned());
    let page_size = signal(cx, || "2".to_owned());
    let last_page = expr!(if page_size.get() == "4" {
        true
    } else {
        page.get() == "2"
    });
    let compact_source = rust_code_block(document, 0);
    let default_source = rust_code_block(document, 1);

    Ok(view! {
        page_header(
            eyebrow: "DATA DISPLAY",
            title: text(locale, "Table 数据表格"),
            description: text(locale, "使用原生表格语义展示结构化数据，并组合 Topcoat signal 或真实链接完成分页。"),
        )
        <div class="grid gap-6">
            component_example(id: "table-preview", title: text(locale, "紧凑表格与页码分页"), description: text(locale, "点击分页按钮，行内容和禁用状态由 Topcoat signal 在浏览器中同步更新。"), source: compact_source,
                data_table(label: text(locale, "GitLab 项目示例"), density: DataTableDensity::Compact, attrs: attributes! { class="min-w-[720px]" },
                    <thead><tr><th>(text(locale, "项目"))</th><th>(text(locale, "所属实例"))</th><th>"Webhook"</th><th>(text(locale, "最近事件"))</th></tr></thead>
                    <tbody>
                        <tr :hidden=$(if page_size.get() == "2" { page.get() != "1" } else { false })><td><strong>"devops/gitlab-review"</strong></td><td>"gl"</td><td><span class="rounded bg-[var(--gr-success-soft)] px-1.5 py-0.5 text-xs text-[var(--gr-success)]">(text(locale, "已生效"))</span></td><td>"2026-09-15 17:49"</td></tr>
                        <tr :hidden=$(if page_size.get() == "2" { page.get() != "1" } else { false })><td><strong>"platform/console"</strong></td><td>"gl1"</td><td><span class="rounded bg-[var(--gr-warning-soft)] px-1.5 py-0.5 text-xs text-[var(--gr-warning)]">(text(locale, "待验证"))</span></td><td>(text(locale, "尚无事件"))</td></tr>
                        <tr :hidden=$(if page_size.get() == "2" { page.get() != "2" } else { false })><td><strong>"security/rules"</strong></td><td>"gl"</td><td><span class="rounded bg-[var(--gr-success-soft)] px-1.5 py-0.5 text-xs text-[var(--gr-success)]">(text(locale, "已生效"))</span></td><td>"2026-09-15 16:48"</td></tr>
                        <tr :hidden=$(if page_size.get() == "2" { page.get() != "2" } else { false })><td><strong>"devops/runner"</strong></td><td>"gl1"</td><td><span class="rounded bg-background px-1.5 py-0.5 text-xs text-muted-foreground">(text(locale, "未配置"))</span></td><td>(text(locale, "尚无事件"))</td></tr>
                    </tbody>
                )
                table_pagination(summary: text(locale, "共 4 个项目"), label: text(locale, "组件示例分页"),
                    table_page_size_select(id: "gallery-table-page-size", label: text(locale, "每页记录"), attrs: attributes! {
                        :value=$(page_size.get())
                        @change=$(|event: Event| { page_size.set(event.target.value); page.set("1".to_owned()); })
                    },
                        <option value="2">"2"</option>
                        <option value="4">"4"</option>
                    )
                    <button type="button" :disabled=$(page.get() == "1") @click=$(|_e| page.set("1".to_owned()))>(text(locale, "上一页"))</button>
                    <span aria-current="page">$(page.get()) " / " $(if page_size.get() == "4" { "1" } else { "2" })</span>
                    <button type="button" :disabled=$(last_page) @click=$(|_e| page.set("2".to_owned()))>(text(locale, "下一页"))</button>
                )
            )
            component_example(id: "table-default-preview", title: text(locale, "默认密度"), description: text(locale, "适合单元格包含说明文字或操作入口的管理表格。"), source: default_source,
                data_table(label: text(locale, "默认密度示例"),
                    <thead><tr><th>(text(locale, "账号"))</th><th>(text(locale, "角色"))</th><th>(text(locale, "状态"))</th></tr></thead>
                    <tbody><tr><td><strong>"demo-admin"</strong><br><small class="text-muted-foreground">"demo.admin@example.com"</small></td><td>(text(locale, "管理员"))</td><td>(text(locale, "启用"))</td></tr></tbody>
                )
            )
            markdown_document(source: document)
        </div>
    })
}
