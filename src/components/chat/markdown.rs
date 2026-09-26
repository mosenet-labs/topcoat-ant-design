use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, html};
use topcoat::{
    Result,
    view::{Unescaped, View, component, view},
};

/// Render untrusted chat Markdown with escaped raw HTML and safe link targets.
pub fn render_chat_markdown(source: &str) -> String {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let events = Parser::new_ext(source, options).filter_map(|event| match event {
        Event::Html(raw) | Event::InlineHtml(raw) => Some(Event::Text(raw)),
        Event::Start(Tag::Image { .. }) | Event::End(TagEnd::Image) => None,
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let url = dest_url.trim();
            let lower = url.to_ascii_lowercase();
            let safe = (lower.starts_with("https://")
                || lower.starts_with("http://")
                || lower.starts_with("mailto:")
                || lower.starts_with('#')
                || (lower.starts_with('/') && !lower.starts_with("//")))
                && !url.chars().any(char::is_control);
            Some(Event::Start(Tag::Link {
                link_type,
                dest_url: if safe {
                    dest_url
                } else {
                    CowStr::Borrowed("#")
                },
                title,
                id,
            }))
        }
        other => Some(other),
    });
    let mut rendered = String::new();
    html::push_html(&mut rendered, events);
    rendered
}

/// Message content renderer for Markdown supplied by a user or model.
#[component]
pub async fn chat_markdown(source: &str) -> Result<impl View> {
    let rendered = render_chat_markdown(source);
    Ok(view! { <div class="gr-chat-markdown">(Unescaped::new_unchecked(rendered))</div> })
}

#[cfg(test)]
mod tests {
    use super::render_chat_markdown;

    #[test]
    fn escapes_html_and_rejects_unsafe_urls() {
        let html = render_chat_markdown(
            "**hello** <script>alert(1)</script> [bad](javascript:alert(1)) [good](https://example.com) ![alt](https://example.com/a.png)",
        );
        assert!(html.contains("<strong>hello</strong>"));
        assert!(!html.contains("<script>"));
        assert!(!html.contains("javascript:"));
        assert!(html.contains("href=\"#\""));
        assert!(html.contains("href=\"https://example.com\""));
        assert!(!html.contains("<img"));
    }
}
