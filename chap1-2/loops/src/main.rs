fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
			println!("end of inner loop");
        }

        count += 1;
    }
    println!("End count = {}", count);
	simple_loop();
	while_loop();
	for_loop();
	for_loop_range();
}

fn simple_loop() {
	let mut counter = 0;
	println!("SIMPLE LOOP");

	let result = loop {
		counter += 1;
		if counter == 10 {
			break counter * 2;
		}
	};

	println!("Result: {}", result);
}

fn while_loop() {
    let mut number = 3;
	println!("WHILE LOOP");

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
	let a = [10, 20, 30, 40, 50];
	println!("FOR LOOP");
	
	for element in a {
		println!("a: {}", element);
	}
}

fn for_loop_range() {
	println!("FOR LOOP WITH RANGE");

	for number in (1..4).rev() {
		println!("{}!", number);
	}
    println!("LIFTOFF!!!");
}
