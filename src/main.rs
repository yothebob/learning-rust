
const HOUR_MINUTES :i32 = 60;

fn print_labeled_hours(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}


fn block_expression() {
    let y = {
	let x = 5;
	x + 1
    };
    println!("the value of y is: {y}");
}

fn five() -> i32 {
// you signify return type by -> syntax. last statement is allways returned (you can still use return for early return)
    5
}

fn plus_one(x: i32) -> i32 { x + 1 } // for some reason this does not return the value if you have a semicolon?

fn conditionals() {
    let number = 5;
    // if number {do thing} THIS DOES NOT WORK, conditionals need to be a boolean, try...
    if number != 0 {
	println!("this worked");
    }
    let testing_val = true;
    let number = if testing_val { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // this DOES NOT WORK, you cannot mix types
}

fn loops() {
    // loop {
    // 	println!("this is an endless loop");
    // }

    let mut counter = 0;
    let res = loop {
	counter += 1;
	if counter == 10 {
	    break counter; //10
	    // break counter * 2; //20
	}
    };
    println!("the result of counter is : {counter}");
    println!("the result of res is : {res}");

    let a = [10,20,30,40,50];
    for ele in a {
	println!("{ele}");
    }
    for num in (1..5) { println!("{num}"); }
    for rnum in (1..5).rev() { println!("{rnum}"); }
}


fn five_value() {
    let five_val = five();
    println!("five function returned: {five_val}");
    let five_val_plus_one = plus_one(five_val);
    println!("five function returned: {five_val_plus_one}");
}

fn shadow() {
    // shadowing, you can declare a new variable with the same name as a previous var
    // the first will be shadowed by the first 

    let x = 5;
    let x = x + 1;
    {
	let x = x * 2;
	println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in the outer scope is: {x}");

    //example of good use of shadowing
    let spaces = "     ";
    let spaces = spaces.len();
    
    println!("spaces length: {spaces}");
}

fn data_types() {
        // 8-bit	i8	u8
	// 16-bit	i16	u16
	// 32-bit	i32	u32
	// 64-bit	i64	u64
	// 128-bit	i128	u128
	// arch	isize	usize
	// Decimal	98_222
	// Hex	0xff
	// Octal	0o77
	// Binary	0b1111_0000
	// Byte (u8 only)	b'A'
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    let tup: (i32, f64, u8) = (500, 6.4, 1); // once declared tuples cannot grow/shrink
// to get indiviual values out of a tuple you can use pattern matching to destruct the tuple value
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    // you can also access a tuple element directly with a period followed by the index of the value
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    //arrays kinda suck, same datatype needed and cannot change length
    // let a = [1, 2, 3, 4, 5]; OR
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_area(rectangle: &Rectangle) -> u32 {
     rectangle.width * rectangle.height
}

fn main() {
    println!("Constants are scoped globally minutes in hour = {HOUR_MINUTES}");

    println!("Variables start as unmutible");
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
    println!("SHADOWING:\n");
    shadow();
    print_labeled_hours(x, 'h');
    five_value();
    loops();

    let rect1 = Rectangle{
	width: 50,
	height: 30
    };
    println!("rect is {:?}", rect1);
    let are: u32 = struct_area(&rect1);
    println!("area is {are}");
}

