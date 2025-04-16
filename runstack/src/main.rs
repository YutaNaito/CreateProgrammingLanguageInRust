#[derive(Debug, PartialEq, Eq)]
enum Value<'src> {
    Num(i32),
    Op(&'src, str)
    Block(Vec<Value<'src>>),
}


fn main() {

    for line in std::io::stdin().lines() {
        let mut stack = vec![];
        if let Ok(line) = line {
            let words: Vec<_> = line.split(" ").collect();
            
            while let Some((&words, mut rest)) = words.split_first() {
                if words == "{" {
                    let value;
                    (value, rest) = parse_block(rest)
                    stack.push(value);
                }else{
                    match word {
                        "+" => add(&mut stack),
                        "-" => sub(&mut stack),
                        "*" => mul(&mut stack),
                        "/" => div(&mut stack),
                        _ => panic!("{word:?} could not be parsed")
                    }
                }
            }
        }

        println!("stack: {stack:?}");
    }
}

fn add(stack: &mut Vec<i32>){
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs + rhs);
}

fn sub(stack: &mut Vec<i32>){
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs - rhs);
}

fn mul(stack: &mut Vec<i32>){
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs * rhs);
}

fn div(stack: &mut Vec<i32>){
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs / rhs);
}

fn parsed_block<'src, 'a>(input: &'a[&'src str],) -> (Value<'src>, &'a[&'src str]) {
    let mut tokens = vec![];
    let mut words = input;
    // TODO: implement logic
    words;
}