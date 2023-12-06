// ----------------------------------------------------------------------
// section 01 - box, stack and heap
/*

// All values in Rust are stack allocated by default. Values can be boxed (allocated on the heap)
// by creating a `Box<T>`. A box is a smart pointer to a heap allocated value of type `T`.
// When a box goes out of scope, its destructor is called, the inner object is destroyed,
// and the memory on the heap is freed.
//
// Boxed values can be dereferenced using the * operator; this removes one layer of indirection.

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// A Rectangle can be specified by where its top left and bottom right corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // stack allocated
    let point = origin();
    let rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // heap allocated
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // the output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!(
        "point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );

    println!(
        "rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    // box size == pointer size
    println!(
        "boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );

    println!(
        "boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // copy the data constrained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!(
        "unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );

    // println!(
    //     "size_of_val(boxed_point) = {}",
    //     mem::size_of_val(boxed_point)
    // ); // ERROR...
}
*/

// ----------------------------------------------------------------------
// section 02 - vectors
/*

// Vectors are re-sizable arrays. Like slices, their size is not known at compile time,
// but they can grow or shrink at any time. A vector is represented using 3 parameters:
// - pointer to the data
// - length
// - capacity
// The capacity indicates how much memory is reserved for the vector.
// The vector can grow as long as the length is smaller than the capacity.
// When this threshold needs to be surpassed, the vector is reallocated with a larger capacity.

fn main() {
    // iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("collected (0..10) into {:?}", collected_iterator);

    // `vec!` macro
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);
    xs.push(4);
    println!("Initial vector: {:?}", xs);

    println!("vector length: {}", xs.len());

    println!("second element: {}", xs[1]);

    println!("pop last element: {:?}", xs.pop());

    // out of bounds
    // println!("forth element: {}", xs[3]); // ERROR...

    // iterator...
    println!("contents of xs: ");
    for x in xs.iter() {
        println!(" > {}", x);
    }

    // iterate with count enumeration
    for (i, x) in xs.iter().enumerate() {
        println!("in position {} we have value {}", i, x);
    }

    // mutable iteration
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("updated vector: {:?}", xs);
}
*/

// ----------------------------------------------------------------------
// section 03 - strings
/*

// There are two types of strings in Rust: `String` and `&str`.
//
// A `String` is stored as a vector of bytes (`Vec<u8>`), but guaranteed to always be a valid UTF-8 sequence.
// `String` is heap allocated, growable and not null terminated.
//
// `&str` is a slice (`&[u8]`) that always points to a valid UTF-8 sequence,
// and can be used to view into a `String`, just like `&[T]` is a view into `Vec<T>`.

fn main() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    // let pangram = "the quick brown fox jumprs over the lazy dog";
    println!("Pangram: {}", pangram);

    // iteration over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // copy chars into a vector, sort, and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("chars: {:?}", chars);

    // create an empty string
    let mut string = String::new();
    for c in chars {
        // insert a char
        string.push(c);
        // inserat a string
        string.push_str(", ");
    }
    println!("string: {}", string);

    // the trimmed string is a slice to the original string. hence no new allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("used characters: {}", trimmed_str);

    // heap allocate a string
    let alice = String::from("I like dogs");
    // allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
*/

/*
// Literals and escapers

use std::{collections::btree_map::Keys, str};

fn main() {
    // You can use escapes to write bytes by their hexadecimal values...
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    // --------------
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // ------------------------------------
    // Note that this is not actually a `&str`
    let bytestring: &[u8; 21] = b"this is a byte string";

    // Byte arrays don't have the `Display` trait, so printing them is a bit limited
    println!("A byte string: {:?}", bytestring);

    // Byte strings can have byte escapes...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...but no unicode escapes
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    // Raw byte strings work just like raw strings
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // Converting a byte array to `str` can fail
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // Byte strings don't have to be UTF-8
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // But then they can't always be converted to `str`
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}
*/

// ----------------------------------------------------------------------
// section 04 - options
/*

fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    let mut result: Option<i32> = None;
    if divisor != 0 {
        result = Some(dividend / divisor);
    }
    return result;

    // if divisor == 0 {
    //     None
    // } else {
    //     Some( dividend / divisor )
    // }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed.", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient);
        }
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;
    let optional_float = Some(0f32);

    println!(
        "{:?} unwraps to {:?}",
        optional_float,
        optional_float.unwrap()
    );
}
*/

// ----------------------------------------------------------------------
// section 05 - result
/*

mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    println!("{}", op(1.0, 10.0));
}
*/

