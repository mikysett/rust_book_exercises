fn main() {
	let s = String::from("Hello, world!");
	let first_word = first_word(&s);
	println!("first word: {}", first_word);

	hard_test();
}

fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();
	
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}

fn hard_test() {
	let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_improved(&my_string[0..6]);
    let word = first_word_improved(&my_string[..]);
    // `first_word_improved` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_improved(&my_string);

    let my_string_literal = "hello world";

    // `first_word_improved` works on slices of string literals, whether partial or whole
    let word = first_word_improved(&my_string_literal[0..6]);
    let word = first_word_improved(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_improved(my_string_literal);
}

fn first_word_improved(s: &str) -> &str {
	let bytes = s.as_bytes();
	
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}
