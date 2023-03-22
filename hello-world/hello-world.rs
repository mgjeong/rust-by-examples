/* Session 01 - hello world */
/*
fn main() {
    println!("hello world");
    println!("I'm Rustacean");
}
*/

/* Session 02 -comment */
/*
//! How to write comment
//!
//! description blar blar
//! description blar blar
//! description blar blar
fn main() {
    // line comment
    /*
     * block comment
     */
    /*
      block comment
    */

    let x = 5;
    println!("x is {}", x);
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
*/

/* Session 03 - Formated print */
/*
fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        verb = "jumps over",
        object = "the lazy dog",
        subject = "the quick brown fox"
    );

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:width$}", number = 1, width = 6);
    println!("{number:width$}", number = 1, width = 8);
    println!("{number:4}", number = 1);

    println!("My name is {0}, {1} {0}", "bond", "James");

    println!("hello {:<5}!", "x");
    println!("hello {:-<5}!", "x");
    println!("hello {:-^5}!", "x");
    println!("hello {:>5}!", "x");

    let pi = 3.1415926;
    println!("Pi = {:.2}", pi);
}
*/

/* Session 04 - Debug */

use std::fmt;

// struct UnPrintable(i32);

// #[derive(Debug)]
// struct Deep(UnPrintable); // error

#[derive(Debug)]
struct DebugPrintable(i32);

impl fmt::Display for DebugPrintable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r: fmt::Result = write!(f, "{}", self.0);
        return r;

        // or simpley
        // write!(f, "{}", self.0);
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} + i{1}", self.real, self.imag)
    }
}

fn main() {
    println!("{:?}", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    // println!("Now {:?} will print!", UnPrintable(3)); // error
    println!("Now {:?} will print!", DebugPrintable(7));
    // println!("Now {:?} will print!", Deep(UnPrintable(9))); // error
    println!("the value {}", DebugPrintable(5));

    let min_max: MinMax = MinMax(0, 14);
    println!("Display: {}", min_max);
    println!("Debug:   {:?}", min_max);

    let big_range = MinMax(-100, 100);
    let small_range = MinMax(-10, 10);
    println!(
        "The big range is {big} and the small range is {small}",
        small = small_range,
        big = big_range
    );

    let point: Point = Point { x: 3.3, y: 5.5 };
    println!("Display: {}", point);
    println!("Debug:   {:?}", point);

    let complex: Complex = Complex {
        real: 2.5,
        imag: 1.8,
    };
    println!("Display: {}", complex);
    println!("Debug:   {:?}", complex);
}
