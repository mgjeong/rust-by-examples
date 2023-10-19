// ---------------------------------------------------------------------------------
// section 01: if/else

/*
fn main() {
    let n = 20;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}
*/

// ---------------------------------------------------------------------------------
// section 02: loop

/*
fn main() {
    let mut count = 0_u32;
    println!("let's count until infinity");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("ok, that's enough");
            break;
        }
    }
}
*/

// ---------------------------------------------------------------------------------
// section 03: nesting and labels

// It's possible to `break` or `continue` outer loops when dealing with nested loops.
// In these cases, the loops must be annotated with some `'label``, and the label must be passed
// to the `break`/`continue` statement.

/*
fn main() {
    'outer: loop {
        println!("enter the outer loop");

        'inner: loop {
            println!("enter the inner loop");
            // break; // this would break only the inner loop

            break 'outer; // this break the outer loop
        }

        println!("this point will never be reached");
    }

    println!("exit the outer loop");
}
*/

// ---------------------------------------------------------------------------------
// section 04: returning from the loops

// One of the uses of a `loop` is to retry an operation until it succeeds.
// If the operation returns a value though, you might need to pass it to the rest of the code:
// put it after the `break`, and it will be returned by the `loop` expression.

/*
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {}", result);
}
*/

// ---------------------------------------------------------------------------------
// section 05: while

/*
fn main() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}
*/

// ---------------------------------------------------------------------------------
// section 06: for loops

/*
fn main() {
    // for in
    // for n in 1..101 {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // iter
    let names1 = vec!["Bob", "Frank", "Ferris"];
    for name in names1.iter() {
        match name {
            &"Ferris" => println!("there is a restacean among us!"),
            _ => println!("hello {}", name),
        }
    }

    println!("names: {:?}", names1);

    // into_iter
    let names2 = ["Bob", "Frank", "Ferris"].to_vec();
    for name in names2.into_iter() {
        match name {
            "Ferris" => println!("there is a restacean among us!"),
            _ => println!("hello {}", name),
        }
    }

    // println!("names: {:?}", names2); // ERROR... the ownership moved..

    // iter_mut
    let mut names3 = vec!["Bob", "Frank", "Ferris"];
    for name in names3.iter_mut() {
        *name = match name {
            &mut "Ferris" => "there is a restacean among us!",
            _ => "hello",
        }
    }

    println!("names: {:?}", names3);

    // experimental
    let names4 = vec!["Bob", "Frank", "Ferris"];
    for name in &names4 {
        match name {
            &"Ferris" => println!("there is a restacean among us!"),
            _ => println!("hello {}", name),
        }
    }

    println!("names: {:?}", names4);
}
*/

// ---------------------------------------------------------------------------------
// section 07: match

/*
fn main() {
    let number = 13;

    println!("tell me about {}", number);
    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("prime number"),
        13..=19 => println!("a teen"),
        _ => println!("ain't special",),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
*/

// ---------------------------------------------------------------------------------
// section 08: match - destructuring

// A match block can destructure items in a variety of ways.
// - Destructuring Tuples
// - Destructuring Arrays and Slices
// - Destructuring Enums
// - Destructuring Pointers
// - Destructuring Structures

// section 08.01: tuples
/*
fn main() {
    let triple = (1, -2, 3);

    println!("tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("first is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("first is `1` and the rest doesn't matter."),
        (.., 2) => println!("latst is `2` and the rest doesn't matter. "),
        (3, .., 4) => println!("first is `3`, last is `4`, and the rest doesn't matter."),
        _ => println!("it doesn't matter what they are"),
    }
}
*/

// secton 08.02: arrays/slices
/*
fn main() {
    // let array = [1, -2, 6];
    // let array = [0, -2, 6];
    // let array = [-1, -2, 6];
    // let array = [3, 4, 5];
    let array = [8, 4, 5];

    match array {
        // f: first, s: second, t: third
        [0, s, t] => println!("array[0] = 0, array[1] = {}, array[2] = {}", s, t),

        [1, _, t] => println!("array[0] = 1, array[2] = {} and array[1] was ignored", t),

        [-1, s, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            s
        ),

        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, s, t @ ..] => println!(
            "array[0] = 3, array[1] = {}, and the other elements were {:?}",
            s, t
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        // f: first, m: middle, l: last
        [f, m @ .., l] => println!("array[0] = {}, middle = {:?}, array[2] = {}", f, m, l),
    }
}
*/

// section 08.03: enums
/*
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    use crate::Color::*;

    let color = Color::RGB(122, 17, 40);

    println!("what color is it?");
    match color {
        Red => println!("Red"),
        Green => println!("Green"),
        Blue => println!("Blue"),
        RGB(r, g, b) => println!("R: {}, G: {}, B: {}", r, g, b),
        HSV(h, s, v) => println!("H: {}, S: {}, V: {}", h, s, v),
        HSL(h, s, l) => println!("H: {}, S: {}, L: {}", h, s, l),
        CMY(c, m, y) => println!("C: {}, M: {}, Y: {}", c, m, y),
        CMYK(c, m, y, k) => println!("C: {}, M: {}, Y: {}, K: {}", c, m, y, k),
    }
}
*/

