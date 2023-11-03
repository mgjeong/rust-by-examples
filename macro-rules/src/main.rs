// -------------------------------------------------------------------
// section 01. overview

/*
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // the macro will expand into the contents of this block
        println!("hello")
    };
}

fn main() {
    say_hello!();
}
*/

// -------------------------------------------------------------------
// section 02. syntax: patterns and designators
// referene: https://doc.rust-lang.org/reference/macros-by-example.html

// some of available designators:
// `block`
// `expr` is used for expressions
// `ident` is used for variable/function names
// `item`
// `literal` is used for literal constants
// `pat` (pattern)
// `path`
// `stmt` (statement)
// `tt` (token tree)
// `ty` (type)
// `vis` (visibility qualifier)

/*
// The arguments of a macro are prefixed by a dollar sign `$` and type annotated with a designator:
macro_rules! create_function {
    // this macro takes an arguement of designator `ident` and create a function named `$func_name`.
    ($func_name:ident) => {
        fn $func_name() {
            // the `stringify!` macro converts an `ident` into a string
            println!("you called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // this macro takes expression of type `expr` and prints it as a string along with its result
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}
*/

// -------------------------------------------------------------------
// section 03. syntax: overloadings
/*
macro_rules! test {
    // arguments don't need to be separated by a comma,
    // any template can be used.
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
*/

// -------------------------------------------------------------------
// section 04. syntax: repeatition

/*
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr), +) => (
        std::cmp::min($x, find_min!($($y), +))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
*/

// -------------------------------------------------------------------
// section 05. DRY( don't repeat yourself )

/*
// Macros allow writing DRY code by factoring out the common parts of functions and/or test suites.
// Here is an example that implements and tests the `+=`, `*=` and `-=` operators on Vec<T>:

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // the `tt` (toket tree) disignator is used for operators and tokens
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y)
            }
        }
    };
}

// implement `add_assign`, `mul_assign`, and `sub_assign` functions.
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();
                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        };
    }

    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}
*/

// -------------------------------------------------------------------
// section 06. DSLs( domain specific languages )

/*
// A DSL is a mini "language" embedded in a Rust macro.
// It is completely valid Rust because the macro system expands into normal Rust constructs,
// but it looks like a small language. This allows you to define concise or intuitive syntax
// for some special functionality (within bounds).
//
// Suppose that I want to define a little calculator API.
// I would like to supply an expression and have the output printed to console.

macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // force types to be integer
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}
fn main() {
    calculate! {
        eval 1 + 2 // eval is _not_ a Rust keyword!
    }
    calculate! {
        eval (1+2)*(3/4)
    }
}
*/

// -------------------------------------------------------------------
// section 07. variadic interface

/*
 */
// A variadic interface takes an arbitrary number of arguments.
// For example, `println!` can take an arbitrary number of arguments, as determined by the format string.
//
// We can extend our `calculate!` macro from the previous section to be variadic:

macro_rules! calculate {
    ( eval $e: expr ) => {
        {
            let val: usize = $e; // force types to be integer
            println!("{} = {}", stringify!($e), val);
        }
    };
    ( eval $e: expr, $(eval $es: expr), + ) => {{
        calculate! { eval $e }
        calculate! { $( eval $es ), + }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval ( 2 * 3 ) + 1
    }
}
