// ------------------------------------------------------------------
// section 01. overview

// Generics is the topic of generalizing types and functionalities to broader cases.
// This is extremely useful for reducing code duplication in many ways, but can call for rather involved syntax.
// Namely, being generic requires taking great care to specify over which types a generic type
// is actually considered valid. The simplest and most common use of generics is for type parameters.
//
//A type parameter is specified as generic by the use of angle brackets and upper camel case: `<Aaa, Bbb, ...>`.
// "Generic type parameters" are typically represented as `<T>`. In Rust, "generic" also describes anything that
// accepts one or more generic type parameters `<T>``. Any type specified as a generic type parameter is generic,
// and everything else is concrete (non-generic).
//
// For example, defining a generic function named foo that takes an argument T of any type:
//
// `fn foo<T>(arg: T) { ... }`
//
// Because `T` has been specified as a generic type parameter using `<T>`, it is considered generic
// when used here as (`arg: T`). This is the case even if `T` has previously been defined as a struct.

/*

// A concrete type 'A'
struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
struct Single(A);
//            ^ Here is `Single`s first use of the type `A`.

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top.
struct SingleGen<T>(T);

fn main() {
    // `Single` is concrete and explicityly takes `A`
    let _s = Single(A); // type aliasing??

    let a: A = A;
    let _s2 = Single(a);

    // create a variable `_char` of type `SingleGen<char>`
    // and give it the value `SingleGen('a')`
    // here, `SingleGen` has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified.
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char_implicit = SingleGen('a');
}
*/

// ------------------------------------------------------------------
// section 02. functions

/*
struct A; // concrete type
struct S(A); // concrete type
struct SGen<T>(T); // generic type

// The following functions all take ownership of the variable passed into
// them and immediately go out of scope, freeing the variable.

// Define a function `reg_fn` that takes an argument `_s` of type `S`.
// This has no `<T>` so this is not a generic function.
fn reg_fn(_s: S) {}

// Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
// It has been explicitly given the type parameter `A`, but because `A` has not
// been specified as a generic type parameter for `gen_spec_t`, it is not generic.
fn get_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
fn get_spec_i32(_s: SGen<i32>) {}

// Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A)); // concrete type
    get_spec_t(SGen(A)); // implicitly specified type parameter
    get_spec_i32(SGen(6)); // implicitly specified type parameter

    generic::<char>(SGen('a')); // explicitly specified type parameter `char` to `generic()`
    generic(SGen('c')); // implicitly specified type parameter `char` to 'generic()`
}
*/

// ------------------------------------------------------------------
// section 03. implementation

/*
// struct S;
// struct GenericVal<T>(T);

// impl GenericVal<f32> {}
// impl GenericVal<S> {}

// // `<T>` must precede the type to remain generic
// impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    let z: GenVal<usize> = GenVal { gen_val: 10 };
    let u = GenVal { gen_val: 10 };

    println!("{}, {}", x.value(), y.value());
    println!("{}, {}", z.value(), u.value());
}
*/

// ------------------------------------------------------------------
// section 04. traits

// Of course traits can also be generic.
// Here we define one which reimplements the Drop trait as a generic method to drop itself and an input.

/*
// non copyable types
struct Empty;
struct Null;

// a trait generic over `T`
trait DoubleDrop<T> {
    // define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it
    fn double_drop(self, _: T);
}

// implementation `DoubleDrop<T>` for any generic parameter `T` and caller `U`
// trait 의 경우 항상 두 개의 generic type을 사용해야 하는가???
impl<T, U> DoubleDrop<T> for U {
    // this method takes ownership of both passed arguements,
    // deallocating both.
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // deallocation `empty` and `null`
    empty.double_drop(null);
}
*/

// ------------------------------------------------------------------
// section 05. bounds

/*

// When working with generics, the type parameters often must use traits as bounds to stipulate
// what functionality a type implements. For example, the following example uses the trait Display
//  to print and so it requires T to be bound by Display; that is, T must implement Display.

// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
// ```
// fn printer<T: Display>(t: T) {
//     println!("{}", t);
// }
// ```
// Bounding restricts the generic to types that conform to the bounds. That is:
// ```
// struct S<T: Display>(T);
//
// // Error! `Vec<T>` does not implement `Display`. This
// // specialization will fail.
// let s = S(vec![1]);
// ```
// Another effect of bounding is that generic instances are allowed to access the methods of traits
// specified in the bounds. For example:

use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rect = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _tri = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rect);
    println!("Area: {}", area(&rect));

    //print_debug(&_tri);
    //println!("Area: {}", area(&_tri));
}
*/

// ------------------------------------------------------------------
// section 06. testcase: empty bounds

/*

// A consequence of how bounds work is that even if a `trait` doesn't include any functionality,
// you can still use it as a bound. `Eq` and `Copy` are examples of such `trait`s from the std library.

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// these functions are only valid for types which implement these traits.
// the fact that the traits are empty is irrelevant.

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("a cardinal is {}", red(&cardinal));
    println!("a blue jay is {}", blue(&blue_jay));
}
*/

// ------------------------------------------------------------------
// section 07. multiple bounds

/*

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug:  `{:?}`", t);
    println!("Display:  `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec)
}
*/

// ------------------------------------------------------------------
// section 08. where clauses

/*

// A bound can also be expressed using a where clause immediately before the opening {,
// rather than at the type's first mention. Additionally, where clauses can apply bounds to arbitrary types,
// rather than just to type parameters.

// Some cases that a where clause is useful:

// - When specifying generic types and bounds separately is clearer:

// ```
// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// // Expressing bounds with a `where` clause
// impl <A, D> MyTrait<A, D> for YourType where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF {}
// ```

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
*/

