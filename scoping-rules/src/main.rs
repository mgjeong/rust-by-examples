// ------------------------------------------------------------------------
// section 01. RAII( Resource Acquisition Is Initialization )

/*
// Variables in Rust do more than just hold data in the stack: they also own resources,
// e.g. Box<T> owns memory in the heap. Rust enforces RAII (Resource Acquisition Is Initialization),
// so whenever an object goes out of scope, its destructor is called and its owned resources are freed.
//
// This behavior shields against resource leak bugs, so you'll never have to manually free memory
// or worry about memory leaks again! Here's a quick showcase:

fn create_box() {
    // allocate an integer on the heap
    let _box1 = Box::new(3i32);
    // `_box1` is destroyed here, and memory gets freed
}

fn main() {
    // allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // a nested scope
    {
        // allocate
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here.
    }

    // create lots of boxes.
    // there is no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here
}
*/

// ------------------------------------------------------------------------
// section 02. Destructor

/*
// The notion of a destructor in Rust is provided through the `Drop` trait.
// The destructor is called when the resource goes out of scope. This trait is not required
// to be implemented for every type, only implement it for your type if you require its own destructor logic.
//
// Run the below example to see how the `Drop` trait works. When the variable in the main function
// goes out of scope the custom destructor will be invoked.

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let _x = ToDrop;
    println!("Made a ToDrop!");
}
*/

// ------------------------------------------------------------------------
// section 03. Ownership and moves

/*
// Because variables are in charge of freeing their own resources, resources can only have one owner.
// This also prevents resources from being freed more than once. Note that not all variables own resources
//
// When doing assignments (`let x = y`) or passing function arguments by value (`foo(x)`),
// the ownership of the resources is transferred. In Rust-speak, this is known as a move.
//
// After moving resources, the previous owner can no longer be used. This avoids creating dangling pointers.
//

// this function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("destroying a box that contains {}", c);
    // `c` is destroyed and the memory  freed.
}

fn main() {
    let x = 5u32; // _Stack_ allocated.
    let y = x; // *Copy* `x` into `y` - no resources are moved;

    println!("x is {}, and y is {}", x, y); // both values can be independently used.

    let a = Box::new(5i32); // `a` is a pointer to a _Heap_ allocated integer

    println!("a contains: {}", a);

    let b = a; // *Move* `a` into `b`
               // the pointer adress of `a` is copied ( not the data ) into `b`
               // both are now pointers to the same heap allocated data, but `b` now owns it.

    // println!("a contains: {}", a); // ERROR...
    // `a` can no longer access the data, because it no longer owns the heap memory

    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    // println!("b contains: {}", b);
}
*/

// ------------------------------------------------------------------------
// section 04. Mutability

/*

fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // *immutable_box = 4; // mutability ERROR...

    // *Move* the box, changing the ownership and mutability
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4u32;

    println!("mutable_box contains {}", mutable_box);
}
*/

// ------------------------------------------------------------------------
// section 05. partial moves

/*
// Within the destructuring of a single variable, both by-move and by-reference pattern bindings
// can be used at the same time. Doing this will result in a partial move of the variable,
// which means that parts of the variable will be moved while other parts stay.
// In such a case, the parent variable cannot be used afterwards as a whole,
// however the parts that are only referenced (and not moved) can still be used.

fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: "Alice".to_owned(),
        age: Box::new(20),
    };

    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // println!("The person is {:?}", person); // ERROR... moved `person.name` ---> `name`

    println!("The person's age from person is {}", person.age);
}
*/

// ------------------------------------------------------------------------
// section 06. borrowing

/*
// Most of the time, we'd like to access data without taking ownership over it.
// To accomplish this, Rust uses a borrowing mechanism. Instead of passing objects by value (T),
// objects can be passed by reference (&T).
//
// The compiler statically guarantees (via its borrow checker) that references always point to valid objects.
// That is, while references to an object exist, the object cannot be destroyed.

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("this int is: {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    // println!("boxed: {}", boxed_i32);
    // println!("stacked: {}", stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        // eat_box_i32(boxed_i32); // ERROR due to the following code....
        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}
*/

// ------------------------------------------------------------------------
// section 07. borrowing: mutability

