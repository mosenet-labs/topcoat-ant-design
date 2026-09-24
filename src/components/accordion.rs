use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::{Event, Signal},
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::icons::DOWN_OUTLINED;

/// 一个手风琴条目的标题、说明和可选标记。
#[derive(Clone, Copy)]
pub struct AccordionItemConfig<'a> {
    id: &'a str,
    title: &'a str,
    description: &'a str,
    badge: Option<&'a str>,
}

impl<'a> AccordionItemConfig<'a> {
    /// `id` 必须在当前页面中唯一，且不能来自未经校验的用户输入。
    pub const fn new(id: &'a str, title: &'a str, description: &'a str) -> Self {
        Self {
            id,
            title,
            description,
            badge: None,
        }
    }

    /// 在标题右侧显示简短标记，例如 `OR`。
    pub const fn with_badge(mut self, badge: &'a str) -> Self {
        self.badge = Some(badge);
        self
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/accordion.md"
))]
#[component]
pub async fn accordion_item(
    cx: &Cx,
    config: AccordionItemConfig<'_>,
    active: &Signal<String>,
    #[default] selected_count: Option<&Signal<f64>>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let AccordionItemConfig {
        id,
        title,
        description,
        badge,
    } = config;
    let trigger_id = format!("{id}-trigger");
    let active_id = id.to_owned();
    let current = active.clone();
    let caller_class = attrs.remove("class");
    let root_class = class!(
        "gr-accordion-item overflow-hidden rounded-lg border border-[#d9d9d9] bg-white",
        caller_class,
    );
    attrs.extend(attributes! { cx => class=(root_class) });

    Ok(view! {
        <section (attrs)>
            <h3 class="m-0">
                <button id=(trigger_id.as_str()) type="button" class="group flex w-full cursor-pointer items-center gap-3 border-0 bg-[#fafafa] px-4 py-3 text-left font-mono text-[#262626] transition-colors duration-150 hover:bg-[#f5f7fa] focus-visible:relative focus-visible:z-10 focus-visible:outline-2 focus-visible:outline-[#91caff]"
                    aria-controls=(id)
                    :aria-expanded=$(if active.get() == active_id { "true" } else { "false" })
                    @click=$(|_event: Event| {
                        if current.get() == active_id {
                            current.set("".to_owned());
                        } else {
                            current.set(active_id.to_owned());
                        }
                    })>
                    <span class="min-w-0 flex-1">
                        <span class="block text-sm font-semibold leading-5">(title)</span>
                        <span class="mt-1 block text-xs font-normal leading-[1.5] text-[#8c8c8c]">(description)</span>
                    </span>
                    if let Some(count) = selected_count {
                        <span class="shrink-0 text-xs font-normal text-[#595959]">$(count.get()) " 项已启用"</span>
                    }
                    if let Some(badge) = badge {
                        <span class="shrink-0 rounded bg-[#e6f4ff] px-[7px] py-[2px] text-[11px] font-semibold text-[#0958d9]">(badge)</span>
                    }
                    icon(data: DOWN_OUTLINED, size: 14, attrs: attributes! { cx =>
                        :class=$(if active.get() == active_id {
                            "shrink-0 rotate-180 text-[#8c8c8c] transition-transform duration-200"
                        } else {
                            "shrink-0 text-[#8c8c8c] transition-transform duration-200"
                        })
                    })
                </button>
            </h3>
            <div id=(id) class="gr-collapse" aria-labelledby=(trigger_id.as_str())
                :data-state=$(if active.get() == active_id { "open" } else { "closed" })
                :aria-hidden=$(if active.get() == active_id { "false" } else { "true" })>
                <div class="gr-collapse-inner border-t border-[#f0f0f0]">(child)</div>
            </div>
        </section>
    })
}
