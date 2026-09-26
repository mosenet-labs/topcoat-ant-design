use topcoat_ant_design::ui as components;

use crate::{app::page_header, demo::component_example, locale::Locale};
use components::{
    accordion::{accordion, accordion_content, accordion_item, accordion_trigger},
    alert::{AlertVariant, alert, alert_description, alert_title},
    alert_dialog::alert_dialog,
    avatar::{AvatarSize, avatar, avatar_fallback, avatar_image},
    badge::{BadgeVariant, badge, badge_variants},
    breadcrumb::{
        breadcrumb, breadcrumb_ellipsis, breadcrumb_item, breadcrumb_link, breadcrumb_list,
        breadcrumb_page, breadcrumb_separator,
    },
    button::{ButtonSize, ButtonVariant, button, button_variants},
    card::{card, card_content, card_description, card_footer, card_header, card_title},
    checkbox::checkbox,
    dialog::{
        dialog, dialog_content, dialog_description, dialog_footer, dialog_header, dialog_title,
    },
    dropdown_menu::{
        dropdown_menu, dropdown_menu_content, dropdown_menu_item, dropdown_menu_label,
        dropdown_menu_separator, dropdown_menu_sub, dropdown_menu_sub_content,
        dropdown_menu_sub_trigger, dropdown_menu_trigger,
    },
    field::{
        FieldLegendVariant, FieldOrientation, field, field_content, field_description, field_error,
        field_group, field_label, field_legend, field_separator, field_set,
    },
    hover_card::{hover_card, hover_card_content},
    input::input,
    kbd::{kbd, kbd_group},
    pagination::{
        pagination, pagination_content, pagination_ellipsis, pagination_item, pagination_link,
        pagination_next, pagination_previous,
    },
    progress::progress,
    radio_group::{radio_group, radio_group_item},
    select::select,
    separator::{SeparatorOrientation, separator},
    sheet::{sheet, sheet_content},
    sidebar::{
        SidebarCollapsible, SidebarVariant, sidebar, sidebar_content, sidebar_footer,
        sidebar_header, sidebar_inset, sidebar_menu, sidebar_menu_button, sidebar_menu_item,
        sidebar_provider, sidebar_trigger,
    },
    skeleton::skeleton,
    spinner::spinner,
    switch::switch,
    table::{
        table, table_body, table_caption, table_cell, table_footer, table_head, table_header,
        table_row,
    },
    tabs::{tabs, tabs_content, tabs_list, tabs_trigger},
    textarea::textarea,
    toggle::{ToggleKind, ToggleSize, toggle, toggle_group},
    tooltip::{tooltip, tooltip_content},
};
use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::page,
    runtime::{Event, expr, shard, signal},
    view::{Child, View, attributes, component, view},
};

/// A stand-in portrait for the workspace's owner, served from the example's
/// own asset bundle.
const PORTRAIT: &str = "/assets/topcoat-ui-portrait.svg";

const DOCS: &str = "https://docs.rs/topcoat";
const REGISTRY: &str = "https://github.com/tokio-rs/topcoat/tree/main/crates/topcoat-ui/registry";

pub(in crate::app) const REGISTRY_COMPONENTS: [&str; 31] = [
    "accordion",
    "alert",
    "alert-dialog",
    "avatar",
    "badge",
    "breadcrumb",
    "button",
    "card",
    "checkbox",
    "dialog",
    "dropdown-menu",
    "field",
    "hover-card",
    "input",
    "kbd",
    "label",
    "pagination",
    "progress",
    "radio-group",
    "select",
    "separator",
    "sheet",
    "sidebar",
    "skeleton",
    "spinner",
    "switch",
    "table",
    "tabs",
    "textarea",
    "toggle",
    "tooltip",
];

/// Deployment statuses and their badge styles.
const STATUSES: [(&str, BadgeVariant); 4] = [
    ("Live", BadgeVariant::Primary),
    ("Building", BadgeVariant::Secondary),
    ("Queued", BadgeVariant::Outline),
    ("Failed", BadgeVariant::Destructive),
];

/// The badge variant the deployment status `status` shows in.
fn status_variant(status: &str) -> BadgeVariant {
    STATUSES
        .iter()
        .find(|(known, _)| *known == status)
        .map_or(BadgeVariant::default(), |(_, variant)| *variant)
}

