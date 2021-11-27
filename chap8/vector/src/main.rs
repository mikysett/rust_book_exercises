use std::collections::HashMap;

fn main() {
    let numbers = vec![10, 5, 3, 100, 50, 76, 80, 81, 82, 86];
	
	print_vector(&numbers);
    println!("The mean of the vector is: {}", mean(&numbers));
	print_vector(&numbers);
	println!("The median of the vector is: {}", median(&numbers));
	print_vector(&numbers);
    println!("Hello, world!");
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

}
