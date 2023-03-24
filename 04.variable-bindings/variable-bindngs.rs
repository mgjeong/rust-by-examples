/* section 01 - basic */

fn section01() {
    let an_integer = 1u32;
    let a_boolen = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolen);
    println!("meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;
}

/* section02 - mutability */

fn section02() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("immutable: {}", _immutable_binding);
    println!("mutable: {}", mutable_binding);

    mutable_binding = 2;
    println!("mmutable: {}", mutable_binding);
}

/* section03 - scope and binding */

fn section03() {
    let long_lived_binding = 1;
    println!("outer long: {}", long_lived_binding);
    {
        println!("inner long: {}", long_lived_binding);

        let short_lived_binding = 2;
        println!("short: {}", short_lived_binding);

        let long_lived_binding = 3.0_f32;
        println!("inner long: {:.2}", long_lived_binding);
    }

    println!("outer long: {}", long_lived_binding);

    // shadowing...
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}

// ToDo: practice section03,, declare first

fn main() {
    section01();
    section02();
    section03();
}