#[page("/topcoat-ui")]
pub(in crate::app) async fn native_ui_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);

    Ok(view! {
        page_header(
            eyebrow: "TOPCOAT 0.9.0 / OFFICIAL UI",
            title: locale.select("Official Topcoat components", "Topcoat 官方组件"),
            description: locale.select(
                "Explore all 31 official modules in the same Gallery as our own components. Open any example to inspect the Rust source.",
                "在同一个组件 Gallery 中查看全部 31 个官方模块。展开任意示例即可查看对应的 Rust 代码。",
            ),
        )
        <section class="mb-8 rounded-xl border border-border bg-card p-6 shadow-sm" aria-labelledby="native-registry-heading">
            <div class="flex flex-wrap items-baseline justify-between gap-3">
                <h2 id="native-registry-heading" class="m-0 text-lg font-semibold">(locale.select("Component index", "组件索引"))</h2>
                <span class="text-xs text-muted-foreground">"Topcoat 0.9.0 · 31 / 31"</span>
            </div>
            <p class="mb-4 mt-2 text-sm leading-6 text-muted-foreground">(locale.select(
                "Each module links to a live example. Composite examples show related primitives together.",
                "每个模块都可以跳转到交互示例；复合示例会一起展示相关基础组件。",
            ))</p>
            <nav class="flex flex-wrap gap-2" aria-label=(locale.select("Official component index", "官方组件索引"))>
                for name in REGISTRY_COMPONENTS {
                    let target = format!("{}#{}", locale.link("/topcoat-ui"), registry_target(name));
                    <a class="rounded-md border border-border bg-background px-2.5 py-1 font-mono text-xs text-foreground no-underline transition-colors hover:border-primary hover:text-primary focus-visible:outline-2 focus-visible:outline-ring" href=(target.as_str())>(name)</a>
                }
            </nav>
        </section>
        <div id="components" class="grid min-w-0 gap-6 xl:grid-cols-2">
            demo(name: "buttons_card", title: locale.select("Buttons", "按钮"), description: locale.select("Variants, sizes, and disabled states.", "展示按钮样式、尺寸与禁用状态。"), buttons_card())
            demo(name: "notices", title: locale.select("Alerts", "警告提示"), description: locale.select("Information and destructive messages.", "展示信息提示与危险提示。"), notices())
            demo(name: "team_card", title: locale.select("Avatars and separator", "头像与分隔线"), description: locale.select("Images, initials, roles, and a divider.", "头像、姓名缩写、角色标记和分隔线。"), team_card())
            demo(name: "status_card", title: locale.select("Badges", "徽标"), description: locale.select("Status badges in all variants.", "不同状态与样式的徽标。"), status_card())
            demo(name: "progress_card", title: locale.select("Progress", "进度条"), description: locale.select("Known and indeterminate progress.", "确定进度与不确定进度。"), progress_card())
            demo(name: "form_card", title: locale.select("Fields and form controls", "字段与表单控件"), description: locale.select("Labels, input, select, textarea, and validation.", "标签、输入框、选择框、多行文本与校验。"), form_card())
            demo(name: "checks_card", title: locale.select("Checkboxes", "复选框"), description: locale.select("Checked, unchecked, and disabled states.", "选中、未选中与禁用状态。"), checks_card())
            demo(name: "switches_card", title: locale.select("Switches", "开关"), description: locale.select("Independent reactive switches.", "独立响应的开关控件。"), switches_card())
            demo(name: "radios_card", title: locale.select("Radio group", "单选组"), description: locale.select("Select one option from a group.", "从一组选项中选择一项。"), radios_card())
            demo(name: "overview_card", title: locale.select("Tabs", "标签页"), description: locale.select("Switch panels without reloading.", "无需刷新即可切换内容面板。"), overview_card())
            demo(name: "faq_card", title: locale.select("Accordion", "手风琴"), description: locale.select("Expand an answer in place.", "原地展开和收起内容。"), faq_card())
            demo(name: "branches_card", title: locale.select("Dropdown menu", "下拉菜单"), description: locale.select("Select branches and tags.", "选择分支与标签。"), branches_card())
            demo(name: "toolbar_card", title: locale.select("Toggle controls", "切换控件"), description: locale.select("Segmented and independent toggles.", "分段选择与独立切换。"), toolbar_card())
            demo(name: "tooltip_card", title: locale.select("Tooltip", "文字提示"), description: locale.select("A short hint on hover or focus.", "悬停或聚焦时显示简短提示。"), tooltip_card())
            demo(name: "hover_card_demo", title: locale.select("Hover card", "悬停卡片"), description: locale.select("A richer preview on hover or focus.", "悬停或聚焦时显示详细预览。"), hover_card_demo())
            demo(name: "dialogs_card", title: locale.select("Dialog and alert dialog", "对话框与确认对话框"), description: locale.select("Focused content and explicit confirmation.", "聚焦内容与明确的确认操作。"), dialogs_card())
            demo(name: "sheet_card", title: locale.select("Sheet", "侧边抽屉"), description: locale.select("An overlay panel from the page edge.", "从页面边缘滑出的叠层面板。"), sheet_card())
            demo(name: "deployments_card", title: locale.select("Table and pagination", "表格与分页"), description: locale.select("Structured rows with page controls.", "展示结构化数据与分页操作。"), deployments_card())
            demo(name: "breadcrumbs_card", title: locale.select("Breadcrumbs", "面包屑"), description: locale.select("A trail through related pages.", "展示页面的层级路径。"), breadcrumbs_card())
            demo(name: "keyboard_card", title: locale.select("Keyboard keys", "键盘按键"), description: locale.select("Single keys and key combinations.", "展示单个按键和组合键。"), keyboard_card())
            demo(name: "skeletons_card", title: locale.select("Skeletons", "骨架屏"), description: locale.select("Placeholders before content loads.", "内容加载前的占位状态。"), skeletons_card())
            demo(name: "spinner_card", title: locale.select("Spinner", "加载指示器"), description: locale.select("A live loading indicator.", "展示动态加载状态。"), spinner_card())
            demo(name: "sidebar_card", title: locale.select("Sidebar", "侧边栏"), description: locale.select("The official sidebar in a compact preview.", "在紧凑预览中体验官方侧边栏。"), sidebar_card())
        </div>
    })
}

/// Map every registry module to a live example on this page.
pub(in crate::app) fn registry_target(name: &str) -> &'static str {
    match name {
        "accordion" => "faq_card",
        "alert" => "notices",
        "alert-dialog" | "dialog" => "dialogs_card",
        "avatar" | "separator" => "team_card",
        "badge" => "status_card",
        "breadcrumb" => "breadcrumbs_card",
        "button" | "card" => "buttons_card",
        "checkbox" => "checks_card",
        "dropdown-menu" => "branches_card",
        "field" | "input" | "label" | "select" | "textarea" => "form_card",
        "hover-card" => "hover_card_demo",
        "kbd" => "keyboard_card",
        "pagination" | "table" => "deployments_card",
        "progress" => "progress_card",
        "radio-group" => "radios_card",
        "sheet" => "sheet_card",
        "sidebar" => "sidebar_card",
        "skeleton" => "skeletons_card",
        "spinner" => "spinner_card",
        "switch" => "switches_card",
        "tabs" => "overview_card",
        "toggle" => "toolbar_card",
        "tooltip" => "tooltip_card",
        _ => "components",
    }
}

/// Show the same preview and expandable source used by the project components.
#[component]
async fn demo(
    name: &str,
    title: &str,
    description: &str,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div id=(name) class="min-w-0 scroll-mt-24">
            component_example(id: name, title: title, description: description, source: example_source(name),
                <div class="native-demo-preview min-w-0 p-6">(child)</div>
            )
        </div>
    })
}

const NATIVE_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bin/gallery/app/_native.rs"
));

fn example_source(name: &str) -> &'static str {
    let function = format!("async fn {name}(");
    let function_start = NATIVE_SOURCE
        .find(&function)
        .expect("native demo source exists");
    let start = NATIVE_SOURCE[..function_start]
        .rfind("\n#[")
        .map_or(function_start, |index| index + 1);
    let end = NATIVE_SOURCE[function_start..]
        .find("\n}\n")
        .map_or(NATIVE_SOURCE.len(), |index| function_start + index + 2);
    &NATIVE_SOURCE[start..end]
}

