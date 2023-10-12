// ------------------------------------------------------------
// section 1: basic types
/*
fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_flaot = 3.0; // f64
    let default_integer = 7; // i32

    let mut mutable = 12; // i32
}
*/

//---------------------------------------------------------------------
// section 2: literals and operations
/*
fn main() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("true AND false is {}", true && false);
    println!("ture OR flase is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}
*/

//-----------------------------------------------------------------
// section 3: Tuples
/*
use std::fmt::{self, Display, Formatter};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("reverse pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,)); // required comma for single element tuple
    println!("just an interger: {:?}", (5u32));

    // tuples can be destructed to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
*/

// -------------------------------------------------------------------
// section 4: arrays and slices

use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5]; // fixed size array
    let ys: [i32; 500] = [0; 500]; // initialize with same value

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());

    // array is allocated in the stack
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]); // expect, [0, 0, 0]

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);
    assert_eq!(empty_array.len(), 0);

    // Arrays can be safely accessed using `.get`, which returns an `Option`.
    // Or used with `.expect()` if you would like the program to exit with a nice message

    // let index: usize = xs.len() + 1;
    // println!(
    //    "{}: {}",
    //     index,
    //     xs.get(index).expect("the index is to far!")
    // );

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is to far!", i),
        }
    }

    // println!("{}", xs[5]); // panic!
}
