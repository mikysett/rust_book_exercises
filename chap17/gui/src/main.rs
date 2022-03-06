use::gui::{Screen, Button, SelectBox};

fn main() {
	let test = Screen {
		components: vec![
			Box::new(Button {
				width: 50,
				height: 10,
				label: String::from("OK"),
			}),
			Box::new(
				SelectBox {
					width: 40,
					height: 40,
					options: vec![
						String::from("Yes"),
						String::from("No"),
						String::from("Maybe"),
					]
				}
			)
		]
	};

	test.run();
}
