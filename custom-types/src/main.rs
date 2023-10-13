// -----------------------------------------------------------------------
// section 01 - structures

/*
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> Result<f32, &'static str> {
    let Rectangle {
        top_left,
        bottom_right,
    } = rect;

    let Point { x: x1, y: y1 } = top_left;
    let Point { x: x2, y: y2 } = bottom_right;

    let width = x2 - x1;
    let height = y2 - y1;

    let area = width * height;
    if area == 0.0 {
        return Err("Not rectangle");
    }

    Ok(area)
}

fn square(p: Point, size: f32) -> Result<Rectangle, &'static str> {
    if size <= 0.0 {
        return Err("size should be positive");
    }

    let Point { x: px, y: py } = p;

    let square_rect = Rectangle {
        top_left: p,
        bottom_right: Point {
            x: px + size,
            y: py + size,
        },
    };

    Ok(square_rect)
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };
    println!("point coordinattes: ( {}, {} )", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ( {}, {} )", bottom_right.x, bottom_right.y);

    // destructuring...
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    println!("left_edge: {}, top_edge: {}", left_edge, top_edge);

    let rect = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // instantiate
    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains: {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains: {:?} and {:?}", integer, decimal);

    let area = rect_area(&rect);
    // println!("{:?}", area);
    let val = area.unwrap_or(0.0);
    println!("the area is {}", val);

    match area {
        Ok(a) => println!("the area is {}", a),
        Err(msg) => println!("{}", msg),
    }

    let rect2 = Rectangle {
        top_left: Point { x: 1.0, y: 1.0 },
        bottom_right: Point { x: 7.0, y: 5.0 },
    };
    let area = rect_area(&rect2);
    // println!("{:?}", area);
    let val = area.unwrap_or(0.0);
    println!("the area is {}", val);

    let rect3 = square(point, 3.0);
    if let Ok(rect3) = rect3 {
        println!("{}", rect_area(&rect3).unwrap());
    }
}
*/

// --------------------------------------------------------------------
// section 02: enums ...
/*
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(web_evt: WebEvent) {
    match web_evt {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
    }
}

// type aliases
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn print(&self) {
        match self {
            Self::Add => println!("Add"),
            Self::Subtract => println!("Subtract"),
        }
    }

    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let alias = Operations::Add;

    alias.print();
    println!("alias.run(3, 5) = {}", alias.run(3, 5));

    let alias = Operations::Subtract;
    alias.print();
    println!("alias.run(3, 5) = {}", alias.run(3, 5));
}
*/

// ----------------------------------------------------------
// section 03: enums - use..
/*
#![allow(dead_code)]

enum Status {
   Rich,
   Poor,
}

enum Work {
   Civilian,
   Soldier,
}

enum Number {
   Zero,
   One,
   Two,
}

enum Color {
   Red = 0xff0000,
   Green = 0x00ff00,
   Blue = 0x0000ff,
}

fn main() {
   use crate::Status::{Poor, Rich};
   use crate::Work::*;

   let status = Poor;
   let work = Civilian;

   match status {
       Poor => println!("the poor has no money"),
       Rich => println!("the rich has lots of money"),
   }

   match work {
       Civilian => println!("civilians work"),
       Soldier => println!("soldiers fight"),
   }

   println!("zero is {}", Number::Zero as i32);
   println!("one is {}", Number::One as i32);
   println!("roses are #{:06x}", Color::Red as i32);
   println!("violets are #{:06x}", Color::Blue as i32);
}
*/

// --------------------------------------------------------------------
// section 04: linked list

/* */
use crate::List::*;

enum List {
    // Tuple struct that wraps an element and pointer to the next node
    Cons(u32, Box<List>),
    // Nil
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // `self` has to be matched, because the behavior of this method
    // depends on the variant of `self`
    // `self` has type `&List`, and `*self` has type `List`, matching on a
    // concrete type `T` is preferred over a match on a reference `&T`
    // after Rust 2018 you can use self here and tail (with no ref) below as well,
    // rust will infer &s and ref tail.
    // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
