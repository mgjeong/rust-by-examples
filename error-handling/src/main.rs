// --------------------------------------------------------------------
// section 01 - panic

/*
// The simplest error handling mechanism we will see is `panic`. It prints an error message,
// starts unwinding the stack, and usually exits the program.
// Here, we explicitly call `panic` on our error condition:
//
// An explicit `panic` is mainly useful for tests and dealing with unrecoverable errors.
// For prototyping it can be useful, for example when dealing with functions that haven't been
// implemented yet, but in those cases the more descriptive `unimplemented` is better.
// In tests `panic` is a reasonable way to explicitly fail.

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("Aaaaaaaa");
    }

    println!("some refreshing {} is all I need.", beverage);
}
fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}
*/

// --------------------------------------------------------------------
// section 02 - abort and unwind

// The previous section illustrates the error handling mechanism `panic`.
// Different code paths can be conditionally compiled based on the panic setting.
// The current values available are `unwind` and `abort`.
//
// Building on the prior lemonade example, we explicitly use the panic strategy
// to exercise different lines of code.
/*
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("this is not your party. Run!!");
        } else {
            println!("spit it out");
        }
    } else {
        println!("some refreshing {} is all I need.", beverage);
    }
}
*/

/*
#[cfg(panic = "unwind")]
fn ah() {
    println!("spit it out");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("this is not your party. Run!!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("some refreshing {} is all I need", beverage);
    }
}
*/

/*
fn main() {
    drink("water");
    drink("lemonade");
}
*/

// --------------------------------------------------------------------
// section 03 - Option & unwrap

/*
// In the last example, we showed that we can induce program failure at will.
// We told our program to `panic` if we drink a sugary lemonade.
// But what if we expect some drink but don't receive one? That case would be just as bad,
// so it needs to be handled!
//
// We could test this against the null string (`""`) as we do with a lemonade.
// Since we're using Rust, let's instead have the compiler point out cases where there's no drink.
//
// An `enum` called `Option<T>` in the std library is used when absence is a possibility.
// It manifests itself as one of two "options":
//
// - `Some(T)`: An element of type T was found
// - `None``: No element was found
//
// These cases can either be explicitly handled via `match` or implicitly with `unwrap`.
// Implicit handling will either return the inner element or `panic`.
//
// Note that it's possible to manually customize `panic` with `expect`,
// but `unwrap` otherwise leaves us with a less meaningful output than explicit handling.
// In the following example, explicit handling yields a more controlled result
// while retaining the option to `panic` if desired.

fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("yuck. too sugary"),
        Some(inner) => println!("{}? how nice.", inner),
        None => println!("no drink? oh well"),
    }
}

fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("Aaaaaaa");
    }
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
*/

// --------------------------------------------------------------------
// section 04 - uOption & unwrap: unpacking options with `?`

/*
// You can unpack `Option`s by using match statements, but it's often easier to use the `?` operator.
// If `x` is an `Option`, then evaluating `x?` will return the underlying value if `x` is `Some`,
// otherwise it will terminate whatever function is being executed and return `None`.

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 2359364,
            }),
        }),
    };

    if let Some(code) = p.work_phone_area_code() {
        println!("{}", code);
    }

    let p = Person {
        job: Some(Job { phone_number: None }),
    };

    let code = p.work_phone_area_code();
    println!("{:?}", code);
}
*/

// --------------------------------------------------------------------
// section 05 - Option & unwrap: combinators: map

/*
// `match` is a valid method for handling `Option`s. However, you may eventually find heavy usage tedious,
// especially with operations only valid with an input. In these cases, combinators can be used to manage
// control flow in a modular fashion.
//
// `Option` has a built in method called `map()`, a combinator for the simple mapping of `Some` -> `Some`
// and `None` -> `None`. Multiple `map()` calls can be chained together for even more flexibility.
//
// In the following example, `process()` replaces all functions previous to it while staying compact.

#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// `map()` can be used instead of `match` for case handling
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// a function to peel, chop, and cook food all in sequence.
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm, I love {:?}", food),
        None => println!("Oh no! It wasn't edible"),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = Some(Food::Potato);
    let none = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(none);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}
*/

// --------------------------------------------------------------------
// section 06 - Option & unwrap: combinators: and_then
/*

#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

// we don't have the ingredients to make sushi
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// we have the recipe for everything excpet Cordon Bleu.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

// to make a dish, we need both the recipe and the ingredients
// match...
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => have_ingredients(food),
    }
}

// flatten() an Option<Option<Food>> to get Option<Food>
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
    // have_recipe(food).map(|f| have_ingredients(f)).unwrap()
}

// conveniently, rewrittten with and_then()
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat(food: Food, day: Day) {
    // match cookable_v1(food) {
    // match cookable_v2(food) {
    match cookable_v2(food) {
        Some(food) => println!("Yay!. On {:?} we get to eat {:?}", day, food),
        None => println!("Oh no. we don't get to eat on {:?}", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
*/