// section 08.04: ponters/ref

/*

// dereferencing uses *
// destructuring uses &, ref, and ref mut

fn main() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("got a value via destructuring: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let not_a_reference = 3;
    match not_a_reference {
        val => println!("got a value via destructuring: {:?}", val),
    }

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref is_a_reference = 3;
    match is_a_reference {
        &val => println!("got a value via destructuring: {:?}", val),
    }

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mutable_value = 6;

    match value {
        ref r => println!("got a reference to a value: {:?}", r),
    }

    match mutable_value {
        ref mut m => {
            // got a reference
            *m += 10;
            println!("added 10 to mutable value: {:?}", *m);
        }
    }
    println!("mutable value: {:?}", mutable_value);
}
*/

// section 08.05: structs

/*
fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("first of x is 1, b = {}, y = {}", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };
    let Foo { x: x0, y: y0 } = faa;
    println!("outside: x0 = {x0:?}, y0 = {y0}");
    println!("Outside: x0 = {:?}, y0 = {}", x0, y0);

    let fbb = Foo { x: (2, 1), y: 5 };
    let fcc = fbb;
    println!("{:?} and {}", fcc.x, fcc.y);
}
*/

// ---------------------------------------------------------------------------------
// section 09: match - guards

/*
#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(24);

    match temperature {
        Temperature::Celsius(c) if c > 30 => println!("{}C is above 30 celsius", c),
        Temperature::Celsius(c) => println!("{}C is below 30 celsius", c),
        Temperature::Fahrenheit(f) if f > 86 => println!("{}F is above 86 fahrenheit", f),
        Temperature::Fahrenheit(f) => println!("{}F is below 86 fahrenheit", f),
    }

    // Note that the compiler won't take guard conditions into account when checking
    // if all patterns are covered by the match expression.
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Smaller than zero"), // REQUIRED.!!!!
    }
}
*/

// ---------------------------------------------------------------------------------
// section 10: match - binding

// Indirectly accessing a variable makes it impossible to branch and use that variable
// without re-binding. `match`` provides the `@`` sigil for binding values to names:

/*
fn age() -> u32 {
    15
}

fn main() {
    println!("tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        // 1..=12 => println!("I'm a child of age {:?}", ?????), // impossible
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an adult of age {:?}", n),
    }

    match some_number() {
        Some(n @ 42) => println!("The anser: {}", n),
        Some(n) => println!("not interesting.. {}", n),
        _ => (),
    }
}

// You can also use binding to "destructure" enum variants, such as Option:

fn some_number() -> Option<u32> {
    Some(44)
}
*/

// ---------------------------------------------------------------------------------
// section 11: if let

/*
 enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(i) = number {
        println!("matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("matched {:?}", i);
    } else {
        println!("didn't match a number");
    }

    let i_like_letters = false;

    if let Some(i) = emotion {
        println!("matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number");
    } else {
        println!("I don't like letters");
    }

    // if let can be used to match any enum value
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    } else {
        println!("a isn't foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    } else {
        println!("b isn't foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(_value @ 100) = c {
        println!("c is one hundres");
    } else if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // if let c = Foo::Qux(value) { // WRONG expression
    //     println!("c is {}", value);
    // }
}
*/

// challenge
/*
enum Foo {
    Bar,
}

fn main() {
    let a = Foo::Bar;

    if let Foo::Bar = a {
        println!("a is foobar");
    }
}
*/

// ---------------------------------------------------------------------------------
// section 12: let else

/*
use core::panic;
use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    // let else..
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };

    // previous
    /*
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment item pair: '{s}'"),
    };

    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse integer: '{count_str}'");
    };
    */

    (count, item)
}

fn main() {
    // assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    let (count, item) = get_count_item("3 chairs");
    println!("{} and {}", count, item);
}
*/

// ---------------------------------------------------------------------------------
// section 13: while let

fn main() {
    let mut optional = Some(0);

    // previous...
    //
    // loop {
    //     match optional {
    //         Some(i) => {
    //             if i > 9 {
    //                 println!("Greater than 9, quit!");
    //                 optional = None;
    //             } else {
    //                 println!("`i` is `{:?}`. try again", i);
    //                 optional = Some(i + 1);
    //             }
    //         }
    //         _ => {
    //             break;
    //         }
    //     }
    // }

    while let Some(i) = optional {
        if i > 9 {
            println!("greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. try again", i);
            optional = Some(i + 1);
        }
    }
}
