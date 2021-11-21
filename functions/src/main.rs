fn main() {
    println!("Hello, world!");

	another_function(5);
	let x = five();
	println!("The value of x is: {}", x);
	let x = plus_one(x);
	println!("The new value is: {}", x);
}

fn five() -> i32 {
	5
}

fn plus_one(x: i32) -> i32 {
	x + 1
}

fn another_function(x: i32) {
	println!("Another function.");
	println!("The value of x is: {}", x);
}