use std::char;

use crate::tokens::{Expression, Operators};

// TODO: stabilish a priority () * /
//  missing * /
pub fn calculate(input: &mut str) -> Result<String, String> {
    let mut terms = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

    // TODO: merge these two loops
    for t in terms.clone() {
        if t == '(' {
            let index = terms.iter().position(|el| el == &t).unwrap();

            let closing_index = find_closing_parenthesis(&terms, index)?;

            let operation = handle_parenthesis(&mut terms.clone()[index + 1..closing_index]);

            let expr = match handle_operation(operation) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            let result = eval_expression(expr);
            terms.splice(index..=closing_index, result);
        }
    }

    while terms.len() > 1 {
        let left = terms.remove(0);
        let operator = terms.remove(0);
        let right = terms.remove(0);

        let expr = match handle_operation(vec![left, operator, right]) {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

        let result = eval_expression(expr);

        for c in result.into_iter() {
            terms.insert(0, c)
        }
    }

    Ok(terms.into_iter().collect())
}
fn handle_parenthesis(input: &mut [char]) -> Vec<char> {
    let mut sanitized_input = input.to_vec();
    sanitized_input.retain(|el| el != &'(' && el != &')');

    sanitized_input
}
fn handle_operation(mut expression: Vec<char>) -> Result<Expression, String> {
    let left = match expression.remove(0) {
        el if el.is_numeric() => el.to_string().parse::<i32>().unwrap(),
        _ => return Err("not a number".to_string()),
    };

    let operator = match expression.remove(0) {
        '+' => Operators::Plus,
        '-' => Operators::Minus,
        '/' => Operators::Division,
        '*' => Operators::Times,
        _ => Operators::Invalid,
    };

    let right = match expression.remove(0) {
        el if el.is_numeric() => el.to_string().parse::<i32>().unwrap(),
        _ => return Err("not a number".to_string()),
    };

    let expr = Expression {
        left,
        operator,
        right,
    };

    Ok(expr)
}

fn eval_expression(
    Expression {
        left,
        operator,
        right,
    }: Expression,
) -> Vec<char> {
    let result = match operator {
        Operators::Times => left * right,
        Operators::Plus => left + right,
        Operators::Division => left / right,
        Operators::Minus => left - right,
        _ => 0 - 1,
    };
    result.to_string().chars().collect()
}

fn find_closing_parenthesis(terms: &[char], open_index: usize) -> Result<usize, String> {
    let mut count = 0;
    for (i, &ch) in terms.iter().enumerate().skip(open_index) {
        if ch == '(' {
            count += 1;
        } else if ch == ')' {
            count -= 1;
            if count == 0 {
                return Ok(i);
            }
        }
    }
    Err("Unmatched parenthesis".to_string())
}
