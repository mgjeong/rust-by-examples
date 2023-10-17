// -------------------------------------------------------------------------
// section 1: casting
// Rust provides no implicit type conversion (coercion) between primitive types.
// But, explicit type conversion (casting) can be performed using the `as`` keyword.
//
// Rules for converting between integral types follow C conventions generally,
// except in cases where C has undefined behavior. The behavior of all casts between integral types is well defined in Rust.
/*
#![allow(overflowing_literals)]
fn main() {
    let decimal = 65.4321_f32;
    // let integer: u8 = decimal; // ERROR
    let integer = decimal as u8;
    let character = integer as char;
    // let character = decimal as char; // ERROR.. a float cannot be directly converted to a char

    println!("casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    println!("1000 as a u16 is : {}", 1000 as u16);
    // 1000 - 256 - 256 - 256 = 232;
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255;
    println!("  -1 as a u8 is : {}", (-1_i8) as u8);
    println!("1000 mod 256 is : {}", 1000 % 256);

    println!(" 128 as a i16 is : {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    println!(" 300.0 as u8 is : {}", 300.0f32 as u8);
    println!("-100.0 as u8 is : {}", -100.0f32 as u8);
    println!("   non as u8 is : {}", f32::NAN as u8);

    unsafe {
        println!(" 300.0 as u8 is : {}", 300.0f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is : {}", (-100.0f32).to_int_unchecked::<u8>());
        println!("   non as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
*/

// -------------------------------------------------------------------------
// section 2: literals

// Numeric literals can be type annotated by adding the type as a suffix. As an example,
// to specify that the literal 42 should have the type i32, write 42i32.
// The type of unsuffixed numeric literals will depend on how they are used. If no constraint exists,
// the compiler will use i32 for integers, and f64 for floating-point numbers.
/*
use std::mem::size_of_val;

fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", size_of_val(&x));
    println!("size of `y` in bytes: {}", size_of_val(&y));
    println!("size of `z` in bytes: {}", size_of_val(&z));
    println!("size of `i` in bytes: {}", size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
*/

// -------------------------------------------------------------------------
// section 3: inference

/*
fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

     // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}
*/

// -------------------------------------------------------------------------
// section 4: aliasing

// The `type` statement can be used to give a new name to an existing type.
// Types must have `UpperCamelCase` names, or the compiler will raise a warning.
// The exception to this rule are the primitive types: `usize`, `f32`, etc./* */
/* */
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
