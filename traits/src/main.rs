// -----------------------------------------------------------------
// section 01. overview

/*
// A `trait` is a collection of methods defined for an unknown type: `Self`.
// They can access other methods declared in the same trait.
//
// Traits can be implemented for any data type. In the example below, we define `Animal`, a group of methods.
// The `Animal trait` is then implemented for the `Sheep` data type,
// allowing the use of methods from `Animal` with a `Sheep`.

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Associated function signature: `Self` refers to the implementation type
    fn new( name: &'static str ) -> Self;

    // method signature: there will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut", self.name());
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah!"
        } else {
            "haaaaaab!"
        }
    }

    // default trait mathods can be overridden
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());

    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    // SAME ?????
    let mut molly = Sheep { name: "molly", naked: false };
    molly.talk();
    molly.shear();
    molly.talk();
}
*/

// -----------------------------------------------------------------
// section 02. Derive
/*
// The compiler is capable of providing basic implementations for some traits via the `#[derive]` attribute.
// These traits can still be manually implemented if a more complex behavior is required.
//
// The following is a list of derivable traits:
//
// - Comparison traits: `Eq`, `PartialEq`, `Ord`, `PartialOrd`.
// - `Clone`, to create `T` from `&T` via a copy.
// - `Copy`, to give a type 'copy semantics' instead of 'move semantics'.
// - `Hash`, to compute a hash from `&T`.
// - `Default`, to create an empty instance of a data type.
// - `Debug`, to format a value using the `{:?}` formatter.

// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // println!("one second looks like: {:?}", _one_second); // ERROR...

    // let _this_is_true = (_one_second == _one_second); // ERROR... requires `PartialEq`

    let foot = Inches(12);
    println!("foot equeals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("foot is {} than one meter", cmp);
}
*/

// -----------------------------------------------------------------
// section 03. Returning traits with dyn
/*
// The Rust compiler needs to know how much space every function's return type requires.
// This means all your functions have to return a concrete type.
// Unlike other languages, if you have a trait like `Animal`, you can't write a function that returns `Animal`,
// because its different implementations will need different amounts of memory.
//
// However, there's an easy workaround. Instead of returning a trait object directly,
// our functions return a `Box` which contains some `Animal`.
// A `box` is just a reference to some memory in the heap. Because a reference has a statically-known size,
// and the compiler can guarantee it points to a heap-allocated `Animal`,
// we can return a trait from our function!
//
// Rust tries to be as explicit as possible whenever it allocates memory on the heap.
// So if your function returns a pointer-to-trait-on-heap in this way,
// you need to write the return type with the dyn keyword, e.g. `Box<dyn Animal>`.

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mooooooo!"
    }
}

// returns some struct that implements Animal,
// but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.32;
    let animal = random_animal(random_number);
    println!("random: {}", animal.noise());

    let random_number = 0.72;
    let animal = random_animal(random_number);
    println!("random: {}", animal.noise());
}
*/

// -----------------------------------------------------------------
// section 04. operator overloading
/*
// In Rust, many of the operators can be overloaded via traits.
// That is, some operators can be used to accomplish different tasks based on their input arguments.
// This is possible because operators are syntactic sugar for method calls.
// For example, the `+` operator in `a + b` calls the `add` method (as in `a.add(b)`).
// This add method is part of the `Add` trait.
// Hence, the `+` operator can be used by any implementor of the `Add` trait.
//
// A list of the traits, such as Add, that overload operators can be found in `core::ops`.

use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
*/

// -----------------------------------------------------------------
// section 05. drop
/*
// The Drop trait only has one method: `drop`, which is called automatically
// when an object goes out of scope. The main use of the Drop trait is to free
// the resources that the implementor instance owns.
//
// `Box`, `Vec`, `String`, `File`, and `Process` are some examples of types
// that implement the `Drop` trait to free resources.
// The `Drop` trait can also be manually implemented for any custom data type.
//
// The following example adds a print to console to the `drop` function to announce when it is called.

struct Droppable {
    name: &'static str,
}

// this is trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };
        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("exiting block B");
        }
        println!("just exited block B");

        println!("exiting block A");
    }
    println!("just exited block A");

    drop(_a);

    println!("end of the main");
}
*/

