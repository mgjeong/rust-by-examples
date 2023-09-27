/**
 * @package     helloworld
 * @file        main.rs
 * @author      mj < mg.jeong@gmail.com >
 */
/*
/// main
fn main() {
    // section 01 - print
    println!("Hello, world!");
    println!("I'm new rustacean!");

    // -----------------------------------------------------------------------------
    // section 02 - formatted print
    println!("{} days", 31i32); // can add type using suffix
    println!("{0}, this is {1}. {1}, this is {0}", "monster", "zombie"); // location

    // named parameter
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // output type format
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // align
    println!("{number:>width$}: right align", number = 1, width = 7);
    println!("{number:>7}: right align", number = 1);
    println!("{number:<width$}: left align", number = 1, width = 7);
    println!("{number:^width$}: center align", number = 1, width = 7);

    let width: usize = 9;
    println!("{number:->width$}: right align", number = 1);
    println!("{number:-<width$}: left align", number = 1);
    println!("{number:-^width$}: center align", number = 1);

    // the number of arguments
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // `std::fmt` has lots of traits, such as `fmt::Debug`, `fmt::Display`
    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("This sturct `{}` won't print...", Structure(3));

    // Activities
    let pi: f64 = 3.1415926;
    println!("{:>9.3}", pi);

    // -----------------------------------------------------------------------------
    // section 03 - debug
    // Derive the `fmt::Debug` implementation for `Structure`.
    // `Structure` is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printalbe also.
    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} month is a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    println!(
        "{1} {0} is the {actor} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));
*/


// ---------------------------------------------------------------------------------------
// section 04 - display
/*
use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct XPoint {
    x: f64,
    y: f64,
}

impl fmt::Display for XPoint {
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
        write!(f, "{} + j{}", self.real, self.imag)
    }
}

fn main() {
    println!("{}", Structure(16));

    let minmax = MinMax(2, 8);
    println!("Compare MinMax structure");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "the big range is {big} and the small range is {small}",
        big = big_range,
        small = small_range
    );

    let p: XPoint = XPoint { x: 2.5, y: 1.6 };
    // let p2 = XPoint(3.5, 2.7); // error !!

    println!("Compare XPoint");
    println!("Display: {}", p);
    println!("Debug: {:?}", p);

    let c: Complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Complare Complex");
    println!("Display: {}", c);
    println!("Debug: {:?}", c);
}
*/
// ---------------------------------------------------------------------------------------
// section 05 - testcase: List
/*
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Dereference `self` and create reference to `vec` via destructuring.
        let List(ref vec) = *self;
        write!(f, "[")?;

        // Iterate over `vec` in `v` while enumerating the iteration count in `count`
        for (count, v) in vec.iter().enumerate() {
            // for every element except the first, add a comma before calling `write!`
            // Use `try!` to return on errors.
            if count != 0 {
                write!(f, ",  ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
*/
// ---------------------------------------------------------------------------------------
// section 06 - formatting
use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{:<20}: {:.3}{} {:.3}{}",
            self.name, self.lat, lat_c, self.lon, lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({r}, {g}, {b}) 0x{r:02X}{g:02X}{b:02X}",
            r = self.r,
            g = self.g,
            b = self.b
        )
    }
}

fn main() {
    let cities = vec![
        City {
            name: "Dublin",
            lat: 53.234235,
            lon: -6.2534325,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ];

    //for city in cities {
    //    println!("{}", city);
    //}

    for city in cities.iter() {
        println!("{}", *city);
    }

    for city in cities {
        println!("{}", city);
    }

    let colors = vec![
        Color {
            r: 128,
            g: 255,
            b: 90,
        },
        Color { r: 0, g: 3, b: 254 },
        Color { r: 0, g: 0, b: 0 },
    ];

    for color in colors.iter() {
        println!("{}", color);
    }
}
