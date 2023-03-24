/* section 01 - Structures */

// unit structure
// struct Nil;

// tuple structure
struct Pair(i32, f32);

// structure has elements
#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

// #[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// get distance between x and y
fn get_distance(p: Point) -> f32 {
    let sign = p.x * p.y;
    if sign > 0. {
        (p.x - p.y).abs()
    } else {
        p.x.abs() + p.y.abs()
    }
}

fn rect_area(rect: Rectangle) -> f64 {
    let width: f32 = get_distance(Point {
        x: rect.p1.x,
        y: rect.p2.x,
    });

    let height: f32 = get_distance(Point {
        x: rect.p1.y,
        y: rect.p2.y,
    });

    width as f64 * height as f64
}

fn square(point: Point, offset: f32) -> Rectangle {
    Rectangle {
        p1: point.clone(),
        p2: Point {
            x: point.x + offset,
            y: point.y + offset,
        },
    }
}

fn section01() {
    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("Point coordinates: ({}, {})", point.x, point.y);

    let Point { x: my_x, y: my_y } = point;
    println!("Point coordinates: ({}, {})", my_x, my_y);

    let _rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };
    println!(
        "( {}, {} )\n( {}, {} )",
        _rectangle.p1.x, _rectangle.p1.y, _rectangle.p2.x, _rectangle.p2.y
    );

    //   let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("integer: {:?}, decimal: {:?}", integer, decimal);

    // activity
    let rect1 = Rectangle {
        p1: Point { x: 1.0, y: 1.0 },
        p2: Point { x: 3.0, y: 4.0 },
    };
    println!("Area of rect1: {}", rect_area(rect1));

    let rect2 = Rectangle {
        p1: Point { x: -1.0, y: -1.0 },
        p2: Point { x: -3.0, y: -4.0 },
    };
    println!("Area of rect2: {}", rect_area(rect2));

    let rect3 = Rectangle {
        p1: Point { x: 1.0, y: -1.0 },
        p2: Point { x: -3.0, y: 4.0 },
    };
    println!("Area of rect3: {}", rect_area(rect3));

    let rect4 = Rectangle {
        p1: Point { x: -1.0, y: 1.0 },
        p2: Point { x: 3.0, y: -4.0 },
    };
    println!("Area of rect4: {}", rect_area(rect4));

    let rect5 = square(Point { x: 1.0, y: 1.0 }, 2.0);
    println!(
        "( {}, {} )\n( {}, {} )",
        rect5.p1.x, rect5.p1.y, rect5.p2.x, rect5.p2.y
    );

    let rect5 = square(Point { x: -1.0, y: -1.0 }, 2.0);
    println!(
        "( {}, {} )\n( {}, {} )",
        rect5.p1.x, rect5.p1.y, rect5.p2.x, rect5.p2.y
    );
}

/* section02 - Enums */

enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 },
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("is an engineer"),
        Person::Scientist => println!("is an scientist"),
        Person::Height(h) => println!("has a height of {}", h),
        Person::Weight(w) => println!("has a weight of {}", w),
        Person::Info { name, height } => {
            println!("{} is {} tall", name, height);
        }
    }
}

fn section02() {
    let person: Person = Person::Weight(18);
    let amira: Person = Person::Height(10);
    let dave = Person::Info {
        name: String::from("Dave"),
        height: 28,
    };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}

/* section 03 - use */
#[allow(dead_code)]
enum Status {
    Rich,
    Poor,
}

#[allow(dead_code)]
enum Work {
    Civilian,
    Soldier,
}

fn section03() {
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("Rich"),
        Poor => println!("Poor"),
    }

    match work {
        Civilian => println!("Civilian"),
        Soldier => println!("Soldier"),
    }
}

/* section 04 - C-like */

#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}

#[allow(dead_code)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn section04() {
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("virolets are #{:06x}", Color::Blue as i32);
}

/* section 05 - linked list */

use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // ToDo: add `append` function

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringfy(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringfy())
            }
            Nil => {
                format!("NIL!")
            }
        }
    }
}

fn section05() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Length of Linked list: {}", list.len());
    println!("{}", list.stringfy());
}

/* section 06 - constants */

static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn section06() {
    let n = 16;

    println!("this is {}", LANGUAGE);
    println!("the threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}

fn main() {
    section01();
    section02();
    section03();
    section04();
    section05();
    section06();
}