fn native_text<'a>(locale: Locale, english: &'a str) -> &'a str {
    if locale == Locale::En {
        return english;
    }
    match english {
        "Buttons" => "按钮",
        "Every variant, size, and state." => "展示全部样式、尺寸和状态。",
        "Primary" => "主要",
        "Secondary" => "次要",
        "Outline" => "描边",
        "Ghost" => "幽灵",
        "Destructive" => "危险",
        "Small" => "小",
        "Medium" => "中",
        "Large" => "大",
        "Add item" => "添加项目",
        "Disabled" => "已禁用",
        "Saving..." => "保存中…",
        "Every control is a link or a form" => "每个控件都有清晰的语义",
        "The destructive variant" => "危险提示",
        "For what went wrong, and for what cannot be taken back." => {
            "用于需要特别注意的失败与不可撤销操作。"
        }
        "Without an icon" => "无图标提示",
        "Avatars" => "头像",
        "Owner" => "所有者",
        "Member" => "成员",
        "Viewer" => "查看者",
        "Badges" => "徽标",
        "Status badges in every variant." => "展示所有状态徽标样式。",
        "Built with Topcoat" => "由 Topcoat 构建",
        "Progress" => "进度条",
        "With a known value or an indeterminate amount of work." => "展示确定进度和不确定进度。",
        "A bar with a value" => "当前进度",
        "Reset" => "重置",
        "Advance" => "前进",
        "Indeterminate" => "不确定进度",
        "Form controls" => "表单控件",
        "Grouped fields, helpful descriptions, and inline errors." => {
            "组合字段、说明文字和行内错误。"
        }
        "Project details" => "项目详情",
        "Name" => "名称",
        "A name to identify this project." => "用于标识此项目的名称。",
        "Enter a project name." => "请输入项目名称。",
        "Region" => "区域",
        "Europe" => "欧洲",
        "Americas" => "美洲",
        "Choose the region closest to your users." => "请选择最靠近用户的区域。",
        "Summary" => "简介",
        "The workspace owner manages this project." => "工作区所有者管理此项目。",
        "Validate" => "验证",
        "Checkboxes" => "复选框",
        "Checked, unchecked, and disabled." => "展示选中、未选中和禁用状态。",
        "Switches" => "开关",
        "On, off, and disabled." => "展示开启、关闭和禁用状态。",
        "Radio group" => "单选组",
        "One choice at a time, with a disabled option." => "一次选择一项，并展示禁用选项。",
        "Tabs" => "标签页",
        "Switch between panels." => "切换不同内容面板。",
        "Recent activity appears here." => "这里展示最近的活动。",
        "Adjust your preferences here." => "在这里调整偏好设置。",
        "A quick overview of your project." => "这里是项目概览。",
        "Accordion" => "手风琴",
        "One section open at a time; the rest fold away." => "一次展开一项，其他内容自动收起。",
        "Where do the components live?" => "组件存放在哪里？",
        "Can I edit them?" => "可以编辑组件吗？",
        "How do updates work?" => "如何更新组件？",
        "Dropdown menu" => "下拉菜单",
        "Pick a branch or tag to update the selection." => "选择分支或标签并更新当前项。",
        "Switch branch" => "切换分支",
        "Checkout tag" => "检出标签",
        "Toggles" => "切换控件",
        "Day" => "日",
        "Week" => "周",
        "Month" => "月",
        "Bold" => "粗体",
        "Italic" => "斜体",
        "Underline" => "下划线",
        "Live updates" => "实时更新",
        "Tooltip" => "文字提示",
        "A short hint on hover or focus." => "悬停或聚焦时显示简短提示。",
        "Hover the button" => "悬停按钮",
        "Read the docs" => "阅读文档",
        "Hover card" => "悬停卡片",
        "A preview on hover or focus." => "悬停或聚焦时显示预览。",
        "Hover the name" => "悬停姓名",
        "Dialog" => "对话框",
        "A content panel or a confirmation prompt." => "展示内容面板或确认提示。",
        "Open dialog" => "打开对话框",
        "Open alert dialog" => "打开确认对话框",
        "Example dialog" => "示例对话框",
        "A dialog brings content into focus above the page." => "对话框将内容聚焦到页面上方。",
        "Put your content here, then close the dialog to return to the page." => {
            "在这里放置内容，关闭后返回页面。"
        }
        "Close" => "关闭",
        "Continue?" => "继续吗？",
        "An alert dialog asks for an explicit choice before continuing." => {
            "继续之前需要明确确认。"
        }
        "Cancel" => "取消",
        "Continue" => "继续",
        "Sheet" => "侧边抽屉",
        "A panel that slides in from the edge of the page." => "从页面边缘滑出的面板。",
        "Open sheet" => "打开抽屉",
        "Example sheet" => "示例抽屉",
        "Use a sheet for content that belongs beside the page." => {
            "适合展示与当前页面相关的辅助内容。"
        }
        "The rest of the page stays visible behind this panel." => "其他页面内容仍在面板后方可见。",
        "Table" => "表格",
        "Deployment rows with badges and pagination." => "用徽标和分页展示部署记录。",
        "Deployments" => "部署记录",
        "Commit" => "提交",
        "Environment" => "环境",
        "Status" => "状态",
        "Total" => "合计",
        "Breadcrumbs" => "面包屑",
        "A trail of links to the current page." => "展示到当前页面的路径。",
        "Docs" => "文档",
        "Components" => "组件",
        "Keyboard keys" => "键盘按键",
        "Keys shown individually or in a group." => "展示单个按键和组合键。",
        "Skeletons" => "骨架屏",
        "Placeholders while content loads." => "加载内容时显示占位。",
        "Show content" => "显示内容",
        "Show skeletons" => "显示骨架屏",
        "Spinner" => "加载指示器",
        "An indicator while work is in progress." => "任务进行时显示加载状态。",
        "Loading" => "加载中",
        "Complete" => "完成",
        "Finish loading" => "完成加载",
        "Load again" => "再次加载",
        "Checked" => "已选中",
        "Unchecked" => "未选中",
        "Checked and disabled" => "已选中且禁用",
        "Unchecked and disabled" => "未选中且禁用",
        "On" => "开启",
        "Off" => "关闭",
        "Off and disabled" => "关闭且禁用",
        "Option one" => "选项一",
        "Option two" => "选项二",
        "Option three (disabled)" => "选项三（禁用）",
        "Overview" => "概览",
        "Activity" => "活动",
        "Settings" => "设置",
        "Move to the next control" => "移动到下一个控件",
        "Move back to the one before" => "返回上一个控件",
        "Submit the form" => "提交表单",
        "Live" => "运行中",
        "Building" => "构建中",
        "Queued" => "排队中",
        "Failed" => "失败",
        "deployments" => "次部署",
        other => other,
    }
}

/// The button family: variants, sizes, and states at a glance.
#[component]
async fn buttons_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Buttons")))
                card_description((native_text(locale, "Every variant, size, and state.")))
            )
            card_content(
                <div class="flex flex-col gap-3">
                    <div class="flex flex-wrap items-center gap-2">
                        for (variant, name) in [
                            (ButtonVariant::Primary, (native_text(locale, "Primary"))),
                            (ButtonVariant::Secondary, (native_text(locale, "Secondary"))),
                            (ButtonVariant::Outline, (native_text(locale, "Outline"))),
                            (ButtonVariant::Ghost, (native_text(locale, "Ghost"))),
                            (ButtonVariant::Destructive, (native_text(locale, "Destructive"))),
                        ] {
                            button(size: ButtonSize::Sm, variant: variant, (name))
                        }
                    </div>
                    <div class="flex flex-wrap items-center gap-2">
                        button(size: ButtonSize::Sm, (native_text(locale, "Small")))
                        button(size: ButtonSize::Md, (native_text(locale, "Medium")))
                        button(size: ButtonSize::Lg, (native_text(locale, "Large")))
                        button(
                            size: ButtonSize::Icon,
                            variant: ButtonVariant::Outline,
                            icon(data: iconify_icon!("lucide:plus"), label: native_text(locale, "Add item"))
                        )
                    </div>
                    <div class="flex flex-wrap items-center gap-2">
                        button(
                            variant: ButtonVariant::Outline,
                            attrs: attributes! { disabled="" },
                            (native_text(locale, "Disabled"))
                        )
                        button(
                            attrs: attributes! { disabled="" },
                            spinner(label: native_text(locale, "Loading").to_owned())
                            (native_text(locale, "Saving..."))
                        )
                    </div>
                </div>
            )
        )
    })
}

/// Alerts displayed without a surrounding card.
#[component]
async fn notices(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        <div class="flex flex-col gap-3">
            // The leading icon is an ordinary child: the alert lays out a
            // column for it only when one is there.
            alert(
                icon(data: iconify_icon!("lucide:info"))
                alert_title((native_text(locale, "Every control is a link or a form")))
                alert_description(
                    (locale.select("This message explains a routine state without interrupting the workflow.", "这条信息说明常规状态，不会中断当前操作。"))
                )
            )
            alert(
                variant: AlertVariant::Destructive,
                icon(data: iconify_icon!("lucide:triangle-alert"))
                alert_title((native_text(locale, "The destructive variant")))
                alert_description(
                    (native_text(locale, "For what went wrong, and for what cannot be taken back."))
                )
            )
            alert(
                alert_title((native_text(locale, "Without an icon")))
                alert_description(
                    (locale.select("The text fills the available space when no icon is present.", "没有图标时，标题和正文会填满可用空间。"))
                )
            )
        </div>
    })
}

