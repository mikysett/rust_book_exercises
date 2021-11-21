use std::io;

fn main() {
	
	println!("Converter between Fahrenheit and Celsius");
	
	loop {
		let mut unity = String::new();
		println!("From (f = Fahrenheit, c = Celsius, q = quit): ");
		io::stdin()
			.read_line(&mut unity)
			.expect("Failed to read line");
		unity = unity.trim().to_string();
		if unity == "q" {
			break;
		}
		else if unity != "f" && unity != "c" {
			println!("Incorrect choice: {}", unity);
			continue ;
		}
		
		println!("Insert the temperature (in {}) to convert:"
			, if unity == "f" {"Fahrenheit"} else {"Celsius"});
		let mut temp = String::new();
		io::stdin()
			.read_line(&mut temp)
			.expect("Failed to read line");
		let temp: f32 = match temp.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Insert a valid number");
				continue;
			}
		};
		let conv_temp =
			if unity == "f" {(temp - 32.) * (5. / 9.)}
			else {temp * 1.8 + 32.};
		println!("Converted: {} {}", conv_temp,
			if unity == "f" {"Celcius"} else {"Fahrenheit"});
	}
}
