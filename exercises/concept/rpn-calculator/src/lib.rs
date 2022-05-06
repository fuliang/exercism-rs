#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use CalculatorInput::*;

pub fn take_pair(stack: &mut Vec<CalculatorInput>) -> (Option<CalculatorInput>, Option<CalculatorInput>) {
    (stack.pop(), stack.pop())
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<CalculatorInput> = vec![];
    for input in inputs {
        match input {
            Value(val) => {
                stack.push(Value(*val));
            },

            Add => {
                match take_pair(&mut stack) {
                    (Some(Value(v2)), Some(Value(v1))) => stack.push(Value(v1 + v2)),
                    _ => return None,
                }
            },
            
            Subtract => {
                match take_pair(&mut stack) {
                    (Some(Value(v2)), Some(Value(v1))) => stack.push(Value(v1 - v2)),
                    _ => return None,
                }
            },

            Multiply => {
                match take_pair(&mut stack) {
                    (Some(Value(v2)), Some(Value(v1))) => stack.push(Value(v1 * v2)),
                    _ => return None,
                }
            },
            
            Divide => {
                match take_pair(&mut stack) {
                    (Some(Value(v2)), Some(Value(v1))) => stack.push(Value(v1 / v2)),
                    _ => return None,
                }
            },
            _ => return None,
        }
    }

    if stack.len() != 1 {
        return None;
    }
    
    let result = stack.pop();
    match result {
        Some(Value(val)) => Some(val),
        _ => None,
    }
}
