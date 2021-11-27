#[derive(Debug)]
enum UsState {
	Alaska,
	Alabama,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn main() {

}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("The quarter is from the state {:?}!", state);
			25
		}
	}
}