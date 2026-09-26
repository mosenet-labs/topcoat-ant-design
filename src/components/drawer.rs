use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::{Event, Signal},
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::icons::CLOSE_OUTLINED;
use crate::{UiLanguage, native_ui::button};

/// Stable drawer ID, title, and optional close route.
#[derive(Clone, Copy)]
pub struct DrawerConfig<'a> {
    id: &'a str,
    title: &'a str,
    close_href: Option<&'a str>,
}

impl<'a> DrawerConfig<'a> {
    /// Create a drawer that closes through its local signal.
    pub const fn new(id: &'a str, title: &'a str) -> Self {
        Self {
            id,
            title,
            close_href: None,
        }
    }

    /// Navigate to a host-provided URL on close, suitable for query-driven detail views.
    pub const fn with_close_href(mut self, close_href: &'a str) -> Self {
        self.close_href = Some(close_href);
        self
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/drawer.md"
))]
#[component]
pub async fn drawer(
    cx: &Cx,
    config: DrawerConfig<'_>,
    open: &Signal<bool>,
    #[default] language: UiLanguage,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let DrawerConfig {
        id,
        title,
        close_href,
    } = config;
    let title_id = format!("{id}-title");
    let caller_class = attrs.remove("class");
    let root_class = class!("native-ui gr-drawer fixed inset-0 z-[1200]", caller_class,);
    let open = open.clone();
    let close_from_backdrop = open.clone();
    let close_from_button = open.clone();
    let close_on_escape = close_href.unwrap_or_default().to_owned();
    attrs.extend(attributes! { cx =>
        id=(id)
        class=(root_class)
        role="dialog"
        aria-modal="true"
        aria-labelledby=(title_id.as_str())
        :aria-hidden=$(if open.get() { "false" } else { "true" })
        :inert=$(!open.get())
        :data-state=$(if open.get() { "open" } else { "closed" })
        @keydown=$(|event: Event| {
            if event.key == "Escape" {
                if close_on_escape.is_empty() {
                    open.set(false);
                } else {
                    raw!("window.location.assign(${close_on_escape}.dehydrate())", ());
                }
            }
        })
    });

    Ok(view! {
        <section (attrs)>
            if let Some(close_href) = close_href {
                <a class="gr-drawer-mask absolute inset-0 cursor-default bg-black/45" href=(close_href) tabindex="-1" aria-label=(language.select("Close drawer", "关闭抽屉"))></a>
            } else {
                <button class="gr-drawer-mask absolute inset-0 cursor-default border-0 bg-black/45 p-0" type="button" tabindex="-1" aria-label=(language.select("Close drawer", "关闭抽屉")) @click=$(|_e| close_from_backdrop.set(false))></button>
            }
            <aside class="gr-drawer-panel absolute inset-y-0 right-0 flex w-[min(720px,100vw)] flex-col bg-[var(--gr-surface)] font-mono text-[var(--gr-fg)] shadow-[-8px_0_24px_var(--gr-shadow-color)]">
                <header class="flex min-h-16 shrink-0 items-center justify-between gap-4 border-b border-[var(--gr-border-subtle)] px-6 py-4">
                    <h2 class="m-0 min-w-0 text-lg font-semibold leading-7" id=(title_id.as_str())>(title)</h2>
                    if let Some(close_href) = close_href {
                        <a class="inline-flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-md text-[var(--gr-fg-subtle)] no-underline transition-colors duration-200 hover:bg-[var(--gr-surface-muted)] hover:text-[var(--gr-fg)] focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-[var(--gr-accent-border)]" href=(close_href) aria-label=(language.select("Close", "关闭"))>icon(data: CLOSE_OUTLINED, size: 16)</a>
                    } else {
                        button::button(variant: button::ButtonVariant::Ghost, size: button::ButtonSize::Icon, attrs: attributes! { cx => class="size-8 shrink-0 bg-transparent p-0 text-[var(--gr-fg-subtle)] hover:text-[var(--gr-fg)]" type="button" aria-label=(language.select("Close", "关闭")) @click=$(|_e: Event| close_from_button.set(false)) }, icon(data: CLOSE_OUTLINED, size: 16))
                    }
                </header>
                <div class="min-h-0 flex-1 overflow-y-auto px-6 py-5">(child)</div>
            </aside>
        </section>
    })
}