// --------------------------------------------------------------------
// section 07 - Option & unwrap: Defaults: or, or_else, get_or_insert, get_or_insert_with
/*
#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let none: Option<Fruit> = None;

    let first_available_fruit = none.or(orange).or(apple);
    println!("first available fruit: {:?}", first_available_fruit);

    // `or` moves its argument.
    // In the example above, `or(orange)` returned a `Some`, so `or(apple)` was not invoked.
    // But the variable named `apple` has been moved regardless, and cannot be used anymore.
    // println!(
    //     "Variable apple was moved, so this line won't compile: {:?}",
    //     apple
    // );
    // TODO: uncomment the line above to see the compiler error
}
*/

/*
#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

fn main() {
    let apple = Some(Fruit::Apple);
    let none: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("providing kiwi as a fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("providing lemon as a fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = none
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);

    println!("first available fruti: {:?}", first_available_fruit);
}
*/
/*
#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("first available fruit: {:?}", first_available_fruit);
    println!("my fruit: {:?}", my_fruit);
    // println!("Variable named `apple` is moved: {:?}", apple);
}
*/
/*
#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("providing lemon as fallback");
        Fruit::Lemon
    };

    // let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    // println!("first available fruit: {:?}", first_available_fruit);
    // println!("my fruit: {:?}", my_fruit);

    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should be apple: {:?}", should_be_apple);
    println!("my apple: {:?}", my_apple);
}
*/

// --------------------------------------------------------------------
// section 08 - Result : map for Result
/*
use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
    match first_num_str.parse::<i32>() {
        Ok(first_num) => match second_num_str.parse::<i32>() {
            Ok(second_num) => Ok(first_num * second_num),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() -> Result<(), ParseIntError> {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);

    Ok(())
}
*/
/*
use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
    first_num_str.parse::<i32>().and_then(|first_num| {
        second_num_str
            .parse::<i32>()
            .map(|second_num| first_num * second_num)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() -> Result<(), ParseIntError> {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);

    Ok(())
}
*/

// --------------------------------------------------------------------
// section 09 - Result : aliases for Result
/*
use std::num::ParseIntError;

// define alias
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_num_str: &str, second_num_str: &str) -> AliasedResult<i32> {
    first_num_str.parse::<i32>().and_then(|first_num| {
        second_num_str
            .parse::<i32>()
            .map(|second_num| first_num * second_num)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);
}
*/

// --------------------------------------------------------------------
// section 10 - early returns
/*
use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
    let first_num = match first_num_str.parse::<i32>() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };
    let second_num = match second_num_str.parse::<i32>() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };

    Ok(first_num * second_num)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);
}
*/

// --------------------------------------------------------------------
// section 11 - introducing `?`, `try!` macro
/*
// Sometimes we just want the simplicity of `unwrap` without the possibility of a `panic`.
// Until now, `unwrap` has forced us to nest deeper and deeper when what we really wanted
// was to get the variable out. This is exactly the purpose of `?`.
//
// Upon finding an `Err`, there are two valid actions to take:
// - `panic!` which we already decided to try to avoid if possible
// - `return` because an `Err` means it cannot be handled
// `?` is almost1 exactly equivalent to an `unwrap` which returns instead of panicking on `Errs`.
// Let's see how we can simplify the earlier example that used combinators:

use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
    let first_num = first_num_str.parse::<i32>()?;
    let second_num = second_num_str.parse::<i32>()?;

    Ok(first_num * second_num)
}

// Before there was `?`, the same functionality was achieved with the `try!` macro.
// The `?` operator is now recommended, but you may still find `try!` when looking at older code.
// The same multiply function from the previous example would look like this using `try!`:

// fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
//     let first_num = try!(first_num_str.parse::<i32>());
//     let second_num = try!(second_num_str.parse::<i32>());

//     Ok(first_num * second_num)
// }

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);
}
*/

// --------------------------------------------------------------------
// section 12 - Multiple error types: overview

/*
// The previous examples have always been very convenient; `Result`s interact with other `Result`s and `Option`s
// interact with other `Option`s. Sometimes an `Option` needs to interact with a `Result`, or a `Result<T, Error1>`
// needs to interact with a `Result<T, Error2>`. In those cases, we want to manage our different error types
// in a way that makes them composable and easy to interact with.
//
// In the following code, two instances of unwrap generate different error types.
// `Vec::first` returns an `Option`, while `parse::<i32>` returns a `Result<i32, ParseIntError>`:

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // generate error 1
    2 * first.parse::<i32>().unwrap() // generate error 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("first doubleis {}", double_first(numbers));
    println!("first doubleis {}", double_first(empty));
    println!("first doubleis {}", double_first(strings));
}
*/

