use crate::{
    parser::{Function, Node, Parser, ParserError, UnaryOperator},
    tokens::Operator,
};

pub fn evaluate(input: &str) -> Result<f64, ParserError> {
    let node = Parser::parse(input)?;
    evaluate_node(&node)
}

pub fn evaluate_node(node: &Node) -> Result<f64, ParserError> {
    match node {
        Node::Number(f) => Ok(*f),
        Node::Variable(s) => {
            if s == "PI" {
                Ok(std::f64::consts::PI)
            } else {
                Err(ParserError {
                    position: 0,
                    message: format!("Unknown constant: {s}"),
                })
            }
        }
        Node::Function { index, arg } => {
            let argument = evaluate_node(arg)?;
            match index {
                Function::Cos => Ok(argument.cos()),
                Function::Sin => Ok(argument.sin()),
                Function::Tan => Ok(argument.tan()),
                Function::Log => Ok(argument.ln()),
                Function::Exp => Ok(argument.exp()),
            }
        }
        Node::BinaryOp { op, left, right } => {
            let lhs = evaluate_node(left)?;
            let rhs = evaluate_node(right)?;
            match op {
                Operator::Plus => Ok(lhs + rhs),
                Operator::Minus => Ok(lhs - rhs),
                Operator::Times => Ok(lhs * rhs),
                Operator::Divide => {
                    if rhs == 0.0 {
                        return Err(ParserError {
                            position: 0,
                            message: "Division by 0".to_string(),
                        });
                    }
                    Ok(lhs / rhs)
                }
                Operator::Power => {
                    let x = lhs.powf(rhs);
                    if x.is_infinite() || x.is_nan() {
                        return Err(ParserError {
                            position: 0,
                            message: format!("Undefined operation {lhs}^{rhs}"),
                        });
                    }
                    Ok(x)
                }
            }
        }
        Node::UnaryOp { op, right } => match op {
            UnaryOperator::Plus => Ok(evaluate_node(right)?),
            UnaryOperator::Minus => Ok(-evaluate_node(right)?),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluate::evaluate;

    #[test]
    fn basic_operations() {
        assert_eq!(evaluate("2+3").unwrap(), 5.0);
        assert_eq!(evaluate("2-3").unwrap(), -1.0);
        assert_eq!(evaluate("4*3.5").unwrap(), 4.0 * 3.5);
        assert_eq!(evaluate("2.7/3.5").unwrap(), 2.7 / 3.5)
    }

    #[test]
    fn precedence() {
        assert_eq!(evaluate("2+3*5").unwrap(), 17.0);
        assert_eq!(evaluate("2-3/3").unwrap(), 1.0);
        assert_eq!(evaluate("4*5^2").unwrap(), 100.0);
        assert_eq!(evaluate("2+5^3").unwrap(), 127.0)
    }

    #[test]
    fn parenthesis() {
        assert_eq!(evaluate("(2+3)*5").unwrap(), 25.0);
        assert_eq!(evaluate("(2-5)/3").unwrap(), -1.0);
        assert_eq!(evaluate("(4*5)^2").unwrap(), 400.0);
        assert_eq!(evaluate("(2+5)^3").unwrap(), 343.0)
    }

    #[test]
    fn unary() {
        assert_eq!(evaluate("-(2+3)").unwrap(), -5.0);
        assert_eq!(evaluate("+(2+3)").unwrap(), 5.0);
        assert_eq!(evaluate("-2.7e3)").unwrap(), -2700.0);
    }

    #[test]
    fn pi() {
        assert_eq!(evaluate("2*PI").unwrap(), std::f64::consts::PI * 2.0);
        assert!(evaluate("Sin(PI)").unwrap().abs() < 2.0e-16);
        assert_eq!(evaluate("Sin(PI/2)").unwrap().abs(), 1.0);
        assert!(evaluate("Cos(PI/2)").unwrap().abs() < 2.0e-16);
    }

    #[test]
    fn diverse_function() {
        assert_eq!(evaluate("Log(Exp(1))").unwrap(), 1.0);
        assert_eq!(evaluate("Exp(1)").unwrap(), std::f64::consts::E);
    }
}
