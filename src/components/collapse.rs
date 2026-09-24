use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal},
    view::{Attributes, Child, View, attributes, class, component, view},
};

/// 生成 Collapse 触发控件需要的 Topcoat 响应式属性。
///
/// 调用方仍负责提供 `type="button"`、样式和按钮内容。
pub fn collapse_trigger_attributes(cx: &Cx, id: &str, open: &Signal<bool>) -> Attributes {
    let open = open.clone();
    attributes! { cx =>
        aria-controls=(id)
        :aria-expanded=$(if open.get() { "true" } else { "false" })
        @click=$(|_e: Event| open.toggle())
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/collapse.md"
))]
#[component]
pub async fn collapse(
    cx: &Cx,
    id: &str,
    open: &Signal<bool>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let root_class = class!("gr-collapse", caller_class);
    let open = open.clone();
    let state = attributes! { cx =>
        id=(id)
        class=(root_class)
        :data-state=$(if open.get() { "open" } else { "closed" })
        :aria-hidden=$(if open.get() { "false" } else { "true" })
    };
    attrs.extend(state);

    Ok(view! {
        <div (attrs)>
            <div class="gr-collapse-inner">(child)</div>
        </div>
    })
}
