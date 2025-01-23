use crate::{
    expressions::expression, identifiers::identifier, whitespace::skip_whitespace, MarkClosed,
    Parser,
};
use tantalum_source::SourceSpan;
use tantalum_syntax::{SyntaxKind, SyntaxToken};

pub fn primary(p: &mut Parser) -> MarkClosed {
    if p.is_at(SyntaxKind::Identifier) {
        let open = p.open(SyntaxKind::Variable);
        let _ = identifier(p).unwrap();
        p.close(open)
    } else if p.is_at(SyntaxKind::Digit) {
        let open = p.open(SyntaxKind::Literal);
        let _ = numeric_literal(p);
        p.close(open)
    } else if p.is_at_any(&[SyntaxKind::True, SyntaxKind::False]) {
        let open = p.open(SyntaxKind::Literal);
        let _ = p.consume();
        p.close(open)
    } else if p.is_at(SyntaxKind::LParen) {
        let open = p.open(SyntaxKind::Grouping);
        let _ = p.consume();
        skip_whitespace(p);
        let _ = expression(p);
        skip_whitespace(p);
        let _ = p.consume_if(SyntaxKind::RParen);
        p.close(open)
    } else {
        let open = p.open(SyntaxKind::Error);
        eprintln!("error: expected primary expression");
        let _ = p.consume();
        p.close(open)
    }
}

pub fn numeric_literal(p: &mut Parser) -> MarkClosed {
    let first = p.advance_while(|kind| kind == SyntaxKind::Digit);

    if first.is_empty() {
        let open = p.open(SyntaxKind::Error);
        eprintln!("error: expected digit");
        let _ = p.consume();
        return p.close(open);
    }

    if !p.is_at(SyntaxKind::Dot)
        || p.peek(2)
            .is_none_or(|token| token.kind() == SyntaxKind::Digit)
    {
        return p.token(SyntaxToken::new(
            SyntaxKind::IntegerLiteral,
            SourceSpan::merge(&[first.first().unwrap().span(), first.last().unwrap().span()]),
        ));
    }

    let second = p.advance_while(|kind| kind == SyntaxKind::Digit);

    p.token(SyntaxToken::new(
        SyntaxKind::FloatLiteral,
        SourceSpan::merge(&[first.first().unwrap().span(), second.last().unwrap().span()]),
    ))
}
