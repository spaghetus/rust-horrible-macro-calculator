extern crate proc_macro;
use std::ops::Range;

use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_math(input: TokenStream) -> TokenStream {
	let input = input.to_string();
	let input = input.chars().filter(|c| c != &' ').collect::<String>();
	let input = input.split("..").collect::<Vec<_>>();
	assert!(input.len() == 2);
	let range: Range<i128> = input[0].trim().parse().unwrap()..input[1].trim().parse().unwrap();
	let operands = ["+", "-", "*", "/"];
	let mut expanded = String::new();
	expanded.extend("match op {".chars());
	for op in operands.iter() {
		expanded.extend(format!("\"{}\" => match (n1, n2) {{", op).chars());
		for n1 in range.clone() {
			for n2 in range.clone() {
				expanded.extend(
					format!(
						"({}, {}) => {},",
						n1,
						n2,
						match *op {
							"+" => format!("{}", n1 + n2),
							"-" => format!("{}", n1 - n2),
							"*" => format!("{}", n1 * n2),
							"/" =>
								if n2 == 0 {
									"panic!(\"Division by zero\")".to_string()
								} else {
									format!("{}", n1 / n2)
								},
							_ => unreachable!(),
						}
					)
					.chars(),
				);
			}
		}
		expanded.extend(
			format!("(_,_)=>panic!(\"Invalid numbers, try compiling with a larger range.\"),")
				.chars(),
		);
		expanded.extend("},".chars());
	}
	expanded.extend("_=>panic!(\"Invalid operator\"),".chars());
	expanded.extend("}".chars());
	expanded.parse().unwrap()
}
