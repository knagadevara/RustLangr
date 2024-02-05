/*  
	All the variables by default are Immutable untill unless they are declared with 'mut' keyword.
	Rust will infer types automatically while compiling for regular variables.
  It is still required to declare the type for constants which should be identified with all CAPS.
*/

pub fn var_like_rust(){

	// Will be infered by compiler
	let is_indian = true; // immutable
	let my_team = 'X'; // immutable
	println!("\nIndian: {0}\nTeam: {1}", is_indian, my_team);

	// Mutablity
	let mut alfa = -1234;	
	println!("\nMutable Integer Before: {0}", alfa);
	// As on 2024 rust does not support prefix or postfix operations
	// so no increment or decrement with ++, --
	// instead do it the normal way around.
	// Substraction -=, Addition +=, Multiplication *=
	alfa += 1;
	println!("\nMutable Integer After: {0}", alfa);

	// Constant - All caps identifier and explicit type declaration
	const SPEED_OF_LIGHT: f64 = 2.99792458e8;
	println!("\nSpeed of Light: {0}", SPEED_OF_LIGHT);
	// Fact written in detail for better understanding and clarity.
	// '_' for readablity and seperation, not to be considerd as decimal point.
	const _SECONDS_IN_HOUR: i32 = 3_600;
	const SECONDS_IN_DAY: i32 = 24 * _SECONDS_IN_HOUR;
	println!("\nSeconds in Day: {0}", SECONDS_IN_DAY);

	// Type casting
	let _salary_in_int; // If not being used start var-name with '_'
	_salary_in_int = 2000;
	let salary_in_float = _salary_in_int as f32;
	println!("\nSalary: {0}" , salary_in_float);

	// Redeclaring wihout "mut" keyword is possible but not recommended.
	let real_number = "12345";
	println!("\nAs String: {}", real_number);
	let real_number = 12345;
	println!("\nNext Number: {}", real_number + 1);

}