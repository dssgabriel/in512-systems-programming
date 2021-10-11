#[derive(Debug, PartialEq)]
pub enum ComputeError {
    DivisionByZero,
    EmptyStack,
    //StackUnderflow,
    //UnfinishedExpression,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
pub enum Token {
    Number(i32),
    Op(Operator),
}

/// Evalutates an expression in Reverse Polish Notation (RPN).
/// If the expression is correct, `compute` returns the result of the expression
/// with type `Ok(i32)`.
/// If the expression raises an error, `compute` returns an error with type
/// `Err(ComputeError)`.
///
/// # Example
/// ```
/// use lab3::exercise3::*;
/// let r = compute(&[Token::Number(4), Token::Number(3), Token::Op(Operator::Plus)]);
/// assert_eq!(r, Ok(7));
/// ```
pub fn compute(input: &[Token]) -> Result<i32, ComputeError> {
    if input.is_empty() {
        return Err(ComputeError::EmptyStack);
    }

    let mut stack = Vec::new();

    for token in input {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Op(op) => {
                // Checking that we have at least 2 operands in the stack
                if stack.len() < 2 {
                    // Could return another error type like `StackUnderflow`
                    return Err(ComputeError::EmptyStack);
                }

                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                match op {
                    Operator::Plus => stack.push(b + a),
                    Operator::Minus => stack.push(b - a),
                    Operator::Times => stack.push(b * a),
                    Operator::Divide => {
                        if a == 0 {
                            return Err(ComputeError::DivisionByZero);
                        }
                        stack.push(b / a);
                    },
                }
            }
        }
    }

    // Could return another error type like `UnfinishedExpression`
    if stack.len() != 1 {
        return Err(ComputeError::EmptyStack);
    }

    Ok(stack[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let r = compute(&[]);
        assert_eq!(r, Err(ComputeError::EmptyStack));
    }
    #[test]
    fn just_a_number() {
        let r = compute(&[Token::Number(5)]);
        assert_eq!(r, Ok(5));
    }

    #[test]
    fn plus() {
        let r = compute(&[
            Token::Number(12),
            Token::Number(-5),
            Token::Op(Operator::Plus),
        ]);
        assert_eq!(r, Ok(7));
    }

    #[test]
    fn minus() {
        let r = compute(&[
            Token::Number(3),
            Token::Number(7),
            Token::Op(Operator::Minus),
        ]);
        assert_eq!(r, Ok(-4));
    }

    #[test]
    fn times() {
        let r = compute(&[
            Token::Number(5),
            Token::Number(7),
            Token::Op(Operator::Times),
        ]);
        assert_eq!(r, Ok(35));
    }

    #[test]
    fn divide_ok() {
        let r = compute(&[
            Token::Number(15),
            Token::Number(2),
            Token::Op(Operator::Divide),
        ]);
        assert_eq!(r, Ok(7));
    }

    #[test]
    fn divide_err() {
        let r = compute(&[
            Token::Number(15),
            Token::Number(0),
            Token::Op(Operator::Divide),
        ]);
        assert_eq!(r, Err(ComputeError::DivisionByZero));
    }

    #[test]
    fn complex_expression() {
        let r = compute(&[
            Token::Number(1),
            Token::Number(2),
            Token::Number(3),
            Token::Number(4),
            Token::Op(Operator::Minus),
            Token::Op(Operator::Times),
            Token::Number(3),
            Token::Op(Operator::Times),
            Token::Op(Operator::Plus),
        ]);
        assert_eq!(r, Ok(-5));
    }

    #[test]
    fn stack_underflow() {
        let r = compute(&[Token::Number(4), Token::Op(Operator::Minus)]);
        assert_eq!(r, Err(ComputeError::EmptyStack));
    }
}
