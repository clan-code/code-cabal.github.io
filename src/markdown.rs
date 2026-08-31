use pulldown_cmark::{Event, Options, Parser, html};

pub fn render_markdown(source: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(source, options).filter_map(|event| match event {
        // Content is versioned in this repository, but raw HTML stays disabled so a guide
        // cannot inject scripts or arbitrary markup by accident.
        Event::Html(_) | Event::InlineHtml(_) => None,
        other => Some(other),
    });

    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_is_rendered() {
        let html = render_markdown("## Hola\n\n**CODE**");
        assert!(html.contains("<h2>"));
        assert!(html.contains("<strong>CODE</strong>"));
    }

    #[test]
    fn raw_html_is_removed() {
        let html = render_markdown("Texto <script>alert('x')</script> seguro");
        assert!(!html.contains("<script>"));
    }
}
