fn main() {
    let mut stack = vec![];

    stack.push(42);
    stack.push(36);
    println!("stack: {stack:?}");

    add(&mut stack);
    println!("stack: {stack:?}");
    
}

fn add(stack: &mut Vec<i32>){
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs + rhs);
}