/*
// Mutable data can be mutably borrowed using `&mut T`. This is called a mutable reference
// and gives read/write access to the borrower. In contrast, `&T` borrows the data via an immutable reference,
// and the borrower can read the data but not modify it:

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// this function takes a referernce to a book
fn borrow_book(book: &Book) {
    println!("immutably borrowed {} - {} edition", book.title, book.year);
}

// this function takes a reference to a mutable book and change `year`
fn new_edition(book: &mut Book) {
    book.year = 2023;
    println!("mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // create a mutable copy of `immutabook`.
    let mut mutabook = immutabook;

    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);

    //new_edition(&mut immutabook); // ERROR....
}
*/

// ------------------------------------------------------------------------
// section 07. borrowing: aliasing

/*
// Data can be immutably borrowed any number of times, but while immutably borrowed,
// the original data can't be mutably borrowed. On the other hand, only one mutable borrow is allowed at a time.
// The original data can be borrowed again only after the mutable reference has been used for the last time.

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // data can be accessed via the reference and the original owner
    println!(
        "point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // Error! Can't borrow `point` as mutable because it's currently borrowed as immutable.
    // let mutable_borrow = &mut point;

    // BECAUSE, The borrowed values are used again here
    println!(
        "point has coordinatte: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // The immutable references are no longer used for the rest of the code so
    // it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! Can't borrow `point` as immutable because it's currently borrowed as mutable.
    // let y = &point.y;
    // Error! Can't print because `println!` takes an immutable reference.
    // println!("Point Z coordinate is {}", point.z);

    // Ok! Mutable references can be passed as immutable to `println!`
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // The mutable reference is no longer used for the rest of the code so it
    // is possible to reborrow
    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}
*/

// ------------------------------------------------------------------------
// section 08. borrowing: the ref pattern

/*
// When doing pattern matching or destructuring via the `let` binding,
// the `ref` keyword can be used to take references to the fields of a struct/tuple.
// The example below shows a few instances where this can be useful:

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
    // println!("ref_c1 : {:?}", &ref_c1);
    // println!("ref_c2 : {:?}", &ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` is also valid when destructuring a struct
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        // return a copy of the `x` field of `point`
        *ref_to_x
    };

    // a mutable copy of `point`
    let mut mutable_point = point;
    {
        // `ref` can be paired with `mut` to take mutable references.
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // a mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        // destructuring `mutable_tuple` to changer the value of `last`
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);

    {
        let last = &mut mutable_tuple.1;
        *last = 1u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}
*/

// ------------------------------------------------------------------------
// section 09. lifetimes - overview
/*

// A lifetime is a construct of the compiler (or more specifically, its borrow checker)
// uses to ensure all borrows are valid. Specifically, a variable's lifetime begins
// when it is created and ends when it is destroyed.
// While lifetimes and scopes are often referred to together, they are not the same.
//
// Take, for example, the case where we borrow a variable via `&`.
// The borrow has a lifetime that is determined by where it is declared.
// As a result, the borrow is valid as long as it ends before the lender is destroyed.
// However, the scope of the borrow is determined by where the reference is used.
//
// In the following example and in the rest of this section,
// we will see how lifetimes relate to scopes, as well as how the two differ.

// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses
// both `borrow1` and `borrow2`. The duration of `borrow1` compared
// to `borrow2` is irrelevant since they are disjoint.

fn main() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }

    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }
}
*/

// ------------------------------------------------------------------------
// section 10. lifetimes - explicit annotation
/*
// The borrow checker uses explicit lifetime annotations to determine how long references should be valid.
// In cases where lifetimes are not elided1, Rust requires explicit annotations to determine
// what the lifetime of a reference should be.
// The syntax for explicitly annotating a lifetime uses an apostrophe character as follows:
//
// `foo<'a>``
// // `foo` has a lifetime parameter `'a`
//
// Similar to closures, using lifetimes requires generics. Additionally, this lifetime syntax indicates that
// the lifetime of `foo` may not exceed that of `'a`.
// Explicit annotation of a type has the form `&'a T` where `'a` has already been introduced.
//
// In cases with multiple lifetimes, the syntax is similar:
//
// `foo<'a, 'b>`
// // `foo` has lifetime parameters `'a` and `'b`
//
// In this case, the lifetime of foo cannot exceed that of either `'a` or `'b`.
//
// See the following example for explicit lifetime annotation in use:

// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// a function which takes no arguments, but has a lifetime parameter `'a`
fn failed_borrow<'a>() {
    let _x = 12;

    // let y: &'a i32 = &_x; // ERROR... `_x` does not live long enough.
    // Attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than that of `y`. A short lifetime cannot be coerced into a longer one.
}

fn main() {
    // create variables to be borrowed below.
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}
*/

