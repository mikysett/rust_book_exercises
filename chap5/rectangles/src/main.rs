#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

// My first method in Rust
impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	fn width(&self) -> bool {
		self.width > 0
	}
	fn can_hold(&self, other_rect: &Rectangle) -> bool {
		self.width > other_rect.width && self.height > other_rect.height
	}
	// Associated function (no self)
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}
}

fn main() {
	let width1 = 30;
	let height1 = 50;
	
	println!("The aria of the rectangle is {} pixels",
		area(width1, height1));

	
	let rect1 = (30, 50);
	println!("The aria of the rectangle is {} pixels",
		area1(rect1));

	let rect2 = Rectangle {
		width: dbg!(30 * 2),
		height: 50,
	};
	println!("The aria of the rectangle is {} pixels",
		area2(&rect2));

	println!("Values of rect2 are {:?}", rect2);
	println!("Values of rect2 are {:#?}", rect2);
	dbg!(&rect2);

	println!("The aria of the rectangle is {} pixels",
		rect2.area());

	can_hold_check();

	let sq = Rectangle::square(10);
	println!("Squared rectangle: {:#?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
	width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

fn area2(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}

fn can_hold_check() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
