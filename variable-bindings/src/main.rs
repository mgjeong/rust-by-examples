// -------------------------------------------------------------------------------
// section 1: overview
/*
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;
    println!("an integer: {:?}", copied_integer);
    println!("a boolean: {:?}", a_boolean);
    println!("meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32; // add underscore ...
}
*/

// -------------------------------------------------------------------------------
// section 2: mutability
/*
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    _immutable_binding += 1; // ERROR!!!!
}
*/

// -------------------------------------------------------------------------------
// section 3: scope and shadowing
/*
fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding); // ERROR
    println!("outer long: {}", long_lived_binding);

    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside the block: {}", shadowed_binding);

    let shadowed_binding = 2;

    println!("shadowed in outer block: {}", shadowed_binding);
}
*/

// -------------------------------------------------------------------------------
// section 4: declare first
/*
fn main() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding = {}", a_binding);

    let another_binding;
    // println!("another binding: {}", another_binding); // ERROR... uninitialized...

    another_binding = 1;
    println!("another binding: {}", another_binding);
}
*/

// -------------------------------------------------------------------------------
// section 5: freezing
/* */
fn main() {
    let mut _mutable_integer = 7i32;
    assert_eq!(7, _mutable_integer);

    {
        let _mutable_integer = _mutable_integer;
        assert_eq!(7, _mutable_integer);
        // _mutable_integer = 50; // ERROR...
    }

    _mutable_integer = 3;
    assert_eq!(3, _mutable_integer);
}