// --------------------------------------------------------------------
// section 13 - Multiple error types: pulling results out of options
/*
use std::num::ParseIntError;

// fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
//     vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
// }

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("first doubleis {:?}", double_first(numbers));
    println!("first doubleis {:?}", double_first(empty));
    println!("first doubleis {:?}", double_first(strings));
}
*/

// --------------------------------------------------------------------
// section 14 - Multiple error types: defining an error type
/*
// Sometimes it simplifies the code to mask all of the different errors with a single type of error.
// We'll show this with a custom error.
//
// Rust allows us to define our own error types. In general, a "good" error type:
//
//   - Represents different errors with the same type
//   - Presents nice error messages to the user
//   - Is easy to compare with other types
//       - Good: `Err(EmptyVec)`
//       - Bad: `Err("Please use a vector with at least one element".to_owned())`
//   - Can hold information about the error
//       - Good: Err(BadChar(c, position))
//       - Bad: Err("+ cannot be used here".to_owned())
//   - Composes well with other errors

use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct DoubleError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // change the error to our new type.
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // update to the new error type here also.
                .map_err(|_| DoubleError)
                .map(|i| i * 2)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("the first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
*/

// --------------------------------------------------------------------
// section 15 - Multiple error types: boxing errors
/*
// A way to write simple code while preserving the original errors is to `Box` them.
// The drawback is that the underlying error type is only known at runtime and not statically determined.
// { ref: https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch }
//
// The stdlib helps in boxing our errors by having `Box` implement conversion from any type that
// implements the `Error` trait into the trait object `Box<Error>`, via From.

use std::error;
use std::fmt;

// change the alias to `Box<error::Error>`
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // converts to Box
                .map(|n| n * 2)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("the first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
*/

// --------------------------------------------------------------------
// section 16 - Multiple error types: other uses of `?`
/*
// Notice in the previous example that our immediate reaction to calling `parse` is to `map`
// the error from a library error into a boxed error:
// ```
// .and_then(|s| s.parse::<i32>())
//     .map_err(|e| e.into())
// ````
// Since this is a simple and common operation, it would be convenient if it could be elided.
// Alas, because `and_then` is not sufficiently flexible, it cannot. However, we can instead use `?`.
//
// `?` was previously explained as either `unwrap` or r`eturn Err(err)``. This is only mostly true.
// It actually means `unwrap` or `return Err(From::from(err))`. Since `From::from` is a conversion
// utility between different types, this means that if you `?` where the error is convertible to
// the return type, it will convert automatically.
//
// Here, we rewrite the previous example using `?`. As a result, the `map_err` will go away
// when `From::from` is implemented for our error type:

use std::error;
use std::fmt;

// change the alias to `Box<error::Error>`
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(parsed * 2)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("the first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
*/

// --------------------------------------------------------------------
// section 17 - Multiple error types: wrapping errors
/*
// an alternative to boxing errors is to wrpa them in your own error type.
use std::error;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // we will defer to the pase error implementation for their error.
    // supplying extra into requires adding more data to the type.
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
            // the wrapped error contains additional information and is available via the source() method
            DoubleError::Parse(..) => write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            // the cause is the underlying implementation error type. is implicitly cast to the
            // trait object `&error::Error`. this works because the underlying type already
            // implements the `Error` trait.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;

    Ok(parsed * 2)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("the first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!(" -> caused by: {}", source);
            }
        }
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
*/

// --------------------------------------------------------------------
// section 18 - Iterating over Results

fn main() {
    // `Iter::map`,
    // let strings = vec!["tofu", "93", "18"];
    // let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    // println!("result: {:?}", numbers);

    // `filter_map`, ignore the failed items
    // let strings = vec!["tofu", "93", "18"];
    // let numbers: Vec<_> = strings
    //     .into_iter()
    //     .filter_map(|s| s.parse::<i32>().ok())
    //     .collect();
    // println!("result: {:?}", numbers);

    // `filter_map` & `map_err`, collect the failed items
    // let strings = vec!["tofu", "93", "18"];
    // let mut errors = vec![];
    // let numbers: Vec<_> = strings
    //     .into_iter()
    //     .map(|s| s.parse::<u8>())
    //     .filter_map(|r| r.map_err(|e| errors.push(e)).ok()).collect();
    // println!("numbers: {:?}", numbers);
    // println!("errors: {:?}", errors);

    // fail the entire operation with `collect`
    // `Result` implements `FromIterator` so that a vector of results (`Vec<Result<T, E>>`) can be turned into
    // a result with a vector (`Result<Vec<T>, E>`). Once an `Result::Err` is found, the iteration will terminate.
    // let strings = vec!["tofu", "29", "93", "18"];
    // let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    // println!("result: {:?}", numbers);

    // collect all valid values and failures with `partition()`
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("numbers: {:?}", numbers);
    println!("errors: {:?}", errors);

    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("numbers: {:?}", numbers);
    println!("errors: {:?}", errors);
}
