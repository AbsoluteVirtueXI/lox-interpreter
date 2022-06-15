use crate::token::Token;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: String,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

trait Parser {
    type Output;
    fn parse_binary(
        &self,
        operator: &Token,
        left: &Box<Expr>,
        right: &Box<Expr>,
    ) -> Result<Self::Output, String>;
    fn parse_grouping(&self, expression: &Box<Expr>) -> Result<Self::Output, String>;
    fn parse_literal(&self, value: &String) -> Result<Self::Output, String>;
    fn parse_unary(&self, operator: &Token, right: &Box<Expr>) -> Result<Self::Output, String>;
}

impl Expr {
    fn accept<T: Parser>(&self, p: &T) -> Result<T::Output, String> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => p.parse_binary(operator, left, right),
            Expr::Grouping { expression } => p.parse_grouping(expression),
            Expr::Literal { value } => p.parse_literal(value),
            Expr::Unary { operator, right } => p.parse_unary(operator, right),
        }
    }
}

pub struct AstPrinter;
impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self).unwrap()
    }

    fn parenthesize(
        &self,
        name: String,
        exprs: Vec<&Expr>,
    ) -> Result<<Self as Parser>::Output, String> {
        let mut output = String::new();
        output.push_str("(");
        output.push_str(&name);
        for expr in exprs {
            output.push_str(" ");
            output.push_str(&expr.accept(self).unwrap());
        }
        output.push_str(")");
        Ok(output)
    }
}

impl Parser for AstPrinter {
    type Output = String;
    fn parse_binary(
        &self,
        operator: &Token,
        left: &Box<Expr>,
        right: &Box<Expr>,
    ) -> Result<Self::Output, String> {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }
    fn parse_grouping(&self, expression: &Box<Expr>) -> Result<Self::Output, String> {
        self.parenthesize("group".to_string(), vec![expression])
    }
    fn parse_literal(&self, value: &String) -> Result<Self::Output, String> {
        if value.is_empty() {
            Ok("nil".to_string())
        } else {
            Ok(value.to_string())
        }
    }
    fn parse_unary(&self, operator: &Token, right: &Box<Expr>) -> Result<Self::Output, String> {
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use super::*;
    use crate::token_type::TokenType;
    #[test]
    fn basic_expression() {
        let expression = Binary {
            left: Box::new(Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), "".to_string(), 1),
                right: Box::new(Literal {
                    value: "123".to_string(),
                }),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), "".to_string(), 1),
            right: Box::new(Grouping {
                expression: Box::new(Literal {
                    value: "45.67".to_string(),
                }),
            }),
        };
        let ast_printer = AstPrinter;
        let res = ast_printer.print(expression);
        println!("===> {}", res);
        assert_eq!("(* (- 123) (group 45.67))".to_string(), res);
    }
}
