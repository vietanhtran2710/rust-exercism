#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for item in inputs {
        match item {
            CalculatorInput::Value(value)=>{
                stack.push(*value);
            },
            operator=> {
                if stack.len() < 2 {
                    return None;
                }
                let value1 = stack.pop().unwrap();
                let value2 = stack.pop().unwrap();
                match operator {
                    CalculatorInput::Add=>{
                        stack.push(value1 + value2);
                    },
                    CalculatorInput::Subtract=>{
                        stack.push(value2 - value1);
                    },
                    CalculatorInput::Multiply=>{
                        stack.push(value1 * value2);
                    },
                    CalculatorInput::Divide=>{
                        stack.push(value2 / value1);
                    },
                    _ => {

                    }
                }
            }
        }
    }
    if stack.len() != 1 {
        return None;
    }
    stack.pop()
}
