/**
 * @package     helloworld
 * @file        main.rs
 * @author      mj < mg.jeong@gmail.com >
 */

/// main
fn main() {
    // section 01 - print
    println!("Hello, world!");
    println!("I'm new rustacean!");

    // -----------------------------------------------------------------------------
    // section 02 - formatted print
    println!("{} days", 31i32); // can add type using suffix
    println!("{0}, this is {1}. {1}, this is {0}", "monster", "zombie"); // location

    // named parameter
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // output type format
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // align
    println!("{number:>width$}: right align", number = 1, width = 7);
    println!("{number:>7}: right align", number = 1);
    println!("{number:<width$}: left align", number = 1, width = 7);
    println!("{number:^width$}: center align", number = 1, width = 7);

    let width: usize = 9;
    println!("{number:->width$}: right align", number = 1);
    println!("{number:-<width$}: left align", number = 1);
    println!("{number:-^width$}: center align", number = 1);

    // the number of arguments
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // `std::fmt` has lots of traits, such as `fmt::Debug`, `fmt::Display`
    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This sturct `{}` won't print...", Structure(3));

    // Activities
    let pi: f64 = 3.1415926;
    println!("{:>9.3}", pi);

    // -----------------------------------------------------------------------------
    // section 03 - debug
}
