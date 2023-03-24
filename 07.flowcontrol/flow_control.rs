/* section 01 - if/else */

fn section01() {
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
    'outer: loop {
        println!("outer loop");
        'inner: loop {
            println!("    inner loop");
            // break 'inner;
            break 'outer;
        }
        println!("    exit inner loop");
    }
    println!("exit outer loop");
}

/* section 04 - while */

fn section04() {
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

/* section 05 - for and range */

fn section05() {
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
}

/* section 06 - match */

fn section06() {
    let number = 19;

    println!("tell me about {}", number);
    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("prime nubmer"),
        13...19 => println!("teen"),
        _ => println!("ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}

/* sectino 07 - destructuring by match, tuple */

fn section07() {
    // let pair = (0, -2);
    // let pair = ( 2, 0 );
    let pair = (2, 2);

    println!("tell me about {:?}", pair);
    match pair {
        (0, y) => println!("first is `0` and `y` is {}", y),
        (x, 0) => println!("`x` is {}, and the last is `0`", x),
        _ => println!("it doesn't matter"),
    }
}

/* section 08 - desturcuting by match, enums */

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

fn section08() {
    let color = Color::RGB(122, 17, 40);
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

fn main() {
    section01();
    section02();
    section03();
    section04();
    section05();
    section06();
    section07();
    section08();
}
