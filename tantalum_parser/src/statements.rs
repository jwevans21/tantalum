use tantalum_ast::{Block, ForInitCondUpdate, If, Return, Statement, VariableDeclaration, While};
use tantalum_lexer::token_kind::TokenKind;
use tantalum_span::Spanned;

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
    ) -> Result<Spanned<'file_name, Statement<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        match self.is_at_any(Self::STATEMENT_START) {
            Some(token) => match token.data().kind() {
                TokenKind::KeywordLet => {
                    let variable_declaration = self.parse_statement_let()?;
                    Ok(variable_declaration.map(Statement::VariableDeclaration))
                }
                TokenKind::KeywordIf => self
                    .parse_statement_if()
                    .map(|statement| statement.map(Statement::If)),
                TokenKind::KeywordFor => self
                    .parse_statement_for()
                    .map(|statement| statement.map(Statement::ForInitCondUpdate)),
                TokenKind::KeywordWhile => self
                    .parse_statement_while()
                    .map(|statement| statement.map(Statement::While)),
                TokenKind::KeywordReturn => self
                    .parse_statement_return()
                    .map(|statement| statement.map(Statement::Return)),
                TokenKind::KeywordBreak => self.parse_statement_break(),
                TokenKind::KeywordContinue => self.parse_statement_continue(),
                TokenKind::LeftBrace => self
                    .parse_statement_block()
                    .map(|statement| statement.map(Statement::Block)),
                _ => unimplemented!(
                    "Statement parsing not yet implemented for {:?}",
                    token.data().kind()
                ),
            },
            None => match self.is_at_any(Self::EXPRESSION_START) {
                Some(_) => {
                    let expression = self.parse_expression()?;
                    let semicolon = self.expect(TokenKind::Semicolon)?;

                    Ok(Spanned::join_spans(
                        expression.span(),
                        semicolon.span(),
                        expression.map(Statement::Expression).data().to_owned(),
                    ))
                }
                None => match self.nth(0) {
                    Some(token) => {
                        return Err(ParseError::unexpected_token_set(
                            self.source,
                            token.span().start(),
                            token.data().kind(),
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
    ) -> Result<
        Spanned<'file_name, VariableDeclaration<'file_name, 'source>>,
        ParseError<'file_name, 'source>,
    > {
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

        Ok(Spanned::join_spans(
            let_token.span(),
            semicolon.span(),
            VariableDeclaration {
                name: name.map(|name| name.lexeme()),
                ty,
                value,
            },
        ))
    }

    fn parse_statement_if(
        &mut self,
    ) -> Result<Spanned<'file_name, If<'file_name, 'source>>, ParseError<'file_name, 'source>> {
        let if_token = self.expect(TokenKind::KeywordIf)?;

        let condition = self.parse_expression()?;
        let body = self.parse_statement()?;

        let else_branch = if self.advance_if(TokenKind::KeywordElse).is_some() {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Spanned::join_spans(
            if_token.span(),
            else_branch
                .as_ref()
                .map_or(body.span(), |branch| branch.span()),
            If {
                condition,
                body: Box::new(body),
                else_branch,
            },
        ))
    }

    fn parse_statement_for(
        &mut self,
    ) -> Result<
        Spanned<'file_name, ForInitCondUpdate<'file_name, 'source>>,
        ParseError<'file_name, 'source>,
    > {
        let for_token = self.expect(TokenKind::KeywordFor)?;

        let initializer = self.parse_statement()?;
        let condition = self.parse_statement()?;
        let update = self.parse_statement()?;

        let body = self.parse_statement()?;

        Ok(Spanned::join_spans(
            for_token.span(),
            body.span(),
            ForInitCondUpdate {
                init: Box::new(initializer),
                condition: Box::new(condition),
                update: Box::new(update),
                body: Box::new(body),
            },
        ))
    }

    fn parse_statement_while(
        &mut self,
    ) -> Result<Spanned<'file_name, While<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let while_token = self.expect(TokenKind::KeywordWhile)?;

        let condition = self.parse_expression()?;
        let body = self.parse_statement()?;

        Ok(Spanned::join_spans(
            while_token.span(),
            body.span(),
            While {
                condition,
                body: Box::new(body),
            },
        ))
    }

    fn parse_statement_return(
        &mut self,
    ) -> Result<Spanned<'file_name, Return<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let return_token = self.expect(TokenKind::KeywordReturn)?;

        let value = if self.is_at(TokenKind::Semicolon).is_some() {
            None
        } else {
            Some(self.parse_expression()?)
        };

        let semicolon = self.expect(TokenKind::Semicolon)?;

        Ok(Spanned::join_spans(
            return_token.span(),
            semicolon.span(),
            Return { value },
        ))
    }

    fn parse_statement_break(
        &mut self,
    ) -> Result<Spanned<'file_name, Statement<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let break_token = self.expect(TokenKind::KeywordBreak)?;
        let semicolon = self.expect(TokenKind::Semicolon)?;

        Ok(Spanned::join_spans(
            break_token.span(),
            semicolon.span(),
            Statement::Break,
        ))
    }

    fn parse_statement_continue(
        &mut self,
    ) -> Result<Spanned<'file_name, Statement<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let continue_token = self.expect(TokenKind::KeywordContinue)?;
        let semicolon = self.expect(TokenKind::Semicolon)?;

        Ok(Spanned::join_spans(
            continue_token.span(),
            semicolon.span(),
            Statement::Continue,
        ))
    }

    fn parse_statement_block(
        &mut self,
    ) -> Result<Spanned<'file_name, Block<'file_name, 'source>>, ParseError<'file_name, 'source>>
    {
        let left_brace = self.expect(TokenKind::LeftBrace)?;

        let mut statements = Vec::new();
        while self.is_at(TokenKind::RightBrace).is_none() {
            statements.push(self.parse_statement()?);
        }

        let right_brace = self.expect(TokenKind::RightBrace)?;

        Ok(Spanned::join_spans(
            left_brace.span(),
            right_brace.span(),
            Block { statements },
        ))
    }
}
