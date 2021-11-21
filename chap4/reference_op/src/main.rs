fn main() {
	let s = String::from("Hello!");
	let mut s_mut = String::from("Hello");

	let len = calculate_length(&s);
	println!("The length of the string '{}' is {}", s, len);
	change(&mut s_mut);
	println!("{}", s_mut);
}

fn calculate_length(s: &String) -> usize {
	s.len()
}

fn change(s: &mut String) {
	s.push_str(", world ;)");
}