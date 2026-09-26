use std::sync::atomic::{AtomicU64, Ordering};

use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, icon},
    runtime::{Event, Signal},
    view::{Attributes, View, attributes, class, component, view},
};

use crate::icons::{
    CHECK_CIRCLE_FILLED, CLOSE_CIRCLE_FILLED, CLOSE_OUTLINED, INFO_CIRCLE_FILLED, WARNING_FILLED,
};
use crate::{UiLanguage, ui::button};

/// Visual and accessibility tone of a notification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotificationTone {
    Success,
    Info,
    Warning,
    Error,
}

impl NotificationTone {
    const fn class_name(self) -> &'static str {
        match self {
            Self::Success => "gr-notification-success",
            Self::Info => "gr-notification-info",
            Self::Warning => "gr-notification-warning",
            Self::Error => "gr-notification-error",
        }
    }

    const fn icon_class(self) -> &'static str {
        match self {
            Self::Success => "bg-[var(--gr-success-soft)] text-[var(--gr-success)]",
            Self::Info => "bg-[var(--gr-accent-soft)] text-[var(--gr-accent)]",
            Self::Warning => "bg-[var(--gr-warning-soft)] text-[var(--gr-warning)]",
            Self::Error => "bg-[var(--gr-error-soft)] text-[var(--gr-error)]",
        }
    }

    const fn timer_class(self) -> &'static str {
        match self {
            Self::Success => "bg-[var(--gr-success)]",
            Self::Info => "bg-[var(--gr-accent)]",
            Self::Warning => "bg-[var(--gr-warning)]",
            Self::Error => "bg-[var(--gr-error)]",
        }
    }

    const fn icon_data(self) -> IconData {
        match self {
            Self::Success => CHECK_CIRCLE_FILLED,
            Self::Info => INFO_CIRCLE_FILLED,
            Self::Warning => WARNING_FILLED,
            Self::Error => CLOSE_CIRCLE_FILLED,
        }
    }

    const fn role(self) -> &'static str {
        match self {
            Self::Error => "alert",
            _ => "status",
        }
    }

    const fn live(self) -> &'static str {
        match self {
            Self::Error => "assertive",
            _ => "polite",
        }
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/notification.md"
))]
#[component]
pub async fn notification(
    cx: &Cx,
    message: &Signal<String>,
    title: &str,
    tone: NotificationTone,
    #[default] language: UiLanguage,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    static NEXT_NOTIFICATION_ID: AtomicU64 = AtomicU64::new(1);

    let popover_id = format!(
        "gr-notification-{}",
        NEXT_NOTIFICATION_ID.fetch_add(1, Ordering::Relaxed)
    );
    // Topcoat UI 组件允许调用方补充根元素属性，并合并而不是覆盖公共样式。
    let caller_class = attrs.remove("class");
    let notification_class = class!(
        "gr-notification group relative grid min-h-[92px] grid-cols-[32px_minmax(0,1fr)] gap-3 overflow-hidden rounded-lg border border-[var(--gr-border-subtle)] bg-[var(--gr-surface)] px-[18px] pb-4 pt-[18px] font-mono text-[var(--gr-fg)] shadow-lg pointer-events-auto animate-[gr-notification-enter_180ms_ease-out]",
        tone.class_name(),
        caller_class,
    );
    let icon_class = class!(
        "grid size-8 place-items-center rounded-full",
        tone.icon_class(),
    );
    let timer_class = class!(
        "gr-notification-timer absolute inset-x-0 bottom-0 h-0.5 origin-left opacity-30 animate-[gr-notification-timeout_4.5s_linear_forwards] group-hover:[animation-play-state:paused] group-focus-within:[animation-play-state:paused]",
        tone.timer_class(),
    );
    let semantics = attributes! { cx =>
        class=(notification_class)
        role=(tone.role())
        aria-live=(tone.live())
        aria-atomic="true"
    };
    // 组件语义属于公共契约，最后写入以避免调用方意外覆盖。
    attrs.extend(semantics);

    Ok(view! {
        <div id=(popover_id.as_str()) popover="manual" class="pointer-events-none fixed bottom-auto left-auto m-0 right-6 top-6 z-[1300] w-[min(400px,calc(100vw_-_32px))] overflow-visible border-0 bg-transparent p-0 max-[600px]:right-4 max-[600px]:top-4"
            :data-state=$(raw!("(() => { const visible = ${message}.get().dehydrate().length > 0; queueMicrotask(() => { const target = document.getElementById(${popover_id}.dehydrate()); if (!target) return; if (visible) { const modal = document.querySelector('dialog:modal'); if (modal && target.parentElement !== modal) modal.appendChild(target); if (!target.matches(':popover-open')) target.showPopover(); } else { if (target.matches(':popover-open')) target.hidePopover(); if (target.parentElement?.matches('dialog')) document.body.appendChild(target); } }); return visible ? 'open' : 'closed'; })()", if message.get_untracked().is_empty() { "closed" } else { "open" }))>
            <article (attrs)>
                <span class=(icon_class) aria-hidden="true">
                    icon(data: tone.icon_data(), size: 20)
                </span>
                <div class="min-w-0 pr-7">
                    <strong class="mb-1.5 mt-px block text-base font-semibold leading-[1.4]">(title)</strong>
                    <p class="m-0 [overflow-wrap:anywhere] text-sm leading-[1.6] text-[var(--gr-fg-muted)]">$(message.get())</p>
                </div>
                button::button(variant: button::ButtonVariant::Ghost, size: button::ButtonSize::Icon, attrs: attributes! { cx => class="absolute right-[18px] top-[18px] size-[22px] rounded-[4px] p-0 text-[var(--gr-fg-subtle)] hover:text-[var(--gr-fg)]" type="button" aria-label=(language.select("Close notification", "关闭通知")) @click=$(|_e: Event| message.set("".to_owned())) },
                    icon(data: CLOSE_OUTLINED, size: 12)
                )
                <span class=(timer_class) aria-hidden="true" @animationend=$(|_e| message.set("".to_owned()))></span>
            </article>
        </div>
    })
}
