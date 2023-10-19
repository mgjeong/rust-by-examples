// ----------------------------------------------------------------------
// section 01. overview

/*
fn main() {
    fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
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
*/

// ----------------------------------------------------------------------
// section 02. associated funtions and methods
/*
struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated functions, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.top_left;
        let Point { x: x2, y: y2 } = self.bottom_right;

        ((x2 - x1) * (y2 - y1)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.top_left;
        let Point { x: x2, y: y2 } = self.bottom_right;

        2.0 * ((x2 - x1).abs() + (y2 - y1).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.top_left.x += x;
        self.top_left.y += y;

        self.bottom_right.x += x;
        self.bottom_right.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("destroying pair({}, {})", first, second);
    }
}

fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        top_left: Point::origin(),
        bottom_right: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)` √
    println!("rectangle perimeter: {}", rectangle.perimeter());
    println!("rectangle area: {}", rectangle.area());

    // Error! `rectangle` is immutable, but this method requires a mutable object
    // rectangle.translate(1.0, 0.0);

    let mut square = Rectangle {
        top_left: Point::origin(),
        bottom_right: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);
    println!(
        "top left point of square:     ({}, {})",
        square.top_left.x, square.top_left.y
    );
    println!(
        "bottom right point of square: ({}, {})",
        square.bottom_right.x, square.bottom_right.y
    );

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // pair.destroy(); // Error! Previous `destroy` call "consumed" `pair`
}
*/

// ----------------------------------------------------------------------
// section 03. closures - overview

/*
fn main() {
    let outer_var = 42;

    // A regular function can't refer to variables in the enclosing environment
    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncomment the line above and see the compiler error. The compiler
    // suggests that we define a closure instead.

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}
*/

// ----------------------------------------------------------------------
// section 04. closure - capturing

/*
// - by reference: &T
// - by mutable reference: &mut T
// - by value: T

fn main() {
    use std::mem;

    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.

    let print = || println!("`color`: {}", color);

    // call the closure using the borrow
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;
    print();

    let ref _ref_color = color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    // let _color_moved = color;
    print();

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    //let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;
    // inc(); // ERROR..
    println!("`count`: {}", count);

    // A non-copy typ.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume(); // ERROR

    // Using `move` before vertical pipes forces closure to take ownership of captured variables:
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("1 -> {}", contains(&1));
    println!("4 -> {}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.

    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}
*/

// ----------------------------------------------------------------------
// section 05. closure - as input parameters

/*
 */
// Fn: the closure uses the captured value by reference (&T)
// FnMut: the closure uses the captured value by mutable reference (&mut T)
// FnOnce: the closure uses the captured value by value (T)

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    // ^ TODO: try changing this to `Fn` or `FnMut`
    f()
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`
        println!("I said {}", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // println!("I said {}", greeting); // Ok
    // println!("Then I screamed {}", farewell); // ERROR

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
