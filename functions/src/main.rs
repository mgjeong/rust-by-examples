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
*/

// ----------------------------------------------------------------------
// section 06. closure - type anonymity

/*

// Closures succinctly capture variables from enclosing scopes. Does this have any consequences?
// It surely does. Observe how using a closure as a function parameter requires generics,
// which is necessary because of how they are defined:
// // `F` must be generic.
// fn apply<F>(f: F) where
//     F: FnOnce() {
//    f();
// }

// When a closure is defined, the compiler implicitly creates a new anonymous structure
// to store the captured variables inside, meanwhile implementing the functionality
// via one of the traits: Fn, FnMut, or FnOnce for this unknown type.
// This type is assigned to the variable which is stored until calling.

// Since this new type is of unknown type, any usage in a function will require generics.
// However, an unbounded type parameter <T> would still be ambiguous and not be allowed.
// Thus, bounding by one of the traits: Fn, FnMut, or FnOnce (which it implements) is sufficient
// to specify its type.

// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    // let x = 7;
    let x = "type anonymity";

    let print = || println!("{}", x);

    apply(print);
}
*/

// ----------------------------------------------------------------------
// section 07. closure - input function

/*

// Since closures may be used as arguments, you might wonder if the same can be said about functions.
// And indeed they can! If you declare a function that takes a closure as parameter,
// then any function that satisfies the trait bound of that closure can be passed as a parameter.

// Define a function which takes a generic `F` argumetn bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a clusure!");

    call_me(function);
    call_me(closure);
}
*/

// ----------------------------------------------------------------------
// section 08. closure - as output parameters

/*

// Closures as input parameters are possible, so returning closures as output parameters should also be possible.
// However, anonymous closure types are, by definition, unknown, so we have to use `impl Trait` to return them.

// The valid traits for returning a closure are:
// - Fn
// - FnMut
// - FnOnce

// Beyond this, the move keyword must be used, which signals that all captures occur by value.
// This is required because any captures by reference would be dropped as soon as the function exited,
// leaving invalid references in the closure.

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut2() -> impl FnMut() {
    let text = "FnMut2".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
    // || println!("This is a: {}", text) // ERROR.
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

    fn_plain();
    fn_mut();
    // fn_once(); // error

    // fn_mut = create_fnmut2(); // ERRROR.. WHY???????
}
*/

// ----------------------------------------------------------------------
// section 09. closure - examples in std

/* part 1 - Iterator::any

// pub trait Iterator {
//     type Item;

//     // `any` takes `&mut self` meaning the caller may be borrowed and modified, but not consumed.
//     fn any<F>(&mut self, f: F) -> bool
//     where
//         F: FnMut(Self::Item) -> bool;
// }

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2)); // `vec2` moved due to the `into_iter`

    println!("{:?}", vec1);
    // println!("{:?}", vec2); // ERROR

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    println!("{:?}", array1);
    println!("{:?}", array2); // NOT error... WHY ????

    // vec uses heap, array uses stack....
    // TODO: check ownership condition.
}
*/

/*
// part 2 - Searching through iterators

// pub trait Iterator {
//     // The type being iterated over.
//     type Item;

//     // `find` takes `&mut self` meaning the caller may be borrowed
//     // and modified, but not consumed.
//     fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
//         // `FnMut` meaning any captured variable may at most be
//         // modified, not consumed. `&Self::Item` states it takes
//         // arguments to the closure by reference.
//         P: FnMut(&Self::Item) -> bool;
// }

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its items,
    // so we have to destructure `&&i32` to `i32`
    println!("find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to referece on of
    // its items, so we have to destructure `&i32` to `i32`
    println!("find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    println!("{:?}", vec1);
    // println!("{:?}", vec2); // ERROR..

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "find 2 in array1: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );

    println!("{:?}", array1);
    println!("{:?}", array2);

    let vec = vec![1, 9, 3, 3, 13, 2];

    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    println!(
        "index of first even number: {:?}",
        index_of_first_even_number
    );
    println!(
        "index of first even number: {}",
        index_of_first_even_number.unwrap()
    );

    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    println!(
        "index of first negative number: {:?}",
        index_of_first_negative_number
    );
    if let Some(pos) = index_of_first_negative_number {
        println!("index of first negative number: {}", pos);
    } else {
        println!("No negative number in the array");
    }
}
*/

// ----------------------------------------------------------------------
// section 10. higher order functions

/*

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // imperative approach....
    let mut acc = 0;
    for n in 1.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("imperative style: {}", acc);

    // functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared <= upper)
        .filter(|&n| n % 2 == 1)
        .sum();

    println!("functional styleL: {}", sum_of_squared_odd_numbers);
}
*/

// ----------------------------------------------------------------------
// section 11. Diverging functions

/*
 */

// Diverging functions never return. They are marked using `!``, which is an empty type.

fn foo() -> ! {
    panic!("this call never returns.");
}

fn some_fn() {
    ()
}

fn main() {
    // foo(); // any code following this expression in unreachable
    let _a: () = some_fn();
    println!("This function returns and you can see this line");

    //f_never_type();
    fn sum_odd_numbers(upto: u32) -> u32 {
        let mut acc = 0;
        for i in 0..upto {
            let addition = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("sum of odd numbers up to 9: {}", sum_odd_numbers(9));
}

// #[feature(never_type)]

// fn f_never_type() {
//     let x: ! = panic!("This call never returns");
//     println!("you will never see this line!");
// }