/// Example people with avatar initials and roles.
const MEMBERS: [(&str, &str, &str, &str); 3] = [
    ("Grace Hopper", "grace@example.com", "GH", "Member"),
    ("Alan Turing", "alan@example.com", "AT", "Member"),
    ("Katherine Johnson", "katherine@example.com", "KJ", "Viewer"),
];

/// A roster showing each person and their role.
#[component]
async fn team_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Avatars")))
                card_description(
                    "A portrait, initials where there is none, and the role \
                     each one reads in a badge."
                )
            )
            card_content(
                // The owner is the only one with a portrait; the others fall
                // back to their initials, which is also what shows while an
                // image is still loading.
                <div class="flex items-center gap-3">
                    avatar(
                        size: AvatarSize::Lg,
                        avatar_image(attrs: attributes! { src=(PORTRAIT) })
                        avatar_fallback("AL")
                    )
                    <div class="min-w-0 flex-1">
                        <p class="truncate text-sm font-medium">"Ada Lovelace"</p>
                        <p class="truncate text-xs text-muted-foreground">
                            "ada@example.com"
                        </p>
                    </div>
                    badge(variant: BadgeVariant::Secondary, (native_text(locale, "Owner")))
                </div>
                separator(attrs: attributes! { class="my-5" })
                <div class="flex flex-col gap-3">
                    for (name, email, initials, role) in MEMBERS {
                        <div class="flex items-center justify-between gap-3">
                            <div class="flex min-w-0 items-center gap-3">
                                avatar(size: AvatarSize::Sm, avatar_fallback((initials)))
                                <div class="min-w-0">
                                    <p class="truncate text-sm font-medium">(name)</p>
                                    <p class="truncate text-xs text-muted-foreground">
                                        (email)
                                    </p>
                                </div>
                            </div>
                            badge(variant: BadgeVariant::Outline, (native_text(locale, role)))
                        </div>
                    }
                </div>
            )
        )
    })
}

/// The badge variants with example deployment counts.
#[component]
async fn status_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    const CRATE: &str = "https://crates.io/crates/topcoat";

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Badges")))
                card_description((native_text(locale, "Status badges in every variant.")))
            )
            card_content(
                <div class="flex flex-col gap-3">
                    for (status, variant) in STATUSES {
                        let count = DEPLOYMENTS
                            .iter()
                            .filter(|(_, _, value)| *value == status)
                            .count();

                        <div class="flex items-center justify-between gap-4">
                            badge(variant: variant, (native_text(locale, status)))
                            <p class="text-sm text-muted-foreground">
                                (format!("{count} {}", native_text(locale, "deployments")))
                            </p>
                        </div>
                    }
                </div>
            )
            card_footer(
                <p class="text-sm text-muted-foreground">(native_text(locale, "Built with Topcoat"))</p>
                // Anything can borrow a badge's looks: `badge_variants`
                // returns the class string for a variant.
                <a href=(CRATE) class=(badge_variants(BadgeVariant::Outline))>
                    (format!("v{}", env!("CARGO_PKG_VERSION")))
                </a>
            )
        )
    })
}

/// Determinate and indeterminate progress bars.
#[component]
async fn progress_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let completed = signal(cx, || 62usize);

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Progress")))
                card_description(
                    (native_text(locale, "With a known value or an indeterminate amount of work."))
                )
            )
            card_content(
                <div class="flex flex-col gap-2">
                    <div class="flex items-center justify-between gap-4">
                        <p class="text-sm text-muted-foreground">
                            (native_text(locale, "A bar with a value"))
                        </p>
                        <p class="text-sm font-medium">
                            $(completed.get())
                            "%"
                        </p>
                    </div>
                    progress(
                        attrs: attributes! { aria-label="Rollout progress" :value=$(completed.get()) }
                    )
                    <div class="flex justify-end gap-2">
                        button(
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            attrs: attributes! { type="button" @click=$(|_e: Event| completed.set(0)) },
                            (native_text(locale, "Reset"))
                        )
                        button(
                            size: ButtonSize::Sm,
                            attrs: attributes! {
                                type="button"
                                :disabled=$(completed.get() >= 100)
                                @click=$(|_e: Event| {
                                    let next = completed.get() + 10;
                                    completed.set(if next > 100 { 100 } else { next });
                                })
                            },
                            (native_text(locale, "Advance"))
                        )
                    </div>
                </div>
                separator(attrs: attributes! { class="my-4" })
                <div class="flex flex-col gap-2">
                    <p class="text-sm text-muted-foreground">(native_text(locale, "Indeterminate"))</p>
                    progress(attrs: attributes! { aria-label="Indeterminate progress" })
                </div>
            )
        )
    })
}

/// Form fields with labels, descriptions, and inline validation.
///
/// The fields keep their values in signals. Reset restores their initial values.
#[component]
async fn form_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let required_name_label = native_text(locale, "Enter a project name.");
    let name = signal(cx, String::new);
    let region = signal(cx, || String::from("eu-central-1"));
    let summary = signal(cx, String::new);
    let validated = signal(cx, || false);
    let invalid = expr!(if validated.get() {
        name.get().trim().is_empty()
    } else {
        false
    });

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Form controls")))
                card_description(
                    (native_text(locale, "Grouped fields, helpful descriptions, and inline errors."))
                )
            )
            card_content(
                <form
                    class="flex flex-col gap-4"
                    @submit=$(|e: Event| {
                        e.prevent_default();
                        validated.set(true);
                    })
                    @reset=$(|e: Event| {
                        e.prevent_default();
                        name.set("".to_owned());
                        region.set("eu-central-1".to_owned());
                        summary.set("".to_owned());
                        validated.set(false);
                    })
                >
                    field_set(
                        field_legend(
                            variant: FieldLegendVariant::Label,
                            (native_text(locale, "Project details"))
                        )
                        field_group(
                            field(
                                field_label(
                                    attrs: attributes! { for="project-name" },
                                    (native_text(locale, "Name"))
                                )
                                input(
                                    attrs: attributes! {
                                        id="project-name"
                                        name="name"
                                        placeholder="my-app"
                                        aria-required="true"
                                        aria-describedby="project-name-description project-name-error"
                                        :aria-invalid=$(if invalid { "true" } else { "false" })
                                        :value=$(name.get())
                                        @input=$(|e: Event| name.set(e.target.value))
                                    }
                                )
                                field_description(
                                    attrs: attributes! { id="project-name-description" },
                                    (native_text(locale, "A name to identify this project."))
                                )
                                field_error(
                                    attrs: attributes! { id="project-name-error" :hidden=$(!invalid) },
                                    $(if invalid { required_name_label } else { "" })
                                )
                            )
                            field(
                                field_label(attrs: attributes! { for="region" }, (native_text(locale, "Region")))
                                select(
                                    attrs: attributes! {
                                        id="region"
                                        name="region"
                                        aria-describedby="region-description"
                                        :value=$(region.get())
                                        @change=$(|e: Event| region.set(e.target.value))
                                    },
                                    <optgroup label=(native_text(locale, "Europe"))>
                                        <legend>(native_text(locale, "Europe"))</legend>
                                        <option>"eu-central-1"</option>
                                        <option>"eu-west-2"</option>
                                    </optgroup>
                                    <optgroup label=(native_text(locale, "Americas"))>
                                        <legend>(native_text(locale, "Americas"))</legend>
                                        <option>"us-east-1"</option>
                                        <option>"sa-east-1"</option>
                                    </optgroup>
                                )
                                field_description(
                                    attrs: attributes! { id="region-description" },
                                    (native_text(locale, "Choose the region closest to your users."))
                                )
                            )
                            field(
                                field_label(
                                    attrs: attributes! { for="summary" },
                                    (native_text(locale, "Summary"))
                                )
                                textarea(
                                    attrs: attributes! {
                                        id="summary"
                                        name="summary"
                                        placeholder="What this project is for."
                                        :value=$(summary.get())
                                        @input=$(|e: Event| summary.set(e.target.value))
                                    }
                                )
                            )
                            field_separator()
                            field(
                                orientation: FieldOrientation::Responsive,
                                field_label(attrs: attributes! { for="owner" }, (native_text(locale, "Owner")))
                                // A disabled field shows a value that is not the
                                // form's to change.
                                field_content(
                                    input(
                                        attrs: attributes! {
                                            id="owner"
                                            value="ada@example.com"
                                            aria-describedby="owner-description"
                                            disabled=""
                                        }
                                    )
                                    field_description(
                                        attrs: attributes! { id="owner-description" },
                                        (native_text(locale, "The workspace owner manages this project."))
                                    )
                                )
                            )
                        )
                    )
                    <div class="flex flex-wrap justify-end gap-2">
                        button(
                            variant: ButtonVariant::Outline,
                            attrs: attributes! { type="reset" },
                            (native_text(locale, "Reset"))
                        )
                        button(attrs: attributes! { type="submit" }, (native_text(locale, "Validate")))
                    </div>
                </form>
            )
        )
    })
}

