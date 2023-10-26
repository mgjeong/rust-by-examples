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
 */
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
// to `borrow2` is irrelevant since they are disjoint.o
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

// ------------------------------------------------------------------------
// section 10. lifetimes - explicit annotation

// ------------------------------------------------------------------------
// section 11. lifetimes - functions

// ------------------------------------------------------------------------
// section 12. lifetimes - methods

// ------------------------------------------------------------------------
// section 13. lifetimes - structs

// ------------------------------------------------------------------------
// section 14. lifetimes - traits

// ------------------------------------------------------------------------
// section 15. lifetimes - bounds

// ------------------------------------------------------------------------
// section 16. lifetimes - coercion

// ------------------------------------------------------------------------
// section 17. lifetimes - static

// ------------------------------------------------------------------------
// section 18. lifetimes - elision
