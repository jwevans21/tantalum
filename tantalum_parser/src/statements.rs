
use tantalum_ast::{Statement, StatementKind};
use tantalum_lexer::token_kind::TokenKind;

use crate::{ParseError, Parser};

impl<'file_name, 'source> Parser<'file_name, 'source> {
    pub const STATEMENT_START: &'static [TokenKind] = &[
        TokenKind::KeywordLet,
        TokenKind::KeywordIf,
        TokenKind::KeywordFor,
        TokenKind::KeywordWhile,
        TokenKind::KeywordReturn,
        TokenKind::KeywordBreak,
        TokenKind::KeywordContinue,
        TokenKind::LeftBrace,
    ];

    pub(crate) fn parse_statement(
        &mut self,
    ) -> Result<Statement<'file_name, 'source>, ParseError<'file_name, 'source>> {
        match self.is_at_any(Self::STATEMENT_START) {
            Some(token) => match token.kind() {
                TokenKind::KeywordLet => self.parse_statement_let(),
                TokenKind::KeywordIf => self.parse_statement_if(),
                TokenKind::KeywordFor => self.parse_statement_for(),
                TokenKind::KeywordWhile => self.parse_statement_while(),
                TokenKind::KeywordReturn => self.parse_statement_return(),
                TokenKind::KeywordBreak => self.parse_statement_break(),
                TokenKind::KeywordContinue => self.parse_statement_continue(),
                TokenKind::LeftBrace => self.parse_statement_block(),
                _ => unimplemented!(
                    "Statement parsing not yet implemented for {:?}",
                    token.kind()
                ),
            },
            None => match self.is_at_any(Self::EXPRESSION_START) {
                Some(_) => {
                    let expression = self.parse_expression()?;
                    let semicolon = self.expect(TokenKind::Semicolon)?;

                    Ok(Statement {
                        span: expression.span.extend(&semicolon.span()),
                        kind: StatementKind::Expression { expression },
                    })
                }
                None => match self.nth(0) {
                    Some(token) => {
                        return Err(ParseError::unexpected_token_set(
                            self.source,
                            token.span(),
                            token.kind(),
                            &[Self::STATEMENT_START, Self::EXPRESSION_START].concat(),
                        ));
                    }
                    None => {
                        return Err(ParseError::unexpected_eof(self.source, self.eof));
                    }
                },
            },
        }
    }

    fn parse_statement_let(
        &mut self,
    ) -> Result<Statement<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let let_token = self.expect(TokenKind::KeywordLet)?;

        let name = self.expect(TokenKind::Identifier)?;

        let ty = if self.advance_if(TokenKind::Colon).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(TokenKind::Equal)?;

        let value = self.parse_expression()?;
        let semicolon = self.expect(TokenKind::Semicolon)?;

        Ok(Statement {
            span: let_token.span().extend(&semicolon.span()),
            kind: StatementKind::VariableDeclaration {
                name: name.lexeme(),
                ty,
                value,
            },
        })
    }

    fn parse_statement_if(
        &mut self,
    ) -> Result<Statement<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let if_token = self.expect(TokenKind::KeywordIf)?;

        let condition = self.parse_expression()?;
        let body = self.parse_statement_block()?;

        let else_branch = if self.advance_if(TokenKind::KeywordElse).is_some() {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement {
            span: if_token.span().extend(
                &else_branch
                    .as_ref()
                    .map_or(body.span, |branch| branch.span),
            ),
            kind: StatementKind::If {
                condition,
                body: Box::new(body),
                else_branch,
            },
        })
    }

    fn parse_statement_for(
        &mut self,
    ) -> Result<Statement<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let for_token = self.expect(TokenKind::KeywordFor)?;

        let initializer = self.parse_statement()?;
        let condition = self.parse_statement()?;
        let update = self.parse_statement()?;
        
        let body = self.parse_statement()?;

        Ok(Statement {
            span: for_token.span().extend(&body.span),
            kind: StatementKind::ForInitCondUpdate {
                initialization: Box::new(initializer),
                condition: Box::new(condition),
                update: Box::new(update),
                body: Box::new(body),
            },
        })
    }

    fn parse_statement_while(
        &mut self,
    ) -> Result<Statement<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let while_token = self.expect(TokenKind::KeywordWhile)?;

        let condition = self.parse_expression()?;
        let body = self.parse_statement_block()?;

        Ok(Statement {
            span: while_token.span().extend(&body.span),
            kind: StatementKind::While {
                condition,
                body: Box::new(body),
            },
        })
    }

    fn parse_statement_return(
        &mut self,
    ) -> Result<Statement<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let return_token = self.expect(TokenKind::KeywordReturn)?;

        let value = if self.is_at(TokenKind::Semicolon).is_some() {
            None
        } else {
            Some(self.parse_expression()?)
        };

        let semicolon = self.expect(TokenKind::Semicolon)?;

        Ok(Statement {
            span: return_token.span().extend(&semicolon.span()),
            kind: StatementKind::Return { value },
        })
    }

    fn parse_statement_break(
        &mut self,
    ) -> Result<Statement<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let break_token = self.expect(TokenKind::KeywordBreak)?;
        let semicolon = self.expect(TokenKind::Semicolon)?;

        Ok(Statement {
            span: break_token.span().extend(&semicolon.span()),
            kind: StatementKind::Break,
        })
    }

    fn parse_statement_continue(
        &mut self,
    ) -> Result<Statement<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let continue_token = self.expect(TokenKind::KeywordContinue)?;
        let semicolon = self.expect(TokenKind::Semicolon)?;

        Ok(Statement {
            span: continue_token.span().extend(&semicolon.span()),
            kind: StatementKind::Continue,
        })
    }

    fn parse_statement_block(
        &mut self,
    ) -> Result<Statement<'file_name, 'source>, ParseError<'file_name, 'source>> {
        let left_brace = self.expect(TokenKind::LeftBrace)?;

        let mut statements = Vec::new();
        while self.is_at(TokenKind::RightBrace).is_none() {
            statements.push(self.parse_statement()?);
        }

        let right_brace = self.expect(TokenKind::RightBrace)?;

        Ok(Statement {
            span: left_brace.span().extend(&right_brace.span()),
            kind: StatementKind::Block { statements },
        })
    }
}
