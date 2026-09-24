use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::{Event, Signal},
    view::{Attributes, Child, View, ViewExt, attributes, class, component, view},
};

use crate::icons::CLOSE_OUTLINED;

/// Dialog 的稳定标识、标题和可选眉题。
#[derive(Clone, Copy)]
pub struct DialogConfig<'a> {
    id: &'a str,
    title: &'a str,
    eyebrow: Option<&'a str>,
}

impl<'a> DialogConfig<'a> {
    /// 创建 Dialog 配置。
    pub const fn new(id: &'a str, title: &'a str) -> Self {
        Self {
            id,
            title,
            eyebrow: None,
        }
    }

    /// 在标题上方显示简短的场景标识。
    pub const fn with_eyebrow(mut self, eyebrow: &'a str) -> Self {
        self.eyebrow = Some(eyebrow);
        self
    }
}

/// 生成打开原生模态 Dialog 所需的按钮属性。
///
/// `id` 必须是由调用方生成的可信 DOM 标识，不能直接使用未经校验的用户输入。
pub fn dialog_trigger_attributes(cx: &Cx, id: &str) -> Attributes {
    attributes! { cx =>
        commandfor=(id)
        command="show-modal"
        aria-haspopup="dialog"
        aria-controls=(id)
    }
}

/// 生成关闭指定 Dialog 所需的按钮属性。
pub fn dialog_close_attributes(cx: &Cx, id: &str) -> Attributes {
    attributes! { cx =>
        commandfor=(id)
        command="close"
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/dialog.md"
))]
#[component]
pub async fn dialog(
    cx: &Cx,
    config: DialogConfig<'_>,
    busy: &Signal<bool>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let DialogConfig { id, title, eyebrow } = config;
    let title_id = format!("{id}-title");
    let close = dialog_close_attributes(cx, id);
    let prevent_cancel = busy.clone();
    let caller_class = attrs.remove("class");
    let root_class = class!(
        "gr-dialog m-auto max-h-[calc(100dvh_-_48px)] w-[min(580px,calc(100%_-_32px))] overflow-hidden rounded-[10px] border border-[#f0f0f0] bg-white p-0 text-left font-mono text-[#262626] shadow-[0_18px_60px_rgba(0,0,0,0.2)]",
        caller_class,
    );
    attrs.extend(attributes! { cx =>
        id=(id)
        class=(root_class)
        role="dialog"
        aria-modal="true"
        aria-labelledby=(title_id.as_str())
        @cancel=$(move |event: Event| {
            if prevent_cancel.get() {
                event.prevent_default();
            }
        })
    });

    // 缩小父级 ThenView 的状态，避免嵌套弹窗渲染时产生大型栈临时值。
    Ok(view! {
        <dialog (attrs)>
            <header class="gr-dialog-header flex shrink-0 items-start justify-between gap-4 border-b border-[#f0f0f0] px-6 pb-[18px] pt-[22px] max-[640px]:px-[18px]">
                <div class="min-w-0">
                    if let Some(eyebrow) = eyebrow {
                        <p class="m-0 text-xs font-bold tracking-[0.08em] text-[#8c8c8c]">(eyebrow)</p>
                    }
                    <h2 class="mb-0 mt-1 text-xl font-semibold leading-7" id=(title_id.as_str())>(title)</h2>
                </div>
                <button class="inline-flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-md border-0 bg-transparent p-0 text-[rgba(0,0,0,0.45)] transition-colors duration-200 hover:bg-[rgba(0,0,0,0.06)] hover:text-[rgba(0,0,0,0.88)] focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-[#91caff]" type="button" (close) :disabled=$(busy.get()) aria-label="关闭">
                    icon(data: CLOSE_OUTLINED, size: 16)
                </button>
            </header>
            (child)
        </dialog>
    }.boxed())
}
