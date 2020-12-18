#![feature(exclusive_range_pattern)]
use adventofcode2020::prelude::*;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Clone)]
enum Expression {
    Literal(i64),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(n) => f.write_fmt(format_args!("{}", n)),
            Expression::Add(left, right) => {
                f.write_char('(')?;
                Self::fmt(left, f)?;
                f.write_char('+')?;
                Self::fmt(right, f)?;
                f.write_char(')')?;

                Ok(())
            }
            Expression::Mul(left, right) => {
                f.write_char('(')?;
                Self::fmt(left, f)?;
                f.write_char('*')?;
                Self::fmt(right, f)?;
                f.write_char(')')?;

                Ok(())
            }
        }
    }
}

impl Expression {
    fn evaluate(&self) -> i64 {
        match self {
            Expression::Literal(n) => *n,
            Expression::Add(left, right) => left.evaluate() + right.evaluate(),
            Expression::Mul(left, right) => left.evaluate() * right.evaluate(),
        }
    }
}

mod part1 {
    use super::Error;
    use super::Expression;
    use super::Result;

    fn parse_literal_or_nested<I: Iterator<Item = char>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expression> {
        loop {
            let t1 = tokens
                .next()
                .ok_or_else(|| Error::General("Unexpected EOF".into()))?;
            match t1 {
                ' ' => continue,
                '0'..='9' => return Ok(Expression::Literal(t1 as i64 - '0' as i64)),
                '(' => {
                    let res = parse_expression(tokens)?;
                    loop {
                        let t2 = tokens
                            .next()
                            .ok_or_else(|| Error::General("Expected ')' but got EOF".into()))?;
                        match t2 {
                            ' ' => continue,
                            ')' => return Ok(res),
                            _ => {
                                return Err(Error::General(format!(
                                    "Expected ')' but got {:?}",
                                    t2
                                )))
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::General(format!(
                        "Expected number or opening parenthesis but got {:?}",
                        t1
                    )))
                }
            }
        }
    }

    fn parse_expression<I: Iterator<Item = char>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expression> {
        let mut expr = parse_literal_or_nested(tokens)?;
        loop {
            let next = tokens.peek();
            match next {
                Some(' ') => {
                    tokens.next();
                    continue;
                }
                Some('+') => {
                    tokens.next();
                    expr =
                        Expression::Add(Box::new(expr), Box::new(parse_literal_or_nested(tokens)?))
                }
                Some('*') => {
                    tokens.next();
                    expr =
                        Expression::Mul(Box::new(expr), Box::new(parse_literal_or_nested(tokens)?))
                }
                None | Some(')') => {
                    break;
                }
                Some(other) => {
                    return Err(Error::General(format!(
                        "Expected operator or ')' but got '{}",
                        other
                    )))
                }
            }
        }
        Ok(expr)
    }

    pub(super) fn parse<I: Iterator<Item = char>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expression> {
        let expr = parse_expression(tokens)?;
        let next = tokens.peek();
        match next {
            None => Ok(expr),
            Some(other) => return Err(Error::General(format!("Expected EOF but got '{}", other))),
        }
    }
}

mod part2 {
    use super::Error;
    use super::Expression;
    use super::Result;

    fn parse_literal_or_nested<I: Iterator<Item = char>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expression> {
        loop {
            let t1 = tokens
                .next()
                .ok_or_else(|| Error::General("Unexpected EOF".into()))?;
            match t1 {
                ' ' => continue,
                '0'..='9' => return Ok(Expression::Literal(t1 as i64 - '0' as i64)),
                '(' => {
                    let res = parse_multiplication(tokens)?;
                    loop {
                        let t2 = tokens
                            .next()
                            .ok_or_else(|| Error::General("Expected ')' but got EOF".into()))?;
                        match t2 {
                            ' ' => continue,
                            ')' => return Ok(res),
                            _ => {
                                return Err(Error::General(format!(
                                    "Expected ')' but got {:?}",
                                    t2
                                )))
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::General(format!(
                        "Expected number or opening parenthesis but got {:?}",
                        t1
                    )))
                }
            }
        }
    }

    fn parse_addition<I: Iterator<Item = char>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expression> {
        let mut expr = parse_literal_or_nested(tokens)?;
        loop {
            let next = tokens.peek();
            match next {
                Some(' ') => {
                    tokens.next();
                    continue;
                }
                Some('+') => {
                    tokens.next();
                    expr =
                        Expression::Add(Box::new(expr), Box::new(parse_literal_or_nested(tokens)?))
                }
                None | Some(')') | Some('*') => {
                    break;
                }
                Some(other) => {
                    return Err(Error::General(format!(
                        "Expected operator or ')' but got '{}",
                        other
                    )))
                }
            }
        }
        Ok(expr)
    }

    fn parse_multiplication<I: Iterator<Item = char>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expression> {
        let mut expr = parse_addition(tokens)?;
        loop {
            let next = tokens.peek();
            match next {
                Some(' ') => {
                    tokens.next();
                    continue;
                }
                Some('*') => {
                    tokens.next();
                    expr = Expression::Mul(Box::new(expr), Box::new(parse_addition(tokens)?))
                }
                None | Some(')') | Some('+') => {
                    break;
                }
                Some(other) => {
                    return Err(Error::General(format!(
                        "Expected operator or ')' but got '{}",
                        other
                    )))
                }
            }
        }
        Ok(expr)
    }

    pub(super) fn parse<I: Iterator<Item = char>>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Expression> {
        let expr = parse_multiplication(tokens)?;
        let next = tokens.peek();
        match next {
            None => Ok(expr),
            Some(other) => return Err(Error::General(format!("Expected EOF but got '{}", other))),
        }
    }
}

fn main() -> Result<()> {
    let lines: Vec<String> = read_file("data/18.txt")?;

    let mut tokens = "8 * 7 + 6".chars().peekable();
    let ast = part1::parse(&mut tokens)?;
    println!("{}", &ast);
    println!("{}", &ast.evaluate());

    let mut tokens = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
        .chars()
        .peekable();
    let ast = part2::parse(&mut tokens)?;
    println!("{}", &ast);
    println!("{}", &ast.evaluate());

    let part1 = lines.iter().try_fold(0_i64, |mut sum, l| -> Result<i64> {
        let mut tokens = l.chars().peekable();
        let expr = part1::parse(&mut tokens)
            .map_err(|e| Error::General(format!("Could not parse line '{}': {}", l, e)))?;
        sum += expr.evaluate();
        Ok(sum)
    })?;

    println!("{}", part1);

    let part2 = lines.iter().try_fold(0_i64, |mut sum, l| -> Result<i64> {
        let mut tokens = l.chars().peekable();
        let expr = part2::parse(&mut tokens)
            .map_err(|e| Error::General(format!("Could not parse line '{}': {}", l, e)))?;
        sum += expr.evaluate();
        Ok(sum)
    })?;

    println!("{}", part2);

    Ok(())
}
