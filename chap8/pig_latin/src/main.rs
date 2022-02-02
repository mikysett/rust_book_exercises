use std::io;

fn main() {
	
	loop {
		let mut line = String::new();
		
		println!("Insert the string to convert in pig latin:");
		io::stdin()
			.read_line(&mut line)
			.expect("Can't read the line");
		line = line.trim().to_string();
		if line == "q" {
			break;
		}
		let converted = convert_to_pig_latin(&line);
		println!("Result: {}", converted);
	}
}

fn convert_to_pig_latin(s1: &String) -> String {
	let mut converted_line = String::new();

	let mut chars = s1.chars();
	while let Some(c) = chars.next() {
		if c.is_alphabetic() {
			let first_char = set_first_char(c);
			if first_char == 'h' {
				converted_line.push(c);
			}
			let mut last_char: Option<char> = None;
			while let Some(c) = chars.next() {
				if c.is_alphabetic() {
					converted_line.push(c);
				}
				else {
					last_char = Some(c);
					break;
				}
			};
			converted_line += &format!("-{}ay", first_char);
			if let Some(c) = last_char {
				converted_line.push(c);
			}
		}
		else {
			converted_line.push(c);
		}
	}
	converted_line
}

fn set_first_char(c: char) -> char {
	if is_vowel(c) {
		'h'
	}
	else {
		c
	}
}

fn is_vowel(c: char) -> bool {
	c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' 
}

#[cfg(test)]
mod test {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
