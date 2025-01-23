use crate::Parser;
use tantalum_source::SourceSpan;
use tantalum_syntax::{SyntaxKind, SyntaxToken};

pub fn skip_whitespace(p: &mut Parser) {
    let tokens = p.advance_while(|kind| kind == SyntaxKind::Whitespace);

    if tokens.is_empty() {
        return;
    }

    p.token(SyntaxToken::new(
        SyntaxKind::Whitespace,
        SourceSpan::merge(
            tokens
                .iter()
                .map(SyntaxToken::span)
                .collect::<Vec<_>>()
                .as_slice(),
        ),
    ));
}
