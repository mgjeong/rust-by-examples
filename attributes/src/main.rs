// ------------------------------------------------------------------------------------------------
// section 01. overview

// An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
//
// - conditional compilation of code
// - set crate name, version and type (binary or library)
// - disable lints (warnings)
// - enable compiler features (macros, glob imports, etc.)
// - link to a foreign library
// - mark functions as unit tests
// - mark functions that will be part of a benchmark
// - attribute like macros
// - When attributes apply to a whole crate, their syntax is #![crate_attribute], and when they apply to a module or item, the syntax is #[item_attribute] (notice the missing bang !).
//
// Attributes can take arguments with different syntaxes:
//
// `#[attribute = "value"]`
// `#[attribute(key = "value")]`
// `#[attribute(value)]`
//
// Attributes can have multiple values and can be separated over multiple lines, too:
//
// `#[attribute(value, value2)]`
// `#[attribute(value, value2, value3,value4, value5)]`
//

// ------------------------------------------------------------------------------------------------
// section 02. dead_code

/*
fn main() {
    println!("Hello, world!");
    used_function();
}

fn used_function() {
    println!("used_function");
}

#[allow(dead_code)]
fn unused_function() {
    println!("unused_function");
}

fn noise_unused_function() {
    println!("noisy_unused_function");
}
*/

// ------------------------------------------------------------------------------------------------
// section 03. crates

/*
// $ rustc main.rs
// do NOT use cargo command

// this crate is a library
#![crate_type = "lib"]
// this library name is "rary"
#![crate_name = "attributes"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    println!("called rary's `indirect_access()`, that\n>");

    private_function();
}
*/

// ------------------------------------------------------------------------------------------------
// section 04. cfg

/*
// Configuration conditional checks are possible through two different operators:
//
// the `cfg` attribute: `#[cfg(...)]` in attribute position
// the `cfg!`` macro: `cfg!(...)` in boolean expressions
// While the former enables conditional compilation,
// the latter conditionally evaluates to `true` or `false` literals allowing for checks at run-time.
// Both utilize identical argument syntax.
//
// `cfg!`, unlike `#[cfg]`, does not remove any code and only evaluates to `true` or `false`.
// For example, all blocks in an if/else expression need to be valid when `cfg!` is used for the condition,
// regardless of what `cfg!` is evaluating.

// this function only gets compiled if the target OS is linux
#[cfg(targeet_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("you are *not* running linux");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("yes, it's definitely linux");
    } else {
        println!("yes, it's definitely *not* linux");
    }
}
*/

// ------------------------------------------------------------------------------------------------
// section 05. cfg - custom
// see the custom.rs

fn main() {} //
