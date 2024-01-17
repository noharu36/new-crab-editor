use syntect::highlighting::{RangedHighlightIterator, HighlightState, Highlighter, ThemeSet, Style};
use syntect::parsing::ScopeStack;
use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightLines;
use syntect::parsing::ScopeStackOp;
use syntect::util::as_24_bit_terminal_escaped;

pub fn highlighting(text: &str) -> Option<String> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ss.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    let mut state = HighlightState::new(&Highlighter::new(&ts.themes["base16-ocean.dark"]), ScopeStack::new());
    let binding = Highlighter::new(&ts.themes["base16-ocean.dark"]);
    let mut iter = RangedHighlightIterator::new(&mut state, &[(0, ScopeStackOp::Noop)], text,  &binding);

    while let Some((_, text, _)) = iter.next() {
        let ranges: Vec<(Style, &str)> = h.highlight_line(text, &ss).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges, false);
        return Some(escaped)
    }

    None
}
