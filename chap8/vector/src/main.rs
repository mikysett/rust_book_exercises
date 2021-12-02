use std::collections::HashMap;

fn main() {
    let numbers = vec![10, 5, 43, 3, 100, 43, 50, 76, 80, 43, 81, 100, 82, 82, 86];
	
	print_vector(&numbers);
    println!("The mean of the vector is: {}", mean(&numbers));
	print_vector(&numbers);
	println!("The median of the vector is: {}", median(&numbers));
	print_vector(&numbers);
	println!("The mode of the vector is: {}", mode(&numbers));
	print_vector(&numbers);
}

fn print_vector(numbers: &Vec<i32>) {
	for nb in numbers.iter() {
		print!("{} ", nb);
	}
	println!("");
}

fn mean(numbers: &Vec<i32>) -> i32 {
    // let mut tot = 0;

    // for nb in numbers.iter() {
    // 	tot += nb;
    // }
    // tot / numbers.len() as i32
    numbers.iter().sum::<i32>() / numbers.len() as i32
}

fn median(numbers: &Vec<i32>) -> i32 {
	let mut numbers = numbers.clone();

	numbers.sort_unstable();
	print!("The sorted vector: ");
	print_vector(&numbers);
    let numbers_len = numbers.len();
    if numbers_len % 2 == 0 {
        (numbers[numbers_len / 2] + numbers[numbers_len / 2 - 1]) / 2
    } else {
        numbers[numbers_len / 2]
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
	let mut map = HashMap::new();

	for nb in numbers {
		let count = map.entry(nb).or_insert(0);
		*count += 1;
	}

	let mut best_nb = (0, 0);
	for (key, val) in map.iter() {
		if *val > best_nb.1 {
			best_nb.0 = **key;
			best_nb.1 = *val;
		}
	}
	best_nb.0
}
