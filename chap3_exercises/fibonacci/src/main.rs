use std::io;

fn main() {
	println!("The nth fibonacci number");
	
	loop {
		let mut number = String::new();

		println!("Insert the position you would like me to calculate:");
		io::stdin()
			.read_line(&mut number)
			.expect("Failed to read the line");
		let number: u32 = match number.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Insert a valid number");
				continue;
			}
		};
		println!("The {}nth fibonacci number is: {}", number, fibonacci(number, 0, 1));
	}
}

fn fibonacci(iterations: u32, last: u32, before_last: u32) -> u32 {
	if iterations == 0 {
		0
	}
	else if iterations == 1 {
		last + before_last
	}
	else {
		fibonacci(iterations - 1, last + before_last, last)
	}
}
