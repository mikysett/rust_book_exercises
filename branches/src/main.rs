fn main() {
	let condition = true;
	let number = if condition {3} else {7};

	if number < 5 {
		println!("Condition was true");
	}
	else {
		println!("Condition was false");
	}
}
