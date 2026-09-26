use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::{Event, Signal, expr},
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::UiLanguage;
use crate::icons::DOWN_OUTLINED;
use crate::native_ui::button;

/// Title, description, and optional badge for an accordion item.
#[derive(Clone, Copy)]
pub struct AccordionItemConfig<'a> {
    id: &'a str,
    title: &'a str,
    description: &'a str,
    badge: Option<&'a str>,
}

impl<'a> AccordionItemConfig<'a> {
    /// `id` must be unique on the page and must not come from unchecked user input.
    pub const fn new(id: &'a str, title: &'a str, description: &'a str) -> Self {
        Self {
            id,
            title,
            description,
            badge: None,
        }
    }

    /// Show a short badge, such as `OR`, beside the title.
    pub const fn with_badge(mut self, badge: &'a str) -> Self {
        self.badge = Some(badge);
        self
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/accordion.md"
))]
#[component]
pub async fn accordion_item(
    cx: &Cx,
    config: AccordionItemConfig<'_>,
    active: &Signal<String>,
    #[default] selected_count: Option<&Signal<f64>>,
    #[default] language: UiLanguage,
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
    let expanded = expr!(active.get() == active_id);
    let expanded_for_icon = expanded.clone();
    let expanded_for_state = expanded.clone();
    let expanded_for_hidden = expanded.clone();
    let count_suffix = language.select(" enabled", " 项已启用");
    let caller_class = attrs.remove("class");
    let root_class = class!(
        "gr-accordion-item native-ui overflow-hidden rounded-lg border border-[var(--gr-border)] bg-[var(--gr-surface)]",
        caller_class,
    );
    attrs.extend(attributes! { cx => class=(root_class) });

    Ok(view! {
        <section (attrs)>
            <h3 class="m-0">
                button::button(variant: button::ButtonVariant::Ghost, attrs: attributes! { cx => id=(trigger_id.as_str()) type="button" class="group flex h-auto w-full justify-start gap-3 rounded-none border-0 bg-[var(--gr-surface-muted)] px-4 py-3 text-left font-mono text-[var(--gr-fg)] hover:bg-[var(--gr-surface-muted)]"
                    aria-controls=(id)
                    :aria-expanded=$(if expanded { "true" } else { "false" })
                    @click=$(|_event: Event| {
                        if current.get() == active_id {
                            current.set("".to_owned());
                        } else {
                            current.set(active_id.to_owned());
                        }
                    }) },
                    <span class="min-w-0 flex-1">
                        <span class="block text-sm font-semibold leading-5">(title)</span>
                        <span class="mt-1 block text-xs font-normal leading-[1.5] text-[var(--gr-fg-subtle)]">(description)</span>
                    </span>
                    if let Some(count) = selected_count {
                        <span class="shrink-0 text-xs font-normal text-[var(--gr-fg-muted)]">$(count.get()) (count_suffix)</span>
                    }
                    if let Some(badge) = badge {
                        <span class="shrink-0 rounded bg-[var(--gr-accent-soft)] px-[7px] py-[2px] text-[11px] font-semibold text-[var(--gr-accent-strong)]">(badge)</span>
                    }
                    icon(data: DOWN_OUTLINED, size: 14, attrs: attributes! { cx =>
                        :class=$(if expanded_for_icon {
                            "shrink-0 rotate-180 text-[var(--gr-fg-subtle)] transition-transform duration-200"
                        } else {
                            "shrink-0 text-[var(--gr-fg-subtle)] transition-transform duration-200"
                        })
                    })
                )
            </h3>
            <div id=(id) class="gr-collapse" aria-labelledby=(trigger_id.as_str())
                :data-state=$(if expanded_for_state { "open" } else { "closed" })
                :aria-hidden=$(if expanded_for_hidden { "false" } else { "true" })>
                <div class="gr-collapse-inner border-t border-[var(--gr-border-subtle)]">(child)</div>
            </div>
        </section>
    })
}
