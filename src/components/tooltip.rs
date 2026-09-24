use topcoat::{
    Result,
    runtime::Event,
    view::{Child, View, component, view},
};

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/components/tooltip.md"))]
#[component]
pub async fn tooltip(id: &str, content: &str, child: Child<'_>) -> Result<impl View> {
    let hover_id = id.to_owned();
    let focus_id = id.to_owned();
    let positioning_id = id.to_owned();
    Ok(view! {
        <span class="gr-tooltip-trigger" tabindex="0" aria-describedby=(id)
            @mouseenter=$(move |_e: Event| {
                let _id = hover_id.to_owned();
                raw!("document.getElementById(${_id}.dehydrate())?.showPopover()", ());
            })
            @focusin=$(move |_e: Event| {
                let _id = focus_id.to_owned();
                raw!("document.getElementById(${_id}.dehydrate())?.showPopover()", ());
            })>
            (child)
            <span id=(id) class="gr-tooltip" role="tooltip" popover="manual"
                @toggle=$(move |_e: Event| {
                    let _id = positioning_id.to_owned();
                    raw!(r#"(() => {
                        const panel = document.getElementById(${_id}.dehydrate());
                        if (!panel) return;
                        panel.grTooltipCleanup?.();
                        if (!panel.matches(':popover-open')) return;
                        const trigger = panel.parentElement;
                        document.querySelectorAll('.gr-tooltip:popover-open').forEach(other => {
                            if (other !== panel) other.hidePopover();
                        });
                        let timer;
                        let frame;
                        const keep = () => clearTimeout(timer);
                        const leave = () => {
                            keep();
                            timer = setTimeout(() => {
                                if (!trigger.matches(':hover, :focus-within') && !panel.matches(':hover')) panel.hidePopover();
                            }, 120);
                        };
                        const escape = event => {
                            if (event.key === 'Escape') panel.hidePopover();
                        };
                        const position = () => {
                            if (!panel.isConnected || !panel.matches(':popover-open')) {
                                panel.grTooltipCleanup?.();
                                return;
                            }
                            const anchor = trigger.getBoundingClientRect();
                            const rect = panel.getBoundingClientRect();
                            const width = document.documentElement.clientWidth;
                            const height = document.documentElement.clientHeight;
                            if (anchor.bottom <= 0 || anchor.top >= height || anchor.right <= 0 || anchor.left >= width) {
                                panel.hidePopover();
                                return;
                            }
                            const above = anchor.top >= rect.height + 16;
                            const left = Math.max(8, Math.min(width - rect.width - 8, anchor.left + anchor.width / 2 - rect.width / 2));
                            const top = Math.max(8, Math.min(height - rect.height - 8, above ? anchor.top - rect.height - 8 : anchor.bottom + 8));
                            panel.style.left = left + 'px';
                            panel.style.top = top + 'px';
                            panel.style.setProperty('--gr-tooltip-arrow-x', Math.max(12, Math.min(rect.width - 12, anchor.left + anchor.width / 2 - left)) + 'px');
                            panel.dataset.placement = above ? 'top' : 'bottom';
                            frame = requestAnimationFrame(position);
                        };
                        trigger.addEventListener('mouseleave', leave);
                        trigger.addEventListener('focusout', leave);
                        panel.addEventListener('mouseenter', keep);
                        panel.addEventListener('mouseleave', leave);
                        document.addEventListener('keydown', escape);
                        panel.grTooltipCleanup = () => {
                            clearTimeout(timer);
                            cancelAnimationFrame(frame);
                            trigger.removeEventListener('mouseleave', leave);
                            trigger.removeEventListener('focusout', leave);
                            panel.removeEventListener('mouseenter', keep);
                            panel.removeEventListener('mouseleave', leave);
                            document.removeEventListener('keydown', escape);
                            delete panel.grTooltipCleanup;
                        };
                        position();
                    })()"#, ());
                })>
                <span class="gr-tooltip-arrow" aria-hidden="true"></span>
                (content)
            </span>
        </span>
    })
}