// ------------------------------------------------------------------------
// section 11. lifetimes - functions
/*

// Ignoring elision, function signatures with lifetimes have a few constraints:
// - any reference must have an annotated lifetime.
// - any reference being returned must have the same lifetime as an input or be static.
// Additionally, note that returning references without input is banned
//  if it would result in returning references to invalid data.
// The following example shows off some valid forms of functions with lifetimes:

// One input reference with lifetime `'a` which must live
// at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// mutable references are possible with lifetime as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'a i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }
// The above is invalid: `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
*/

// ------------------------------------------------------------------------
// section 12. lifetimes - methods
/*

// methods are annotated similarity to funcitons:
struct Owner(i32);

impl Owner {
    // annotate lifetime as in a standalone function
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
*/

// ------------------------------------------------------------------------
// section 13. lifetimes - structs
/*
// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
*/

// ------------------------------------------------------------------------
// section 14. lifetimes - traits

/*
// Annotation of lifetimes in trait methods basically are similar to functions. Note that impl may have annotation of lifetimes too.

#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("{}", b.x);
    println!("b is {:?}", b);
}
*/

// ------------------------------------------------------------------------
// section 15. lifetimes - bounds

/*
// Just like generic types can be bounded, lifetimes (themselves generic) use bounds as well.
// The `:` character has a slightly different meaning here, but `+` is the same. Note how the following read:
//
// 1. `T`: 'a`: All references in `T` must outlive lifetime `'a`.
// 2. `T: Trait + 'a`: Type `T` must implement trait `Trait` and all references in T must outlive `'a`.
//
// The example below shows the above syntax in action used after keyword `where:``

use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime of `Ref` may not exceed `'a`.

// a generic function which prints using the `Debug` triat.
fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements `Debug` and all *references* in `T` outlive `'a`.
// In addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`:t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
*/

// ------------------------------------------------------------------------
// section 16. lifetimes - coercion
/*
// A longer lifetime can be coerced into a shorter one so that it works inside a scope it normally wouldn't work in.
// This comes in the form of inferred coercion by the Rust compiler, and also in the form of declaring a lifetime difference:

// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    // fn multiply(first: &i32, second: &i32) -> i32 {
    first * second
}

//`<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    // fn choose_first<'a>(first: &'a i32, _: &'a i32) -> &'a i32 {
    first
}

fn main() {
    let first = 2;
    {
        let second = 3;
        println!("the product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}
*/

// ------------------------------------------------------------------------
// section 17. lifetimes - static

// Rust has a few reserved lifetime names. One of those is 'static.
// You might encounter it in to situation.
// ```
// // A reference with 'static lifetime:
// let s: &'static str = "hello world";
//
// // 'static as part of a trait bound:
// fn generic<T>(x: T) where T: 'static {}
// ```
// Both are related but subtly different and this is a common source for confusion when learning Rust.
// Here are some examples for each situation:

/*
// ** Reference lifetime **
// As a reference lifetime `'static` indicates that the data pointed to by the reference lives
// for the entire lifetime of the running program. It can still be coerced to a shorter lifetime.
// There are two ways to make a variable with `'static` lifetime,
// and both are stored in the read-only memory of the binary:
// - Make a constant with the `static` declaration.
// - Make a `string` literal which has type: `&'static str`.
// See the following example for a display of each method:

// make a constant with `'static`lifetime
static NUM: i32 = 18;

// return a reference to `NUM` where its `'static` lifetime is coered
// to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // make a `string` literal and print it
        let static_stringr = "I'm read-only memory";
        println!("staic_string: {}", static_string);
        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }
    {
        // make an integer to use for `coerce_static`
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays assessible!", NUM);
}
*/

/*
// ** Trait bound **

// As a trait bound, it means the type does not contain any non-static references.
// Eg. the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it.

// It's important to understand this means that any owned data always passes a `'static` lifetime bound,
// but a reference to that owned data generally does not:

use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of main(),
    // so it's not 'static
    // print_it(&i); // ERROR...

    // let string = "I'm s string";
    let string: &'static str = "I'm s string"; // same as above line.
    print_it(string);
}
*/

// ------------------------------------------------------------------------
// section 18. lifetimes - elision

// Some lifetime patterns are overwhelmingly common and so the borrow checker will allow you
// to omit them to save typing and to improve readability. This is known as elision.
// Elision exists in Rust solely because these patterns are common.
// The following code shows a few examples of elision.
// For a more comprehensive description of elision, see lifetime elision in the book.

// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is inferred by the compiler
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn main() {
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