// ------------------------------------------------------------------
// section 09. new type idiom

/*

struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);

    let age_days = age.to_days();

    println!("old enough {}", old_enough(&age));
    println!("old enough {}", old_enough(&age_days.to_years()));
    // println!("old enough {}", old_enough(&age_days)); // ERROR...

    let years = Years(42);
    let years_as_primitive_1 = years.0;
    let Years(years_as_primitive_2) = years;
    println!("{}, {}", years_as_primitive_1, years_as_primitive_2);
}
*/

// ------------------------------------------------------------------
// section 10. associate items: the problem

/*

// A `trait` that is generic over its container type has type specification requirements - users of the `trait`
// must specify all of its generic types.
//
// In the example below, the `Contains` `trait` allows the use of the generic types `A` and `B`.
// The trait is then implemented for the `Container` type, specifying `i32` for `A` and `B`
// so that it can be used with fn difference().
//
// Because `Contains` is generic, we are forced to explicitly state all of the generic types for fn `difference()`.
// In practice, we want a way to express that `A` and `B` are determined by the input `C`.
// As you will see in the next section, associated types provide exactly that capability.

struct Container(i32, i32);

// a trait which checks if 2 itmes are stored inside of container.
// also retrieves first or last value.
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // explicityly requires `A` and `B`
    fn first(&self) -> i32; // doesn't explicitly require `A` or `B`
    fn last(&self) -> i32; // doesn't explicitly require `A` or `B`
}

impl Contains<i32, i32> for Container {
    // true if the numbers stored are equal.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and `B` again is nuisance.
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn main() {
    let n1 = 3;
    let n2 = 10;

    let container = Container(n1, n2);

    println!(
        "does container contain {} and {}: {}",
        &n1,
        &n2,
        container.contains(&n1, &n2)
    );

    println!("first number: {}", container.first());
    println!("last number: {}", container.last());

    println!("the difference: {}", difference(&container));
}
*/

// ------------------------------------------------------------------
// section 11. associate items: associated types

/*

// The use of "Associated types" improves the overall readability of code by moving inner types locally
// into a trait as output types. Syntax for the `trait`` definition is as follows:

// ```
// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for aliases).
// trait Contains {
//     type A;
//     type B;

//     // Updated syntax to refer to these new types generically.
//     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
// }
// ```

// Note that functions that use the trait Contains are no longer required to express A or B at all:

// ```
// Without using associated types
// fn difference<A, B, C>(container: &C) -> i32 where
//    C: Contains<A, B> { ... }

// Using associated types
// fn difference<C: Contains>(container: &C) -> i32 { ... }
// ```

//Let's rewrite the example from the previous section using associated types:

struct Container(i32, i32);

// a trait which check if 2 items are stored inside of container.
// also retrieves first or last value.
trait Contains {
    // define generic types here which methods will be albe to utilize
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here
    // fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let n1 = 3;
    let n2 = 10;

    let container = Container(n1, n2);

    println!(
        "does container contain {} and {}: {}",
        &n1,
        &n2,
        container.contains(&n1, &n2)
    );

    println!("first number: {}", container.first());
    println!("last number: {}", container.last());

    println!("the difference: {}", difference(&container));
}
*/

// ------------------------------------------------------------------
// section 12. phantom type parameters

/*
// A phantom type parameter is one that doesn't show up at runtime,
// but is checked statically (and only) at compile time.
// Data types can use extra generic type parameters to act as markers
// or to perform type checking at compile time. These extra parameters
// hold no storage values, and have no runtime behavior.
//
// In the following example, we combine `std::marker::PhantomData` with
// the phantom type parameter concept to create tuples containing different data types.

use std::marker::PhantomData;

// a phantom tuple struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // allow equality test for this type
struct PhantomTuple<A, B>(A, PhantomData<B>);

// a phantom type sturct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // allow equality test fo this type
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// note: storage is allocated for generic type `A`, but not for `B`
// therefore, `B` cannot be used in computations.
fn main() {
    // here, `f32` and `f64` are the hidden parameters.
    // PhantomTuple type specified as `<char, f32>`
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);

    // PhantomTuple type specified as `<char, f64>`
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // type specified as `<char, f32>`
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // type specified as `<char, f64>`
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2); // ERROR...
    // println!("_sturct1 == _struct2 yields: {}", _struct1 == _struct2); // ERROR...
}
*/

// ------------------------------------------------------------------
// section 13. phantom type parameters - testcase: unit clarification

// A useful method of unit conversions can be examined by implementing Add with a phantom type parameter.
// The Add trait is examined below:
// ```
// // This construction would impose: `Self + RHS = Output`
// // where RHS defaults to Self if not specified in the implementation.
// pub trait Add<RHS = Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

// // `Output` must be `T<U>` so that `T<U> + T<U> = T<U>`.
// impl<U> Add for T<U> {
//     type Output = T<U>;
//     ...
// }
// ```

use std::marker::PhantomData;
use std::ops::Add;

// create void enumerations to define unit types
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

// `Length` is a type with phantom type parameter `Unit` and is not generic over the length type( this is `f64` ).
// `f64` already implements the `Clone` and `Copy` traits.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// the `Add` trait defines that behavior of the `+` operator.
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // fn add(self, rhs: Output) -> Output { // ERROR... wwhy ??????
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` calls the `Add` implementation for `f64`
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // `+` calls the `add()` method we implemented for `Length<Unit>`
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one foot = {:?}", two_feet.0);
    println!("one meter + one meter = {:?}", two_meters.0);

    // let one_feter = one_foot + one_meter; // ERROR.. type mismatch..
}
