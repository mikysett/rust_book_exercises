pub trait Draw {
	fn draw(&self);
}

pub struct Screen {
	pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}

pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
}

impl Button {
	pub fn new() -> Button {
		Button {
			width: 0,
			height: 0,
			label: String::new(),
		}
	}
}

impl Draw for Button {
	fn draw(&self) {
		println!("Printing a button");
	}
}

pub struct SelectBox {
	pub width: u32,
	pub height: u32,
	pub options: Vec<String>,
}

impl SelectBox {
	pub fn new() -> SelectBox {
		SelectBox {
			width: 0,
			height: 0,
			options: vec![],
		}
	}
}

impl Draw for SelectBox {
	fn draw(&self) {
		println!("Printing a select box");
	}
}