use blog::Post;

fn main() {
	let mut post = Post::new();

	post.add_text("I ate a good pasta");
	assert_eq!("", post.content());

	post.request_review();
	assert_eq!("", post.content());

	post.approve();
	assert_eq!("I ate a good pasta", post.content());

	println!("No panic means it's all good");
	println!("The text of the blog is: {}", post.content());
}
