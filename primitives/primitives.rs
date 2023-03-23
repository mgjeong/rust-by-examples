/*
 * reference: https://hanbum.gitbooks.io/rustbyexample/content/primitives.html
 * written by mj
 */

/* Session 01 - basic */

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn session01() {
    println!("-[ session 01 ]--------------------");

    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    println!(
        "{} {} {} {} {}",
        logical, a_float, an_integer, default_float, default_integer
    );

    println!(
        "{:?} {:?} {:?} {:?} {:?}",
        logical, a_float, an_integer, default_float, default_integer
    );

    print_type_of(logical);
    print_type_of(a_float);
    print_type_of(an_integer);
    print_type_of(default_float);
    print_type_of(default_integer);

    println!("\n");
}

/* Session 02 - Literals and operators */

fn session02() {
    println!("-[ session 02 ]--------------------");

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    println!("0011 and 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 or 0101 is {:04b}", 0b0011 | 0b0101);
    println!("0011 xor 0101 is {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is {}", 0x80u32 >> 2);

    println!("one million is written as {}", 1_000_000u32);
    println!("\n");
}

/* Session 03 - Tuples */

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn reverse_alt(pair: (i32, bool)) -> (bool, i32) {
    (pair.1, pair.0)
}

fn get_tuple_length<T: std::fmt::Debug + Sized>(tuple: &T) -> usize {
    let s = format!("{:?}", tuple);
    match s.contains(", ") {
        true => {
            let parts = s.matches(", ").collect::<Vec<&str>>();
            parts.len() + 1
        }
        _ => 1,
    }
}

use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn session03() {
    println!("-[ session 03 ]--------------------");
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);
    println!("size: {}", get_tuple_length(&long_tuple));
    // TODO: access the last one... using iterator?

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));
    println!("the reversed pair is {:?}", reverse_alt(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer {:?}", (5u32));

    let single = (5u32,);
    println!("size of (5u32,) is {}", get_tuple_length(&single));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
    println!("\n");
}

/* Session 04 - Arrays and slices */

use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of hte slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn session04() {
    println!("-[ session 04 ]--------------------");
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("1st elements of the array: {}", xs[0]);
    println!("the size of array: {}", xs.len());

    println!("the array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    
    println!("borrow a section of the array");
    analyze_slice(&ys[1..4]); 
    println!("\n");
}

fn main() {
    session01();
    session02();
    session03();
    session04();
}
