/* section 01 - basic */

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

fn section01() {
    fizzbuzz_to(100);
}

/* section 02 - associated functions and methods */

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(self: &Self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(self: &mut Self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self: Self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

fn section02() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}:", rectangle.area());

    // rectangle.translate(1.0, 0.0); // error

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    let Point { x: x1, y: y1 } = square.p1;
    let Point { x: x2, y: y2 } = square.p2;
    println!("square Point1:({}, {}), Point2:({}, {})", x1, y1, x2, y2);

    square.translate(1.0, 1.0);
    let Point { x: x1, y: y1 } = square.p1;
    let Point { x: x2, y: y2 } = square.p2;
    println!("square Point1:({}, {}), Point2:({}, {})", x1, y1, x2, y2);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
    // pair.destroy(); // error.. ownership already taken
}

/* section 03 - closures */

fn section03() {
    let outer_var = 43;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| -> i32 { i + outer_var };

    println!("closure annotated: {}", closure_annotated(1));
    println!("closure inferred: {}", closure_inferred(1));

    // println!("cannot reuse.. {}", closure_inferred(23i64)); // error

    let one = || 1;
    println!("closure returning one: {}:", one());
}

/* section 04 - closures, capturing */

fn section04() {
    use std::mem;

    let color = String::from("green");

    let print = || println!("color: {}", color); // borrow immutable reference ^T
    print(); // borrow...
    print(); // possible

    let _reborrow = &color; // an immutable reference of `color`
    print(); // possible
    print(); // possible

    let _color_moved = color; // move owership.

    // print(); // error

    let mut count = 0;

    let mut inc = || {
        // borrow mutable reference &mut T
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    // let _reborrow = &count; // immutable borrow

    // inc(); // error

    let _count_reborrowed = &mut count; // borrow mutable reference
                                        // inc(); // error

    let movable = Box::new(3);
    let consume = || {
        // move ownership because mem:drop requires T
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // consume(); // error.

    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("there're {} elements in vec", haystack.len()); // error... due to the `move`

    // quick test for the `move`
    let mut var: i32 = 5;
    let mut tentimes = /* move */ || {
        var *= 10;
        println!("time times: {}", var);
    };

    tentimes();
    tentimes();

    drop(tentimes);

    println!("var: {}", var);
    var *= 10;
    println!("var * 10: {}", var);
    // tentimes();
    println!("var * 10: {}", var);
}

/* section05 - functions, as input parameters */

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn section05() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("then I screamed {}", farewell);
        println!("Now I can sleep. zzz");

        mem::drop(farewell);
    };

    // println!("farewell: {}", farewell.clone());

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

fn main() {
    section01();
    section02();
    section03();
    section04();
    section05();
}
