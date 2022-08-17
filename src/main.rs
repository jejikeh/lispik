use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = String::from("(pow 10 6)");

    let mut ops: HashMap<&str, fn(Vec<String>) -> i32> = HashMap::new();
    ops.insert("+", plus);
    ops.insert("-", minus);
    ops.insert("*", multiply);
    ops.insert("/", devide);
    ops.insert("pow", pow);

    let tokens = tokenize(&input, ops.to_owned());

    // stack of all expressions
    let mut expressions_stack: Vec<String> = Vec::new();
    // stack of results of each expressions
    let mut i_stack: Vec<i32> = Vec::new();
    // stack of operator of each expression
    // let mut i_child_expressions_stack: Vec<i32> = Vec::new();
    for token in tokens.iter() {
        if *token == ")" {
            let mut expresion_stack = parse_expression(&mut expressions_stack, ops.to_owned());

            let operation = expresion_stack.pop().unwrap();
            let r = ops.get(operation.as_str()).unwrap()(expresion_stack);
            i_stack.push(r);
            expressions_stack.push(r.to_string());
        } else {
            expressions_stack.push(token.to_string());
        }
    }

    println!("{:?}", i_stack.pop().unwrap());
}

fn tokenize(input: &str, ops: HashMap<&str, fn(Vec<String>) -> i32>) -> Vec<String> {
    let iterator = input.split_whitespace();
    let parsed: Vec<&str> = iterator.collect();

    let mut tokens: Vec<String> = Vec::new();

    let mut opened_expressions = 0;
    let mut closed_expressions = 0;
    let mut operators = 0;

    for item in parsed.iter() {
        // TODO: automatic add spaces to each char
        let mut p_item = item.replace("(", " ( ").replace(")", " ) ");
        let white_space = " ";
        for op in ops.iter() {
            let temp_paste = [white_space, op.0].join("");
            let paste = [temp_paste, white_space.to_string()].join(" ");
            p_item = p_item.replace(op.0, &paste);
        }

        let iterator = p_item.split_whitespace();
        let tokenst: Vec<&str> = iterator.collect();
        for i in tokenst.iter() {
            if i == &"(" {
                opened_expressions += 1;
            } else if i == &")" {
                closed_expressions += 1;
            } else if ops.contains_key(i) {
                operators += 1;
            }
            tokens.push(i.to_string());
        }
    }
    if opened_expressions != closed_expressions {
        println!("The number of open expressions and closed ones are not equal!\nNumber of open expressions: {}\nNumber of closed expressions: {}", opened_expressions, closed_expressions);
        std::process::exit(1);
    }
    if operators != closed_expressions {
        println!("The number of operators is greater than the number of expressions.\nOne operator per expression.");
        std::process::exit(1);
    }

    tokens
}

fn parse_expression(
    expressions_stack: &mut Vec<String>,
    ops: HashMap<&str, fn(Vec<String>) -> i32>,
) -> Vec<String> {
    let mut sub_stack = Vec::new();
    loop {
        let t = match expressions_stack.pop() {
            None => continue,
            Some(t) => {
                sub_stack.push(t.clone().to_string());
                t
            }
        };

        // Iterate the expression until we find some operator
        if t == "(" {
            sub_stack.pop();
            if sub_stack.len() == 0 {
                println!("Empty expression!");
                std::process::exit(1);
            }
            if ops.contains_key(&sub_stack[sub_stack.len() - 1].as_str()) {
                return sub_stack;
            } else {
                // TODO: fix typos
                println!(
                    "Invalid operator {:?} or it is in the wrong place.",
                    sub_stack
                );
                std::process::exit(1);
            }
        }
    }
}

fn get_expression_numbers(expression_stack: Vec<String>) -> Vec<i32> {
    let mut i_values: Vec<i32> = Vec::new();
    // println!("{:?}", expression_stack);
    for values in expression_stack.iter() {
        match i32::from_str(values) {
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(1);
            }
            Ok(v) => {
                i_values.push(v);
            }
        };
    }
    i_values
}

fn plus(expression_stack: Vec<String>) -> i32 {
    let mut i_stack = get_expression_numbers(expression_stack);
    let first = i_stack.pop().unwrap();
    i_stack.iter().fold(first, |acc, x| acc + x)
}

fn minus(expression_stack: Vec<String>) -> i32 {
    let mut i_stack = get_expression_numbers(expression_stack);
    let first = i_stack.pop().unwrap();

    i_stack.iter().fold(first, |acc, x| acc - x)
}

fn multiply(expression_stack: Vec<String>) -> i32 {
    let mut i_stack = get_expression_numbers(expression_stack);
    let first = i_stack.pop().unwrap();

    i_stack.iter().fold(first, |acc, x| acc * x)
}

fn devide(expression_stack: Vec<String>) -> i32 {
    let mut i_stack = get_expression_numbers(expression_stack);
    let first = i_stack.pop().unwrap();

    i_stack.iter().fold(first, |acc, x| acc / x)
}

fn incf(expression_stack: Vec<String>) -> i32 {
    let mut i_stack = get_expression_numbers(expression_stack);
    let first = i_stack.pop().unwrap();

    i_stack
        .iter()
        .fold(first, |acc, x| i32::pow(acc, x.abs() as u32))
}

fn pow(expression_stack: Vec<String>) -> i32 {
    let mut i_stack = get_expression_numbers(expression_stack);
    let first = i_stack.pop().unwrap();

    i_stack
        .iter()
        .fold(first, |acc, x| i32::pow(acc, x.abs() as u32))
}
