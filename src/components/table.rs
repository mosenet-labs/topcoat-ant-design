use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::native_ui::{label, pagination, select, table};

/// Display density of a data table.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DataTableDensity {
    /// Standard table rows for supporting text or action buttons.
    #[default]
    Default,
    /// Compact rows for showing more events or logs at once.
    Compact,
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/table.md"
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
        "native-ui gr-data-table",
        "gr-data-table-compact" if density == DataTableDensity::Compact,
        caller_class,
    );
    attrs.extend(attributes! { cx =>
        class=(table_class)
        aria-label=(label)
    });

    Ok(view! { table::table(attrs: attrs, (child)) })
}

/// Table pagination area. The caller supplies real links or Topcoat buttons as children.
///
/// See [`data_table`] for complete parameters and numbered or cursor pagination examples.
#[component]
pub async fn table_pagination(
    cx: &Cx,
    summary: &str,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let root_class = class!("native-ui gr-table-pagination", caller_class);
    attrs.extend(attributes! { cx =>
        class=(root_class)
    });

    Ok(view! {
        <footer (attrs)>
            <span class="gr-table-pagination-summary">(summary)</span>
            pagination::pagination(attrs: attributes! { aria-label=(label) }, (child))
        </footer>
    })
}

/// Page-size selector for the pagination area.
///
/// The component provides consistent labels, styles, and accessibility attributes. The caller controls
/// the current value, options, and query behavior through reactive attributes and children.
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
        label::label(attrs: attributes! { class="gr-table-page-size native-ui" for=(id) },
            <span>(label)</span>
            select::select(attrs: attrs, (child))
        )
    })
}
