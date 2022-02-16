use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, U, R>
where
	T: Fn(U) -> R,
{
	calculation: T,
	values: HashMap<U, R>
}

impl<T, U, R> Cacher<T, U, R>
where
	T: Fn(U) -> R,
	U: Copy + Hash + Eq + PartialEq,
	R: Copy
{
	fn new(calculation: T) -> Cacher<T, U, R> {
		Cacher {
			calculation,
			values: HashMap::new(),
		}
	}

	fn value(&mut self, arg: U) -> R {
		let value = self.values.get(&arg);
		match value {
			Some(result) => *result,
			None => {
				let new_val = (self.calculation)(arg);
				self.values.insert(arg, new_val);
				new_val
			}
		}
	}
}

fn main() {
	let intensity = 7;
	let random_nb = 3;

	generate_workout(intensity, random_nb);
}

fn generate_workout(intensity: u32, random_nb: u32) {
	let mut expensive_result = Cacher::new(|num| {
		println!("Calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num * 2
	});

	if intensity < 25 {
		println!("Do {} pushups", expensive_result.value(intensity));
		println!("Then do {} pullups", expensive_result.value(intensity));
	} else {
		if random_nb == 3 {
			println!("Rest and enjoy");
		} else {
			println!("Run for {} minutes", expensive_result.value(intensity));
		}
	}
}


 #[cfg(test)]
 mod tests {
	use super::*;

	#[test]
	fn call_with_equal_values() {
		let mut result = Cacher::new(|a| a * 2);

		let a = result.value(10);
		let b = result.value(10);
		let c = result.value(10);
		assert_eq!(a, b);
		assert_eq!(b, c);
	}

	#[test]
	fn call_with_different_values() {
		let mut result = Cacher::new(|a| a);

		let a = result.value(10);
		let b = result.value(20);
		assert_ne!(a, b);
	}
}
