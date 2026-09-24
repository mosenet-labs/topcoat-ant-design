use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

/// 数据表格的显示密度。
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DataTableDensity {
    /// 适合包含说明文字或操作按钮的常规表格。
    #[default]
    Default,
    /// 适合事件、日志等需要在一屏展示更多记录的表格。
    Compact,
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/table.md"
))]
#[component]
pub async fn data_table(
    cx: &Cx,
    label: &str,
    #[default] density: DataTableDensity,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let table_class = class!(
        "gr-data-table",
        "gr-data-table-compact" if density == DataTableDensity::Compact,
        caller_class,
    );
    attrs.extend(attributes! { cx =>
        class=(table_class)
        aria-label=(label)
    });

    Ok(view! {
        <div class="gr-data-table-scroll">
            <table (attrs)>(child)</table>
        </div>
    })
}

/// 表格分页区域。调用方通过子内容提供真实链接或 Topcoat 交互按钮。
///
/// 完整参数、页码分页和游标分页示例见 [`data_table`] 的组件文档。
#[component]
pub async fn table_pagination(
    cx: &Cx,
    summary: &str,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let root_class = class!("gr-table-pagination", caller_class);
    attrs.extend(attributes! { cx =>
        class=(root_class)
    });

    Ok(view! {
        <footer (attrs)>
            <span class="gr-table-pagination-summary">(summary)</span>
            <nav aria-label=(label)>(child)</nav>
        </footer>
    })
}

/// 分页区域中的页容量选择器。
///
/// 组件只提供一致的标签、样式和无障碍属性。当前值、选项以及变更后的查询行为
/// 由调用方通过响应式属性和子内容控制。
#[component]
pub async fn table_page_size_select(
    cx: &Cx,
    id: &str,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let select_class = class!("gr-table-page-size-select", caller_class);
    attrs.extend(attributes! { cx =>
        id=(id)
        class=(select_class)
        aria-label=(label)
    });

    Ok(view! {
        <label class="gr-table-page-size" for=(id)>
            <span>(label)</span>
            <select (attrs)>(child)</select>
        </label>
    })
}
