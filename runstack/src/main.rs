#[derive(Debug, PartialEq, Eq)]
enum Value<'src> {
    Num(i32),
    Op(&'src str),
    Block(Vec<Value<'src>>),
}


fn main() {

    for line in std::io::stdin().lines() {
        let mut stack = vec![];
        if let Ok(line) = line {
            let words: Vec<_> = line.split(" ").collect();
            
            while let Some((&word, mut rest)) = words.split_first() {
                if word == "{" {
                    let value;
                    (value, rest) = parse_block(rest);
                    stack.push(value);
                }else{
                    match word {
                        "+" => add(&mut stack),
                        "-" => sub(&mut stack),
                        "*" => mul(&mut stack),
                        "/" => div(&mut stack),
                        _ => panic!("{word:?} could not be parsed"),
                    }
                }
            }
        }

        println!("stack: {stack:?}");
    }
}

impl<'src> Value<'src> {
    fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("Value is not number")  
        }
    }
}

fn add(stack: &mut Vec<i32>){
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();
    stack.push(lhs + rhs);
}

fn sub(stack: &mut Vec<i32>){
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();
    stack.push(lhs - rhs);
}

fn mul(stack: &mut Vec<i32>){
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();
    stack.push(lhs * rhs);
}

fn div(stack: &mut Vec<i32>){
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();
    stack.push(lhs / rhs);
}

fn parse_block<'src, 'a>(input: &'a[&'src str],) -> (Value<'src>, &'a[&'src str]) {
    let mut tokens = vec![];
    let mut words = input;
    
    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            tokens.push(value);
        } else if word == "}" {
            return (Value::Block(tokens), rest);
        } else if Ok(value) = word.parse::<i32>() {
            tokens.push(Value::Num(value));
        } else {
            tokens.push(Value::Op(word))
        }
        words = rest;
    }
    (Value::Block(tokens), words)
}