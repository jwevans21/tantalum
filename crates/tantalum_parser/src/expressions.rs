//! Handles the parsing of expressions in the Tantalum language.
//!
//! Uses Pratt parsing to handle operator precedence and associativity.
//!
//! Based on [Simple but Powerful Pratt Parsing] which is licensed under the [MIT OR Apache-2.0]
//! license.
//!
//! [Simple but Powerful Pratt Parsing]: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
//! [MIT OR Apache-2.0]: https://matklad.github.io/about.html

use crate::{
    expressions::infix::{consume_infix_op, infix_op, infix_op_bp},
    expressions::postfix::{consume_postfix_op, postfix_op, postfix_op_bp},
    expressions::prefix::{consume_prefix_op, prefix_op, prefix_op_bp},
    expressions::primary::primary,
    statements::statement,
    whitespace::skip_whitespace,
    MarkClosed, Parser,
};
use tantalum_syntax::SyntaxKind;

mod infix;
mod postfix;
mod prefix;
mod primary;

/// Parse an expression
pub fn expression(p: &mut Parser) -> MarkClosed {
    if p.is_at(SyntaxKind::LBrace) {
        block(p)
    } else if p.is_at(SyntaxKind::If) {
        if_expression(p)
    } else {
        expression_bp(p, 0)
    }
}

/// Parses a block expression (a sequence of statements)
pub fn block(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::Block);

    let Some(_) = p.consume_if(SyntaxKind::LBrace) else {
        eprintln!("error: expected '{{'");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };
    skip_whitespace(p);

    while !p.is_at(SyntaxKind::RBrace) {
        statement(p);
        skip_whitespace(p);
    }

    let Some(_) = p.consume_if(SyntaxKind::RBrace) else {
        eprintln!("error: expected '}}'");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };

    p.close(open)
}

fn if_expression(p: &mut Parser) -> MarkClosed {
    let open = p.open(SyntaxKind::IfExpression);

    let Some(_) = p.consume_if(SyntaxKind::If) else {
        eprintln!("error: expected `if`");
        p.adjust(open, SyntaxKind::Error);
        return p.close(open);
    };
    skip_whitespace(p);

    let condition = p.open(SyntaxKind::Condition);
    let _ = expression(p);
    let _ = p.close(condition);
    skip_whitespace(p);

    let _ = block(p);
    skip_whitespace(p);

    if p.consume_if(SyntaxKind::Else).is_some() {
        skip_whitespace(p);
        if p.is_at(SyntaxKind::If) {
            let _ = if_expression(p);
        } else {
            let _ = block(p);
        }
    }

    p.close(open)
}

fn expression_bp(p: &mut Parser, min_bp: u8) -> MarkClosed {
    // handle prefix operators and primary expressions
    let mut lhs = if let Some(op) = prefix_op(p) {
        let ((), right_bp) = prefix_op_bp(op.kind());

        let unary = p.open(SyntaxKind::PrefixExpression);

        consume_prefix_op(p).expect("expected prefix operator");
        skip_whitespace(p);

        let _ = expression_bp(p, right_bp);

        p.close(unary)
    } else {
        primary(p)
    };

    loop {
        skip_whitespace(p);

        // handle postfix operators
        if let Some(op) = postfix_op(p) {
            let (left_bp, ()) = postfix_op_bp(op.kind());

            if left_bp < min_bp {
                break;
            }

            let unary = p.open_before(lhs, SyntaxKind::PostfixExpression);

            consume_postfix_op(p).expect("expected postfix operator");

            lhs = p.close(unary);
            continue;
        }

        // handle infix (binary) operators
        let Some(op) = infix_op(p) else {
            break;
        };

        let (left_bp, right_bp) = infix_op_bp(op.kind());
        if left_bp < min_bp {
            break;
        }

        consume_infix_op(p).expect("expected infix operator");

        let binary = p.open_before(lhs, SyntaxKind::BinaryExpression);

        skip_whitespace(p);

        let _ = expression_bp(p, right_bp);

        lhs = p.close(binary);
    }

    lhs
}

#[cfg(test)]
mod tests;
