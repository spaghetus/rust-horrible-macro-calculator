use std::io::{BufRead, Write};

fn main() {
	let stdout = std::io::stdout();
	let stdin = std::io::stdin();
	let mut stdout = stdout.lock();
	let mut stdin = stdin.lock().lines();
	stdout.write(b"Input the first number: ").unwrap();
	stdout.flush().unwrap();
	let n1: isize = stdin.next().unwrap().unwrap().parse().unwrap();
	stdout.write(b"Input the operator: ").unwrap();
	stdout.flush().unwrap();
	let op = stdin.next().unwrap().unwrap();
	let op: &str = &op;
	stdout.write(b"Input the second number: ").unwrap();
	stdout.flush().unwrap();
	let n2: isize = stdin.next().unwrap().unwrap().parse().unwrap();
	let result = horrible_calculator_derive::generate_math!(0..100);
	println!("Your result is: {}", result);
}
