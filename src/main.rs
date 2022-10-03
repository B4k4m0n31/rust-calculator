use std::io::stdin;

fn main() {
    calculator();
}

fn calculator() {
    println!("");
    println!("----------------------------");
    println!("|                          |");
    println!("|  Rust Simple Calculator  |");
    println!("|                          |");
    println!("----------------------------");
    println!("");

    let num1: i32 = get_number();
    let num2: i32 = get_number();

    println!("");

    let operator: char = get_operator();

    let result: i32 = calculate(num1, num2, operator);

    println!("");

    println!(
        "The result of {} {} {} = {}\n",
        num1, operator, num2, result
    )
}

fn get_number() -> i32 {
    println!("Write a number: ");
    let mut line: String = "".to_string();
    stdin().read_line(&mut line).expect("Number not valid");

    let trimmed = line.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(..) => return err_num(),
    }
}

fn err_num() -> i32 {
    println!("----");
    println!("The number introduced is not valid\n");
    return get_number();
}

fn get_operator() -> char {
    println!("Select one operation (You must introduce the character, e.g. '+'): ");
    println!("\t1. Add (+)");
    println!("\t2. Substract (-)");
    println!("\t3. Multiply (*)");
    println!("\t4. Divide (/)");

    let mut line: String = "".to_string();
    stdin().read_line(&mut line).expect("Operator not valid");
    let operator: char = line.chars().next().unwrap();

    if operator != '+' && operator != '-' && operator != '*' && operator != '/' {
        println!("----");
        println!("Operator not valid\n");
        return get_operator();
    }

    return operator;
}

fn calculate(num1: i32, num2: i32, operator: char) -> i32 {
    match operator {
        '+' => return num1 + num2,
        '-' => return num1 - num2,
        '*' => return num1 * num2,
        '/' => return num1 / num2,
        _ => return -1,
    }
}