/// Checkboxes in their checked, unchecked, and disabled states.
///
/// Each control keeps its own signal, starting in the state its row names.
#[component]
async fn checks_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    // The id, label, checked state, and disabled state of each checkbox.
    const CHECKS: [(&str, &str, bool, bool); 4] = [
        ("check-on", "Checked", true, false),
        ("check-off", "Unchecked", false, false),
        ("check-on-off", "Checked and disabled", true, true),
        ("check-off-off", "Unchecked and disabled", false, true),
    ];

    let checks: Vec<_> = CHECKS
        .into_iter()
        .map(|(id, text, checked, disabled)| {
            (id, text, signal(&cx.keyed(id), || checked), disabled)
        })
        .collect();

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Checkboxes")))
                card_description((native_text(locale, "Checked, unchecked, and disabled.")))
            )
            card_content(
                field_group(
                    attrs: attributes! { class="gap-3" },
                    for (id, text, checked, disabled) in checks {
                        field(
                            orientation: FieldOrientation::Horizontal,
                            checkbox(
                                attrs: attributes! {
                                    id=(id)
                                    :checked=$(checked.get())
                                    @change=$(|e: Event| checked.set(e.target.checked))
                                    disabled=(disabled)
                                }
                            )
                            field_label(attrs: attributes! { for=(id) }, (native_text(locale, text)))
                        )
                    }
                )
            )
        )
    })
}

/// Switches with an independent signal for each control.
#[component]
async fn switches_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    // The id, label, checked state, and disabled state of each switch.
    const SWITCHES: [(&str, &str, bool, bool); 3] = [
        ("switch-on", "On", true, false),
        ("switch-off", "Off", false, false),
        ("switch-off-off", "Off and disabled", false, true),
    ];

    let switches: Vec<_> = SWITCHES
        .into_iter()
        .map(|(id, text, checked, disabled)| {
            (id, text, signal(&cx.keyed(id), || checked), disabled)
        })
        .collect();

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Switches")))
                card_description((native_text(locale, "On, off, and disabled.")))
            )
            card_content(
                field_group(
                    attrs: attributes! { class="gap-3" },
                    for (id, text, checked, disabled) in switches {
                        field(
                            orientation: FieldOrientation::Horizontal,
                            field_label(attrs: attributes! { for=(id) }, (native_text(locale, text)))
                            switch(
                                attrs: attributes! {
                                    id=(id)
                                    :checked=$(checked.get())
                                    @change=$(|e: Event| checked.set(e.target.checked))
                                    disabled=(disabled)
                                }
                            )
                        )
                    }
                )
            )
        )
    })
}

/// A standalone radio group whose selection is managed by the browser.
#[component]
async fn radios_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Radio group")))
                card_description((native_text(locale, "One choice at a time, with a disabled option.")))
            )
            card_content(
                radio_group(
                    attrs: attributes! { aria-label=(locale.select("Example options", "选项示例")) },
                    for (value, text, checked, disabled) in [
                        ("one", "Option one", true, false),
                        ("two", "Option two", false, false),
                        ("three", "Option three (disabled)", false, true),
                    ] {
                        let id = format!("radio-demo-{value}");

                        field(
                            orientation: FieldOrientation::Horizontal,
                            radio_group_item(
                                attrs: attributes! {
                                    id=(id.as_str())
                                    name="radio-demo"
                                    value=(value)
                                    checked=(checked)
                                    disabled=(disabled)
                                }
                            )
                            field_label(
                                attrs: attributes! { for=(id.as_str()) },
                                (native_text(locale, text))
                            )
                        )
                    }
                )
            )
        )
    })
}

/// A card that switches panels in the browser.
#[component]
async fn overview_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    // The tab values and labels. The first is selected when the page opens.
    const TABS: [(&str, &str); 3] = [
        ("overview", "Overview"),
        ("activity", "Activity"),
        ("settings", "Settings"),
    ];

    let selected = signal(cx, || TABS[0].0.to_owned());

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Tabs")))
                card_description((native_text(locale, "Switch between panels.")))
            )
            card_content(
                tabs(
                    tabs_list(
                        for (value, text) in TABS {
                            tabs_trigger(
                                active: $(selected.get() == value),
                                attrs: attributes! {
                                    href="#"
                                    @click=$(|e: Event| {
                                        e.prevent_default();
                                        selected.set(value.to_owned());
                                    })
                                },
                                (native_text(locale, text))
                            )
                        }
                    )
                    for (value, _) in TABS {
                        tabs_content(
                            attrs: attributes! { :hidden=$(selected.get() != value) },
                            <p class="text-sm text-muted-foreground">
                                (match value {
                                    "activity" => native_text(locale, "Recent activity appears here."),
                                    "settings" => native_text(locale, "Adjust your preferences here."),
                                    _ => native_text(locale, "A quick overview of your project."),
                                })
                            </p>
                        )
                    }
                )
            )
        )
    })
}

/// A FAQ whose answers fold away, one open at a time.
#[component]
async fn faq_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Accordion")))
                card_description((native_text(locale, "One section open at a time; the rest fold away.")))
            )
            card_content(
                accordion(
                    // The name the sections share is what closes the open one
                    // when another is opened.
                    for (question, answer, open) in [
                        (
                            (native_text(locale, "Where do the components live?")),
                            locale.select("In your project's component directory.", "位于项目中的组件目录。"),
                            true,
                        ),
                        (
                            (native_text(locale, "Can I edit them?")),
                            locale.select("Yes. You can change their styles and behavior like any other source file.", "可以。它们与其他源码一样可以修改样式和行为。"),
                            false,
                        ),
                        (
                            (native_text(locale, "How do updates work?")),
                            locale.select("Run `topcoat ui list` to see registry changes.", "运行 `topcoat ui list` 查看注册表变更。"),
                            false,
                        ),
                    ] {
                        accordion_item(
                            attrs: attributes! { name="faq" open=(open) },
                            accordion_trigger((question))
                            accordion_content((answer))
                        )
                    }
                )
            )
        )
    })
}