// ----------------------------------------------------------------------
// section 06 - result: ?
/*
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    fn op_(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;
        let log = ln(ratio)?;
        sqrt(log)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Ok(value) => println!("{}", value),
            Err(why) => panic!(
                "{}",
                match why {
                    MathError::DivisionByZero => "divide by zero",
                    MathError::NegativeSquareRoot => "square of negative",
                    MathError::NonPositiveLogarithm => "logarithm of non-positive",
                }
            ),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
*/

// ----------------------------------------------------------------------
// section 07 - panic!
/*

// The `panic!` macro can be used to generate a panic and start unwinding its stack.
// While unwinding, the runtime will take care of freeing all the resources owned by the thread
// by calling the destructor of all its objects.
//
// Since we are dealing with programs with only one thread, `panic!` will cause the program
// to report the panic message and exit.

// Re-implementation of integer division (/)
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Division by zero triggers a panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// The `main` task
fn main() {
    // Heap allocated integer
    let _x = Box::new(0i32);

    // This operation will trigger a task failure
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` should get destroyed at this point
}

// ```
// $ rustc panic.rs && valgrind ./panic
// ==4401== Memcheck, a memory error detector
// ==4401== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
// ==4401== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
// ==4401== Command: ./panic
// ==4401==
// thread '<main>' panicked at 'division by zero', panic.rs:5
// ==4401==
// ==4401== HEAP SUMMARY:
// ==4401==     in use at exit: 0 bytes in 0 blocks
// ==4401==   total heap usage: 18 allocs, 18 frees, 1,648 bytes allocated
// ==4401==
// ==4401== All heap blocks were freed -- no leaks are possible
// ==4401==
// ==4401== For counts of detected and suppressed errors, rerun with: -v
// ==4401== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
// ```
*/

// ----------------------------------------------------------------------
// section 08 - hashmap
/*
// Where vectors store values by an integer index, `HashMap`s store values by key.
// `HashMap` keys can be booleans, integers, strings, or any other type that implements
// the `Eq` and `Hash` traits. More on this in the next section.
//
// Like vectors, `HashMap`s are growable, but HashMaps can also shrink themselves
// when they have excess space. You can create a HashMap with a certain starting capacity
// using `HashMap::with_capacity(uint)`, or use `HashMap::new()` to get a HashMap with
// a default initial capacity (recommended).

use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot blar blar",
        "645-7698" => "Hello, this is Mr. Awesomes's pizza. blar...",
        _ => "Hi! who is this again?",
    }
}

fn main() {
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7698");
    contacts.insert("Katie", "234-1451");
    contacts.insert("Robert", "123-5325");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("calling Daniel: {}", call(number)),
        _ => println!("don't have daniel's number"),
    }

    contacts.insert("Daniel", "932-2345");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("calling Ashley: {}", call(number)),
        _ => println!("don't have ashley's number"),
    }

    contacts.remove(&"Ashley");

    for (contact, &number) in contacts.iter() {
        println!("calling {}: {}", contact, call(number));
    }
}
*/

// ----------------------------------------------------------------------
// section 09 - hashmap: alternative/custom key types
/*
 */
// Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`. This includes:
// - `bool` (though not very useful since there is only two possible keys)
// - `int`, `uint`, and all variations thereof
// - `String` and `&str` (protip: you can have a `HashMap` keyed by `String` and call `.get()` with an `&str`)
// Note that `f32` and `f64` do not implement `Hash`, likely because floating-point precision errors
// would make using them as hashmap keys horribly error-prone.
//
// All collection classes implement `Eq` and `Hash` if their contained type also respectively implements
// `Eq` and `Hash`. For example, `Vec<T>` will implement `Hash` if `T` implements `Hash`.
//
// You can easily implement `Eq` and `Hash` for a custom type with just one line: `#[derive(PartialEq, Eq, Hash)]`
//
// The compiler will do the rest. If you want more control over the details,
// you can implement `Eq` and/or `Hash` yourself. This guide will not cover the specifics of implementing `Hash`.
//
// To play around with using a `struct` in `HashMap`, let's try making a very simple user logon system:

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_login<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("username: {}", username);
    println!("passowrd: {}", password);
    println!("Atempting login...");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("login success");
            println!("name: {}", account_info.name);
            println!("email: {}", account_info.email);
        }
        _ => println!("login failed..."),
    }
}

fn main() {
    let mut accounts: Accounts = Accounts::new();

    let account = Account {
        username: "asdf",
        password: "qwer",
    };

    let account_info = AccountInfo {
        name: "ASDF",
        email: "asdf@email.com",
    };

    accounts.insert(account, account_info);

    try_login(&accounts, "asdf", "qewr");

    try_login(&accounts, "asdf", "qwer");
}

// ----------------------------------------------------------------------
// section 10 - hashmap: hashset
/*
 */

// ----------------------------------------------------------------------
// section 11 - rc
/*
 */

// ----------------------------------------------------------------------
// section 12 - arc
/*
 */
