fn main() {
	let gifts = [
		"partridge in a pear tree",
		"turtle doves",
		"French hens",
		"calling birds",
		"gold rings",
		"geese a-laying",
		"swans a-swimming",
		"maids a-milking",
		"ladies dancing",
		"lords a-leaping",
		"pipers piping",
		"drummers drumming"
	];
	let ordinal_numbers = [
		"first",
		"second",
		"third",
		"fourth",
		"fifth",
		"sixth",
		"seventh",
		"eighth",
		"ninth",
		"tenth",
		"eleventh",
		"twelfth"
	];
	let mut day = 1;
	for ordinal_day in ordinal_numbers {
		println!("On the {} day of Christmas my true love love sent me",
			ordinal_day);
		println!("{} {}", day, gifts[day - 1]);
		for gift_index in (0..day - 1).rev() {
			println!("And {} {}", gift_index + 1, gifts[gift_index]);
		}
		println!("");
		day += 1;
	}
}