/// A branch switcher that updates its label and closes the menu locally.
#[component]
async fn branches_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    // The first branch is selected until another branch or tag is picked.
    const BRANCHES: [&str; 3] = ["main", "feature/showcase", "feature/dark-mode"];
    const TAGS: [&str; 3] = ["v1.2.0", "v1.1.0", "v1.0.0"];

    let selected = signal(cx, || BRANCHES[0].to_owned());
    let open = signal(cx, || false);
    let tags_open = signal(cx, || false);

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Dropdown menu")))
                card_description((native_text(locale, "Pick a branch or tag to update the selection.")))
            )
            card_content(
                dropdown_menu(
                    attrs: attributes! { :open=$(open.get()) },
                    // The trigger takes any content; this one borrows the
                    // outline button's looks and adds a flipping chevron.
                    dropdown_menu_trigger(
                        attrs: attributes! {
                            class=(button_variants(
                                ButtonVariant::Outline,
                                ButtonSize::Sm,
                            ))
                            @click=$(|e: Event| {
                                e.prevent_default();
                                open.toggle();
                                tags_open.set(false);
                            })
                        },
                        $(selected.get())
                        icon(
                            data: iconify_icon!("lucide:chevron-down"),
                            attrs: attributes! { class="transition-transform group-open:rotate-180" }
                        )
                    )
                    dropdown_menu_content(
                        dropdown_menu_label((native_text(locale, "Switch branch")))
                        for branch in BRANCHES {
                            dropdown_menu_item(
                                attrs: attributes! {
                                    type="button"
                                    @click=$(|_e: Event| {
                                        selected.set(branch.to_owned());
                                        open.set(false);
                                        tags_open.set(false);
                                    })
                                },
                                (branch)
                            )
                        }
                        dropdown_menu_separator()
                        // A submenu opens its own panel beside this row.
                        dropdown_menu_sub(
                            attrs: attributes! { :open=$(tags_open.get()) },
                            dropdown_menu_sub_trigger(
                                attrs: attributes! {
                                    @click=$(|e: Event| {
                                        e.prevent_default();
                                        tags_open.toggle();
                                    })
                                },
                                (native_text(locale, "Checkout tag"))
                            )
                            dropdown_menu_sub_content(
                                for tag in TAGS {
                                    dropdown_menu_item(
                                        attrs: attributes! {
                                            type="button"
                                            @click=$(|_e: Event| {
                                                selected.set(tag.to_owned());
                                                open.set(false);
                                                tags_open.set(false);
                                            })
                                        },
                                        (tag)
                                    )
                                }
                            )
                        )
                    )
                )
            )
        )
    })
}

/// Independent toggles and a group that allows one selection.
#[component]
async fn toolbar_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let range = signal(cx, || String::from("week"));
    let bold = signal(cx, || true);
    let italic = signal(cx, || false);
    let underline = signal(cx, || false);
    let live = signal(cx, || true);

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Toggles")))
                card_description(
                    "A segmented control that keeps one pressed, and toggles \
                     that press on their own."
                )
            )
            card_content(
                <div class="flex flex-col items-start gap-4">
                    // The groups of a toolbar stand apart with a rule
                    // between them, and the row's height is what gives the
                    // rule its own.
                    <div class="flex h-9 items-center gap-2">
                        toggle_group(
                            for (value, text) in [
                                ("day", (native_text(locale, "Day"))),
                                ("week", (native_text(locale, "Week"))),
                                ("month", (native_text(locale, "Month"))),
                            ] {
                                toggle(
                                    kind: ToggleKind::Exclusive,
                                    size: ToggleSize::Sm,
                                    attrs: attributes! {
                                        name="range"
                                        value=(value)
                                        :checked=$(range.get() == value)
                                        @change=$(|_e: Event| range.set(value.to_owned()))
                                    },
                                    (text)
                                )
                            }
                        )
                        separator(orientation: SeparatorOrientation::Vertical)
                        <div class="flex items-center gap-1">
                            for (name, data, text, pressed) in [
                                ("bold", iconify_icon!("lucide:bold"), (native_text(locale, "Bold")), &bold),
                                (
                                    "italic",
                                    iconify_icon!("lucide:italic"),
                                    (native_text(locale, "Italic")),
                                    &italic,
                                ),
                                (
                                    "underline",
                                    iconify_icon!("lucide:underline"),
                                    (native_text(locale, "Underline")),
                                    &underline,
                                ),
                            ] {
                                toggle(
                                    attrs: attributes! {
                                        name=(name)
                                        :checked=$(pressed.get())
                                        @change=$(|e: Event| pressed.set(e.target.checked))
                                    },
                                    icon(data: data, label: text)
                                )
                            }
                        </div>
                    </div>
                    toggle(
                        size: ToggleSize::Md,
                        attrs: attributes! {
                            name="live"
                            :checked=$(live.get())
                            @change=$(|e: Event| live.set(e.target.checked))
                        },
                        icon(data: iconify_icon!("lucide:activity"))
                        (native_text(locale, "Live updates"))
                    )
                </div>
            )
        )
    })
}

/// A short hint shown on hover or focus.
#[component]
async fn tooltip_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Tooltip")))
                card_description((native_text(locale, "A short hint on hover or focus.")))
            )
            card_content(
                <div class="flex items-center justify-between gap-4">
                    <p class="text-sm text-muted-foreground">(native_text(locale, "Hover the button"))</p>
                    tooltip(
                        <a
                            href=(DOCS)
                            class=(button_variants(
                                ButtonVariant::Outline,
                                ButtonSize::Icon,
                            ))
                        >
                            icon(
                                data: iconify_icon!("lucide:book-open"),
                                label: native_text(locale, "Read the docs")
                            )
                        </a>
                        tooltip_content((native_text(locale, "Read the docs")))
                    )
                </div>
            )
        )
    })
}

/// A preview with richer content, shown on hover or focus.
#[component]
async fn hover_card_demo(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Hover card")))
                card_description((native_text(locale, "A preview on hover or focus.")))
            )
            card_content(
                <div class="flex items-center gap-2 text-sm">
                    <p class="text-muted-foreground">(native_text(locale, "Hover the name"))</p>
                    hover_card(
                        // The trigger takes focus, so the card comes up for a
                        // reader on the keyboard as well.
                        <button type="button" class="font-medium underline">
                            "@ada"
                        </button>
                        hover_card_content(attrs: attributes! { class="max-[640px]:left-1/2 max-[640px]:-translate-x-1/2" },
                            <div class="flex items-center gap-3">
                                avatar(
                                    size: AvatarSize::Md,
                                    avatar_image(attrs: attributes! { src=(PORTRAIT) })
                                    avatar_fallback("AL")
                                )
                                <div class="min-w-0">
                                    <p class="truncate text-sm font-medium">"Ada Lovelace"</p>
                                    <p class="truncate text-xs text-muted-foreground">
                                        (native_text(locale, "Owner"))
                                    </p>
                                </div>
                            </div>
                            <p class="text-sm text-muted-foreground">
                                (locale.select("A hover card can show rich content and actions.", "悬停卡片可以展示更丰富的内容与操作。"))
                            </p>
                        )
                    )
                </div>
            )
        )
    })
}