// -----------------------------------------------------------------
// section 06. iterators
/*
// The Iterator trait is used to implement iterators over collections such as arrays.
//
// The trait requires only a method to be defined for the `next` element,
// which may be manually defined in an impl block or automatically defined (as in arrays and ranges).
//
// As a point of convenience for common situations, the `for` construct turns some collections
// into iterators using the `.into_iter()` method.

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    // we can refer to this type use Self::Item
    type Item = u32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

// return a Fibonacchi sequence generator
fn fibonacchi() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate throught 0..3 using for");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("the first four terms of the sequence are: ");
    for i in fibonacchi().take(4) {
        println!("> {}", i);
    }

    println!("the nexts four terms of the sequence are: ");
    for i in fibonacchi().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
    println!("iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
*/

// -----------------------------------------------------------------
// section 07. impl trait

/*
// part 1. as an argument type
// if your function is generic over a trait but you don't mind the specific type,
// you can simplify the function declaration using `impl Trait` as the type of the argument.

// before,, using generic..
fn parse_csv_document1<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}

// after,, it't not important what type `R` is...
fn parse_csv_document2(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}
*/

/*
// part 2. as a return type
// If your function returns a type that implements `MyTrait``,
// you can write its return type as -> `impl MyTrait`.
// This can help simplify your type signatures quite a lot!

use std::iter;
use std::vec::IntoIter;

fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

fn double_positive<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);

    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    // for i in v3.take(10) {
    //     println!("{:?}", i);
    // }

    let v5 = vec![1, 2, 3];
    let v6 = vec![4, 5];
    let mut v4 = combine_vecs_explicit_return_type(v5, v6);
    assert_eq!(Some(1), v4.next());
    assert_eq!(Some(2), v4.next());
    assert_eq!(Some(3), v4.next());
    assert_eq!(Some(4), v4.next());
    assert_eq!(Some(5), v4.next());
    // for i in v4.take(10) {
    //     println!("{:?}", i);
    // }

    let plus_one = make_adder_function(1);
    println!("plus one(2) = {}", plus_one(2));

    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positive(&singles);
    for v in doubles {
        println!("{:?}", v);
    }
}
*/

// -----------------------------------------------------------------
// section 08. clone
/*
#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let unit = Unit;
    let copied_unit = unit; // copy.. no resource moving

    // both can be used independently
    println!("original: {:?}", unit);
    println!("copied: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // println!("original: {:?}", pair); // ERROR..  alreay moved..

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    // println!("moved: {:?}", moved_pair); // ERROR... dropped...

    println!("cloned: {:?}", cloned_pair);
}
*/

// -----------------------------------------------------------------
// section 09. supertraits
/*
// a Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait.
trait Person {
    fn name(&self) -> String;
}

// person is a supertrait of Student
// implementing Student requires you to also impl Person
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait ComSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct Engineer {
    name: String,
    univ: String,
    lang: String,
    uname: String,
}

impl ComSciStudent for Engineer {
    fn git_username(&self) -> String {
        self.uname.clone()
    }
}

impl Programmer for Engineer {
    fn fav_language(&self) -> String {
        self.lang.clone()
    }
}

impl Student for Engineer {
    fn university(&self) -> String {
        self.univ.clone()
    }
}

impl Person for Engineer {
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn comp_sci_student_greeting(student: &dyn ComSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {
    let engineer = Engineer {
        name: "Hoejoo".to_string(),
        univ: "Bugyeong".to_string(),
        lang: "Java".to_string(),
        uname: "hoejoo".to_string(),
    };

    println!("info: {}", comp_sci_student_greeting(&engineer));
}
*/

// -----------------------------------------------------------------
// section 10. disambiquating overlapping traits
/*
 */
// A type can implement many different traits. What if two traits both require the same name?
// For example, many traits might have a method named get(). They might even have different return types!
//
// Good news: because each trait implementation gets its own impl block,
// it's clear which trait's get method you're implementing.
//
// What about when it comes time to call those methods?
// To disambiguate between them, we have to use Fully Qualified Syntax.

trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 20,
    };

    // println!("{}", form.get()); // ERROR.. multiple get.... ambiguaty..
    let uname = <Form as UsernameWidget>::get(&form);
    let age = <Form as AgeWidget>::get(&form);
    println!("{}, {}", uname, age);
}
