#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];

    inputs.iter().for_each(|i| {
        let result = match i {
            CalculatorInput::Add => add(&mut stack),
            CalculatorInput::Subtract => subtract(&mut stack),
            CalculatorInput::Multiply => multiply(&mut stack),
            CalculatorInput::Divide => divide(&mut stack),
            CalculatorInput::Value(x) => Some(*x),
        };

        if let Some(x) = result {
            stack.push(x);
        }
    });

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

fn add(stack: &mut Vec<i32>) -> Option<i32> {
    if let (Some(n2), Some(n1)) = (stack.pop(), stack.pop()) {
        Some(n1 + n2)
    } else {
        None
    }
}

fn subtract(stack: &mut Vec<i32>) -> Option<i32> {
    if let (Some(n2), Some(n1)) = (stack.pop(), stack.pop()) {
        Some(n1 - n2)
    } else {
        None
    }
}

fn multiply(stack: &mut Vec<i32>) -> Option<i32> {
    if let (Some(n2), Some(n1)) = (stack.pop(), stack.pop()) {
        Some(n1 * n2)
    } else {
        None
    }
}

fn divide(stack: &mut Vec<i32>) -> Option<i32> {
    if let (Some(n2), Some(n1)) = (stack.pop(), stack.pop()) {
        Some(n1 / n2)
    } else {
        None
    }
}
