use std::io;

fn main() {
    let (n0, n1);
    n0 = get_f();
    n1 =  get_f();
    let operator = get_c();

    println!("{n0} {operator} {n1} = {}", calc(operator, n0, n1))
}

fn calc(c: char, n0: f32, n1: f32) -> f32 {
    let mut n: f32 = 0.0; 
    if c=='+' {
        n = n0 + n1;
    } else if c=='-' {
        n = n0 - n1;
    } else if c=='*' {
        n = n0 * n1;
    } else if c=='/' {
        n = n0 / n1;
    } else if c=='%' {
        n = n0 % n1;
    };
    return n;
}

fn get_f() -> f32 {
    println!("Enter a number: ");
    let mut n: String = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line.");
    return n.trim().parse::<f32>().unwrap();
}

fn get_c() -> char {
    println!("Enter an operator: ");
    let mut c: String = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line.");
    return c.trim().parse::<char>().unwrap();
}