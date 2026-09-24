use pulldown_cmark::{Options, Parser, html};
use topcoat::{
    Result,
    view::{Unescaped, View, component, view},
};

const MISSING_RUST_EXAMPLE: &str = "// No Rust example is available in this document.";

/// 读取 Markdown 中第 `index` 个 Rust fenced code block。
pub(crate) fn rust_code_block(source: &str, index: usize) -> &str {
    let mut remaining = source;

    for current in 0..=index {
        let Some(opening) = remaining.find("```rust") else {
            return MISSING_RUST_EXAMPLE;
        };
        let after_opening = &remaining[opening..];
        let Some(line_end) = after_opening.find('\n') else {
            return MISSING_RUST_EXAMPLE;
        };
        let code = &after_opening[line_end + 1..];
        let Some(closing) = code.find("\n```") else {
            return MISSING_RUST_EXAMPLE;
        };

        if current == index {
            return code[..closing].trim_end();
        }
        remaining = &code[closing + 4..];
    }

    MISSING_RUST_EXAMPLE
}

#[component]
pub(crate) async fn markdown_document(source: &str) -> Result<impl View> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let mut rendered = String::new();
    html::push_html(&mut rendered, Parser::new_ext(source, options));

    // 内容只来自编译进二进制的仓库文档；这里不接受用户输入或外部 Markdown。
    Ok(view! {
        <article class="gallery-markdown">(Unescaped::new_unchecked(rendered))</article>
    })
}

#[cfg(test)]
mod tests {
    use super::rust_code_block;

    #[test]
    fn extracts_rust_examples_by_position() {
        let markdown = "before\n```rust,ignore\nlet first = 1;\n```\nmiddle\n```rust\nlet second = 2;\n```\nafter";

        assert_eq!(rust_code_block(markdown, 0), "let first = 1;");
        assert_eq!(rust_code_block(markdown, 1), "let second = 2;");
        assert!(rust_code_block(markdown, 2).contains("No Rust example"));
    }
}
