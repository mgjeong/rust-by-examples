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
// section 08 - Result

// --------------------------------------------------------------------
// section 09 - Multiple error types

// --------------------------------------------------------------------
// section 10 - Iterating over Results