/// A dialog and an alert dialog, each controlled by a local signal.
#[component]
async fn dialogs_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let open = signal(cx, || false);
    let confirming = signal(cx, || false);

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Dialog")))
                card_description((native_text(locale, "A content panel or a confirmation prompt.")))
            )
            card_content(
                <div class="flex flex-wrap gap-2">
                    button(
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        attrs: attributes! { type="button" @click=$(|_e: Event| open.set(true)) },
                        (native_text(locale, "Open dialog"))
                    )
                    button(
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        attrs: attributes! { type="button" @click=$(|_e: Event| confirming.set(true)) },
                        (native_text(locale, "Open alert dialog"))
                    )
                </div>
            )
        )
        dialog(
            open: $(open.get()),
            attrs: attributes! { aria-label=(native_text(locale, "Example dialog")) },
            dialog_content(
                dialog_header(
                    dialog_title((native_text(locale, "Example dialog")))
                    dialog_description(
                        (native_text(locale, "A dialog brings content into focus above the page."))
                    )
                )
                <p class="text-sm">
                    (native_text(locale, "Put your content here, then close the dialog to return to the page."))
                </p>
                dialog_footer(
                    button(
                        attrs: attributes! { type="button" @click=$(|_e: Event| open.set(false)) },
                        (native_text(locale, "Close"))
                    )
                )
            )
        )
        alert_dialog(
            open: $(confirming.get()),
            attrs: attributes! { aria-label=(native_text(locale, "Continue?")) },
            dialog_content(
                dialog_header(
                    dialog_title((native_text(locale, "Continue?")))
                    dialog_description(
                        (native_text(locale, "An alert dialog asks for an explicit choice before continuing."))
                    )
                )
                dialog_footer(
                    button(
                        variant: ButtonVariant::Outline,
                        attrs: attributes! {
                            type="button"
                            @click=$(|_e: Event| confirming.set(false))
                        },
                        (native_text(locale, "Cancel"))
                    )
                    button(
                        attrs: attributes! {
                            type="button"
                            @click=$(|_e: Event| confirming.set(false))
                        },
                        (native_text(locale, "Continue"))
                    )
                )
            )
        )
    })
}

/// A sheet controlled by a local signal.
#[component]
async fn sheet_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let open = signal(cx, || false);

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Sheet")))
                card_description((native_text(locale, "A panel that slides in from the edge of the page.")))
            )
            card_content(
                button(
                    variant: ButtonVariant::Outline,
                    attrs: attributes! { type="button" @click=$(|_e: Event| open.set(true)) },
                    (native_text(locale, "Open sheet"))
                )
            )
        )
        sheet(
            open: $(open.get()),
            attrs: attributes! { aria-label=(native_text(locale, "Example sheet")) },
            sheet_content(
                dialog_header(
                    dialog_title((native_text(locale, "Example sheet")))
                    dialog_description(
                        (native_text(locale, "Use a sheet for content that belongs beside the page."))
                    )
                )
                <p class="text-sm">
                    (native_text(locale, "The rest of the page stays visible behind this panel."))
                </p>
                dialog_footer(
                    attrs: attributes! { class="mt-auto" },
                    button(
                        attrs: attributes! { type="button" @click=$(|_e: Event| open.set(false)) },
                        (native_text(locale, "Close"))
                    )
                )
            )
        )
    })
}

/// Example deployments shown in the table.
const DEPLOYMENTS: [(&str, &str, &str); 12] = [
    ("a1b2c3d", "production", "Live"),
    ("9f8e7d6", "staging", "Building"),
    ("4c5b6a7", "preview", "Queued"),
    ("2e1d0c9", "preview", "Failed"),
    ("7b6a5f4", "production", "Live"),
    ("3d2c1b0", "staging", "Live"),
    ("8e7d6c5", "preview", "Queued"),
    ("1a0b9c8", "production", "Failed"),
    ("5c4b3a2", "staging", "Building"),
    ("0f9e8d7", "preview", "Live"),
    ("6a5b4c3", "production", "Queued"),
    ("d4c3b2a", "staging", "Failed"),
];

/// A table of deployments with pagination underneath.
#[shard]
async fn deployments_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    const PER_PAGE: usize = 3;

    let page = signal(cx, || 1usize);
    let rows = &DEPLOYMENTS;
    let pages = rows.len().div_ceil(PER_PAGE).max(1);
    // Clamp the requested page to the available rows.
    let current = page.get().clamp(1, pages);
    let previous = current.saturating_sub(1).max(1);
    let next = (current + 1).min(pages);

    Ok(view! {
        let shown = rows.chunks(PER_PAGE).nth(current - 1).unwrap_or_default();

        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Table")))
                card_description((native_text(locale, "Deployment rows with badges and pagination.")))
            )
            // The card pads its sections rather than itself, so the table can
            // span its full width; the table's own padding lines the cells up
            // with the sections above and below it.
            table(
                attrs: attributes! { class="px-3" },
                table_caption((native_text(locale, "Deployments")))
                table_header(
                    table_row(
                        table_head((native_text(locale, "Commit")))
                        table_head((native_text(locale, "Environment")))
                        table_head((native_text(locale, "Status")))
                    )
                )
                table_body(
                    for (commit, env, status) in shown.iter().copied() {
                        table_row(
                            table_cell(
                                attrs: attributes! { class="font-mono" },
                                (commit)
                            )
                            table_cell((env))
                            table_cell(badge(variant: status_variant(status), (native_text(locale, status))))
                        )
                    }
                )
                table_footer(
                    table_row(
                        table_cell(attrs: attributes! { colspan="2" }, (native_text(locale, "Total")))
                        table_cell((format!("{} {}", rows.len(), native_text(locale, "deployments"))))
                    )
                )
            )
            card_footer(
                attrs: attributes! { class="justify-center" },
                pagination(
                    pagination_content(
                        pagination_item(
                            pagination_previous(
                                label: locale.select("Previous", "上一页").to_owned(),
                                attrs: attributes! {
                                    href="#"
                                    @click=$(|e: Event| {
                                        e.prevent_default();
                                        page.set(previous);
                                    })
                                }
                            )
                        )
                        for number in 1..=pages {
                            if listed(number, current, pages) {
                                pagination_item(
                                    pagination_link(
                                        active: number == current,
                                        attrs: attributes! {
                                            href="#"
                                            @click=$(|e: Event| {
                                                e.prevent_default();
                                                page.set(number);
                                            })
                                        },
                                        (number)
                                    )
                                )
                            } else if listed(number - 1, current, pages) {
                                // The first page left out of a run stands for
                                // the whole run.
                                pagination_item(pagination_ellipsis(attrs: attributes! { aria-label=(locale.select("More pages", "更多页码")) }))
                            }
                        }
                        pagination_item(
                            pagination_next(
                                label: locale.select("Next", "下一页").to_owned(),
                                attrs: attributes! {
                                    href="#"
                                    @click=$(|e: Event| {
                                        e.prevent_default();
                                        page.set(next);
                                    })
                                }
                            )
                        )
                    )
                )
            )
        )
    })
}

/// Whether to show a numbered link for this page. Shows the first, last, and current
/// pages to keep the navigation compact.
fn listed(number: usize, page: usize, pages: usize) -> bool {
    number == 1 || number == pages || number == page
}

