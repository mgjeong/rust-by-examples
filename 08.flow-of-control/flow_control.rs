/* section 01 - if/else */

fn section01() {
    println!("-- section 01 -----------------");
    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        n * 10
    } else {
        println!(", and is a big number, reduce by two");
        n / 2
    };
    println!("{} -> {}", n, big_n);
}

/* section 02 - loop */

fn section02() {
    println!("-- section 02 -----------------");
    let mut count = 0u32;

    println!("let's count until infinity");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("okay, enough!");
            break;
        }
    }
}

/* section 03 - nesting and labels */

fn section03() {
    println!("-- section 03 -----------------");
    'outer: loop {
        println!("outer loop");
        'inner: loop {
            println!("    inner loop");
            // break 'inner;
            break 'outer;
        }
        // println!("    exit inner loop");
    }
    println!("exit outer loop");
}

/* section 04 - returning from loops */
fn section04() {
    println!("-- section 04 -----------------");
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("return result: {}", result);
}

/* section 05 - while */

fn section05() {
    println!("-- section 05 -----------------");
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

/* section 05 - for */

fn section06() {
    println!("-- section 06 -----------------");
    for n in 1..101 {
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

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }
    println!("names: {:?}", names);

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("there is a rustacean among us!"),
            _ => println!("hello {}:", name),
        }
    }
    // println!("names: {:?}", names); // error!

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "there is a rustacean among us!",
            _ => "hello",
        }
    }
    println!("names: {:?}", names);
}

/* section 07 - match */

fn section07() {
    println!("-- section 07 -----------------");
    let number = 19;

    println!("tell me about {}", number);
    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("prime nubmer"),
        13..=19 => println!("teen"),
        _ => println!("ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}

/* sectino 08 - match, destructuring tuples */

fn section08() {
    println!("-- section 08 -----------------");
    // let pair = (0, -2);
    // let pair = ( 2, 0 );
    let pair = (2, 2);

    println!("tell me about {:?}", pair);
    match pair {
        (0, y) => println!("first is `0` and `y` is {}", y),
        (x, 0) => println!("`x` is {}, and the last is `0`", x),
        _ => println!("it doesn't matter"),
    }

    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("first is `0`, `y` is {:?}, `z` is {:?}", y, z),
        (1, ..) => println!("first is `1`, and the rest doesn't matter"),
        (.., 2) => println!("last is `2`, and the rest doesn't matter"),
        (3, .., 4) => println!("frist is `3`, last is `4`, and the rest doesn't matter"),
        _ => println!("it doesn't matter"),
    }
}

/* section 09 - match, destructuring arrays/slices */
fn section09() {
    // let array = [1, -2, 6];
    // let array = [-1, 2, 6];
    // let array = [3, 2, 5];
    let array = [4, 2, 3];

    match array {
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => println!(
            "array[0] = , array[2] = {}, and array[1] was ignored.",
            third
        ),
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {}, and all the other ones were ignored",
            second
        ),
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {}, and the other elements were {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle ={:?}, array[2] = {}",
            first, middle, last
        ),
        // [-1, second] => println!("is it correct"), // error
    }
}

/* section 10 - match, destructuring enums */

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn section10() {
    println!("-- section 09 -----------------");
    //let color = Color::RGB(122, 17, 40);
    let color = Color::CMYK(56, 234, 23, 135);
    println!("What color is it?");

    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) => {
            println!("R: {}, G:{}, B:{}", r, g, b);
        }
        Color::HSV(h, s, v) => {
            println!("H: {}, S:{}, V:{}", h, s, v);
        }
        Color::HSL(h, s, l) => {
            println!("H: {}, S:{}, L:{}", h, s, l);
        }
        Color::CMY(c, m, y) => {
            println!("C: {}, M:{}, Y:{}", c, m, y);
        }
        Color::CMYK(c, m, y, k) => {
            println!("C: {}, M:{}, Y:{}, K:{}", c, m, y, k);
        }
    }
}

/* section 11 - match, destructuring pointers/ref */

fn section11() {
    println!("-- section 11 ---------------");

    let reference: &i32 = &4;

    match reference {
        &val => println!("got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("got a value via dereferecing: {:?}", val),
    }

    let _not_a_referece = 3;
    let &reference = &4;
    match reference {
        val => println!("got a value fia instance: {:?}", val),
    }

    // let &reference = 4;  // error, not allowed.
    let ref reference = 4;
    println!("reference: {}", reference);
    println!("*reference: {}", *reference);
    println!("&reference: {}", &reference);

    println!("reference + 5 = {}", reference + 5); // allowed.
    println!("*reference + 5 = {}", *reference + 5); // standard.

    // println!("&reference + 5 = {}", &reference + 5); // error, not allowed. &reference -> &&{integer}

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("got a reference to a value: {:?}", *r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("add 10 to `mut_value`: {:?}", *m);
            println!("add 10 to `mut_value`: {:?}", m);
        }
    }
    println!("mut_value = {}", mut_value);
    println!("mut_value + 1 = {}", mut_value + 1);
}

/* section 12 - match, destructuring sturcts */

fn section12() {
    println!("-- section 12 ---------------");
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    //let foo: Foo = Foo { x: (1, 2), y: 3 };
    //let foo: Foo = Foo { x: (1, 2), y: 2 };
    //let foo: Foo = Foo { x: (2, 2), y: 2 };
    let foo: Foo = Foo { x: (2, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("first of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, x is {:?}", i),
        // Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // Foo { y, _ } => println!("y = {}, we don't care about x", y), // error.. x should be `..`
        Foo { x: (a, 2), .. } => println!("second of x is 2, a = {}, we don't care of y", a),
        // Foo { x: (a, 2), _ } => println!("second of x is 2, a = {}, we don't care of y", a), // error..
        _ => todo!(),
    }
}

/* section 13 - match, guards */

enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn section13() {
    println!("-- section 13 ---------------");
    // let temperature = Temperature::Celsius(24);
    // let temperature = Temperature::Celsius(19);
    // let temperature = Temperature::Fahrenheit(77);
    let temperature = Temperature::Fahrenheit(69);

    match temperature {
        Temperature::Celsius(t) if t > 20 => println!("{}C is above 20 celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 20 celsius", t),
        Temperature::Fahrenheit(t) if t > 70 => println!("{}F is above 70 fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 70 fahrenheit", t),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("positive"),
        _ => println!("negative"),
    }
}

/* section 14 - match, binding */

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(48)
}

fn section14() {
    println!("-- section 14 ------------------");

    println!("tell me what type of person you are");
    match age() {
        0 => println!("are you kidding?"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teens of age {:?}", n),
        n => println!("I'm an adult of age {:?}", n),
    }

    match some_number() {
        Some(n @ 49) => println!("The answer: {}", n),
        Some(n @ 1..=48) => println!("smaller: {}", n),
        Some(n) => println!("not interesting.. {}", n),
        None => println!("What the..Fxxk"),
    }
}

fn main() {
    section01();
    section02();
    section03();
    section04();
    section05();
    section06();
    section07();
    section08();
    section09();
    section10();
    section11();
    section12();
    section13();
    section14();
}
