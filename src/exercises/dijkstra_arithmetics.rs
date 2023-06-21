use crate::data_structures::stack::Stack;

pub fn evaluate(expression: &str) -> f32 {
    let mut operator_stack: Stack<Box<dyn Fn(f32, f32) -> f32>> = Stack::new();
    let mut operand_stack: Stack<f32> = Stack::new();

    for c in expression.chars() {
        match c {
            c if c.is_digit(10) => {
                let d = c.to_digit(10).unwrap() as f32;
                operand_stack.push(d);
            }
            c if is_operator(c) => {
                let operator = to_operator(c);
                operator_stack.push(operator);
            }
            ')' => {
                let operand1 = operand_stack.pop().expect("Invalid syntax");
                let operand2 = operand_stack.pop().expect("Invalid syntax");
                let operator = operator_stack.pop().expect("Invalid syntax");

                operand_stack.push(operator(operand1, operand2));
            }
            _ => continue,
        }
    }

    operand_stack.pop().expect("Invalid syntax")
}

fn to_operator(c: char) -> Box<dyn Fn(f32, f32) -> f32> {
    match c {
        '+' => Box::new(add),
        '*' => Box::new(mul),
        op => panic!("Operator now considered: {op}"),
    }
}

fn is_operator(c: char) -> bool {
    c == '*' || c == '+'
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}