/// A breadcrumb trail with a collapsed middle section.
#[component]
async fn breadcrumbs_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Breadcrumbs")))
                card_description((native_text(locale, "A trail of links to the current page.")))
            )
            card_content(
                breadcrumb(
                    breadcrumb_list(
                        breadcrumb_item(
                            breadcrumb_link(attrs: attributes! { href=(DOCS) }, (native_text(locale, "Docs")))
                        )
                        breadcrumb_separator()
                        // The steps between are collapsed into an ellipsis.
                        breadcrumb_item(breadcrumb_ellipsis(attrs: attributes! { aria-label=(locale.select("More", "更多")) }))
                        breadcrumb_separator()
                        breadcrumb_item(
                            breadcrumb_link(
                                attrs: attributes! { href=(REGISTRY) },
                                (native_text(locale, "Components"))
                            )
                        )
                        breadcrumb_separator()
                        breadcrumb_item(breadcrumb_page((native_text(locale, "Breadcrumbs"))))
                    )
                )
            )
        )
    })
}

/// Individual keys and key combinations.
#[component]
async fn keyboard_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    // The browser's own keys for moving through a form.
    const KEYS: [(&str, &[&str]); 3] = [
        ("Move to the next control", &["Tab"]),
        ("Move back to the one before", &["Shift", "Tab"]),
        ("Submit the form", &["Enter"]),
    ];

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Keyboard keys")))
                card_description((native_text(locale, "Keys shown individually or in a group.")))
            )
            card_content(
                <div class="flex flex-col gap-3">
                    for (action, keys) in KEYS {
                        <div class="flex items-center justify-between gap-4">
                            <p class="text-sm text-muted-foreground">(native_text(locale, action))</p>
                            kbd_group(
                                for key in keys.iter().copied() {
                                    kbd((key))
                                }
                            )
                        </div>
                    }
                </div>
            )
        )
    })
}

/// Skeleton placeholders that can be replaced with loaded content.
#[component]
async fn skeletons_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let show_content_label = native_text(locale, "Show content");
    let show_skeletons_label = native_text(locale, "Show skeletons");
    let loading = signal(cx, || true);

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Skeletons")))
                card_description((native_text(locale, "Placeholders while content loads.")))
            )
            card_content(
                // The skeletons take the size of what they stand in for, so
                // the card keeps its height once the roster lands.
                <div class="flex flex-col gap-4" :hidden=$(!loading.get())>
                    for _ in 0..2 {
                        <div class="flex items-center gap-3">
                            skeleton(attrs: attributes! { class="size-8 rounded-full" })
                            <div class="flex flex-1 flex-col gap-1.5">
                                skeleton(attrs: attributes! { class="h-3.5 w-28" })
                                skeleton(attrs: attributes! { class="h-3 w-40" })
                            </div>
                            skeleton(attrs: attributes! { class="h-8 w-20 rounded-md" })
                        </div>
                    }
                </div>
                <div class="flex flex-col gap-4" :hidden=$(loading.get())>
                    for (name, email, initials, role) in MEMBERS.into_iter().take(2) {
                        <div class="flex items-center gap-3">
                            avatar(size: AvatarSize::Sm, avatar_fallback((initials)))
                            <div class="flex min-w-0 flex-1 flex-col gap-1.5">
                                <p class="truncate text-sm font-medium">(name)</p>
                                <p class="truncate text-xs text-muted-foreground">
                                    (email)
                                </p>
                            </div>
                            badge(variant: BadgeVariant::Outline, (native_text(locale, role)))
                        </div>
                    }
                </div>
            )
            card_footer(
                button(
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    attrs: attributes! { type="button" @click=$(|_e: Event| loading.toggle()) },
                    $(if loading.get() { show_content_label } else { show_skeletons_label })
                )
            )
        )
    })
}

/// A spinner accompanying a loading message.
#[component]
async fn spinner_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let loading_label = native_text(locale, "Loading");
    let complete_label = native_text(locale, "Complete");
    let finish_label = native_text(locale, "Finish loading");
    let again_label = native_text(locale, "Load again");
    let loading = signal(cx, || true);

    Ok(view! {
        card(attrs: attributes! { class="native-card" },
            card_header(
                card_title((native_text(locale, "Spinner")))
                card_description((native_text(locale, "An indicator while work is in progress.")))
            )
            card_content(
                <div class="flex flex-col gap-2">
                    <p class="flex items-center gap-1.5 text-sm text-muted-foreground">
                        <span class="contents" :hidden=$(!loading.get())>
                            spinner(label: loading_label.to_owned())
                        </span>
                        $(if loading.get() { loading_label } else { complete_label })
                    </p>
                    button(
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        attrs: attributes! {
                            type="button"
                            class="self-end"
                            @click=$(|_e: Event| loading.toggle())
                        },
                        $(if loading.get() { finish_label } else { again_label })
                    )
                </div>
            )
        )
    })
}

/// A compact instance of the official sidebar inside the shared Gallery.
#[component]
async fn sidebar_card(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let open = signal(cx, || true);
    let mobile_open = signal(cx, || false);

    Ok(view! {
        <div class="native-sidebar-demo overflow-hidden rounded-lg border border-border">
            sidebar_provider(
                attrs: attributes! { class="native-sidebar-provider" },
                sidebar(
                    open: $(open.get()),
                    mobile_open: $(mobile_open.get()),
                    variant: SidebarVariant::Sidebar,
                    collapsible: SidebarCollapsible::Offcanvas,
                    sheet_attrs: attributes! { id="native-sidebar-preview" aria-label=(locale.select("Example sidebar", "侧边栏示例")) },
                    sidebar_header(
                        <strong class="px-4 py-3 text-sm">(locale.select("Workspace", "工作区"))</strong>
                    )
                    sidebar_content(
                        sidebar_menu(
                            sidebar_menu_item(
                                sidebar_menu_button(href: Some("#native-registry-heading"),
                                    icon(data: iconify_icon!("lucide:layout-grid"))
                                    <span>(locale.select("Components", "组件"))</span>
                                )
                            )
                            sidebar_menu_item(
                                sidebar_menu_button(href: Some("#components"),
                                    icon(data: iconify_icon!("lucide:book-open"))
                                    <span>(locale.select("Examples", "示例"))</span>
                                )
                            )
                        )
                    )
                    sidebar_footer(
                        <span class="px-4 py-3 text-xs text-muted-foreground">"Topcoat UI"</span>
                    )
                )
                sidebar_inset(
                    sidebar_header(attrs: attributes! { class="[&]:h-12 [&]:px-3" },
                        sidebar_trigger(open: $(open.get()), attrs: attributes! { class="max-md:hidden" aria-label=(locale.select("Toggle sidebar", "切换侧边栏")) aria-controls="native-sidebar-preview" @click=$(|_e: Event| open.toggle()) })
                        sidebar_trigger(open: $(mobile_open.get()), attrs: attributes! { class="md:hidden" aria-label=(locale.select("Toggle sidebar", "切换侧边栏")) aria-controls="native-sidebar-preview" @click=$(|_e: Event| mobile_open.toggle()) })
                        <span class="text-xs font-semibold">(locale.select("Project view", "项目视图"))</span>
                    )
                    <p class="p-4 text-sm text-muted-foreground">(locale.select("The content fills the remaining width as the sidebar opens and closes.", "侧边栏展开或收起时，内容会自动填充剩余宽度。"))</p>
                )
            )
        </div>
    })
}
