use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn highlight_syntax(code: &str) -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    
    // Try to find a syntax for assembly, fallback to plain text
    let syntax = ps.find_syntax_by_extension("asm")
        .or_else(|| ps.find_syntax_by_extension("s"))
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    
    LinesWithEndings::from(code)
        .map(|line| {
            let highlights = h.highlight_line(line, &ps).unwrap_or_default();
            as_24_bit_terminal_escaped(&highlights[..], false)
        })
        .collect()
}