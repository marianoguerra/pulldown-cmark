use pulldown_cmark::{Options, Parser};

use crate::normalize_html;

pub fn test_markdown_html(input: &str, output: &str, slack_dialect: bool) {
    let mut s = String::new();

    let mut opts = Options::empty();

    if slack_dialect {
        opts.insert(Options::ENABLE_SINGLE_EMPHASIS_IS_STRONG);
    }

    let p = Parser::new_ext(input, opts);

    for event in p {
        println!("{event:?}");
    }

    let p = Parser::new_ext(input, opts);
    pulldown_cmark::html::push_html(&mut s, p);

    assert_eq!(normalize_html(output), normalize_html(&s));
}

#[test]
fn default_is_emphasis() {
    let original = "*hi*";
    let expected = "<p><em>hi</em></p>";

    test_markdown_html(original, expected, false);
}

#[test]
fn with_option_is_strong() {
    let original = "*hi*";
    let expected = "<p><strong>hi</strong></p>";

    test_markdown_html(original, expected, true);
}

#[test]
fn underline_with_option_is_emphasis() {
    let original = "_hi_";
    let expected = "<p><em>hi</em></p>";

    test_markdown_html(original, expected, true);
    test_markdown_html(original, expected, false);
}

#[test]
fn autolink_no_label() {
    let original = "<http://example.com>";
    let expected = r###"<p><a href="http://example.com">http://example.com</a></p>"###;

    test_markdown_html(original, expected, true);
    test_markdown_html(original, expected, false);
}

fn test_title_link(url: &str, title: &str) {
    let original = format!("<{url}|{title}>");
    let expected = format!("<p><a href=\"{url}\" title=\"{title}\">{url}|{title}</a></p>");

    test_markdown_html(&original, &expected, true);
}

#[test]
fn autolink_with_label() {
    test_title_link("http://example.com", "The Example");
    test_title_link("mailto:bob@example.com", "The Example");
    test_title_link("http://example.com", "ab \t -123");
    test_title_link("http://example.com", "ab \t 123!?.;,#~´`'asd'");
}
