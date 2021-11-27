struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

// Tuble structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit like structs
struct AlwaysEqual;

fn main() {
	let user1 = User {
		username: String::from("first user"),
		email: String::from("myemail@email.com"),
		active: true,
		sign_in_count: 0,
	};

	let mut user2 = User {
		username: String::from("second guy"),
		email: String::from("simple@email.com"),
		active: false,
		sign_in_count: 10,
	};
	user2.email = String::from("complicate@email.com");

	let automatic_user = build_user(String::from("test@email.com"), String::from("test user"));

	let user3 = User {
		email: String::from("second_test@email.com"),
		..automatic_user
	};
}

fn build_user(email: String, username: String) -> User {
	User {
		email, // Field init shorthand
		username,
		active: true,
		sign_in_count: 1,
	}
}
