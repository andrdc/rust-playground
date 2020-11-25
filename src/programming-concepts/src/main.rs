fn main() {
	println!("--- Mutable variables ---");
	let mut x = 5;
	println!("The value of x is: {}", x);
	x = 6;
	println!("The value of x is: {}", x);

	println!("--- Shadowing ---");
	let y = 5;
	let y = y + 1;
	let y = y * 2;
	println!("The value of y is: {}", y);
	let spaces = "   ";
	let spaces = spaces.len();
	println!("The value of spaces is: {}", spaces);

	println!("--- Data Types ---");
	// For signed integers the range is -(2^(n - 1)) to 2^(n - 1) - 1
	// For unsigned integers the range is 0 to 2^n - 1
	let eight_bit_signed_integer_negative: i8 = -128;
	let eight_bit_signed_integer_positive: i8 = 127;
	println!("8 bit signed integers go from {} to {}",
			 eight_bit_signed_integer_negative,
			 eight_bit_signed_integer_positive);
	let sixteen_bit_signed_integer_negative: i16 = -32768;
	let sixteen_bit_signed_integer_positive: i16 = 32767;
	println!("16 bit signed integers go from {} to {}",
			 sixteen_bit_signed_integer_negative,
			 sixteen_bit_signed_integer_positive);
	let thirtytwo_bit_signed_integer_negative: i32 = -2147483648;
	let thirtytwo_bit_signed_integer_positive: i32 = 2147483647;
	println!("32 bit signed integers go from {} to {}",
			 thirtytwo_bit_signed_integer_negative,
			 thirtytwo_bit_signed_integer_positive);
	let sixtyfour_bit_signed_integer_negative: i64 = -9223372036854775808;
	let sixtyfour_bit_signed_integer_positive: i64 = 9223372036854775807;
	println!("64 bit signed integers go from {} to {}",
			 sixtyfour_bit_signed_integer_negative,
			 sixtyfour_bit_signed_integer_positive);
	let onehundred_and_twentyeight_bit_signed_integer_negative: i128 = -170141183460469231731687303715884105728;
	let onehundred_and_twentyeight_bit_signed_integer_positive: i128 = 170141183460469231731687303715884105727;
	println!("128 bit signed integers go from {} to {}",
			 onehundred_and_twentyeight_bit_signed_integer_negative,
			 onehundred_and_twentyeight_bit_signed_integer_positive);
	let thirtytwo_bit_floating: f32 = 5.54;
	let sixtyfour_bit_floating: f64 = 107.42;
	println!("32 bit floating point number {}", thirtytwo_bit_floating);
	println!("64 bit floating point number {}", sixtyfour_bit_floating);
	let bool_variable: bool = true;
	println!("Bool variable is {}", bool_variable);
	let emoji = '😻';
	println!("Emoji is a char varbiable: {}", emoji);
}
