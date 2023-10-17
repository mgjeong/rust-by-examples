// Conversion
//
// Primitive types can be converted to each other through casting.
// Rust addresses conversion between custom types (i.e., struct and enum) by the use of traits.
// The generic conversions will use the From and Into traits. However there are more specific ones
// for the more common cases, in particular when converting to and from Strings.

// --------------------------------------------------------------------------------
// section 1: From and Into

/*
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32, // check the warning...
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("num is {:?}", num);

    let num2 = Number { value: 15 };
    println!("num2 is {:?}", num2);
}

*/

/*
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

fn main() {
    let int = 5;
    let num: Number = int.into();
    println!("num is {:?}", num);
    println!("the value of num is : {}", num.value);
}
*/

// --------------------------------------------------------------------------------
// section 2: TryFrom and TryInto

// Similar to From and Into, TryFrom and TryInto are generic traits for converting between types.
// Unlike From/Into, the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.

/*
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let res = EvenNumber::try_from(4);
    println!("{:?}", res);
    match res {
        Ok(num) => println!("value is {}", num.0),
        Err(()) => println!("not an even number"),
    }

    let res = EvenNumber::try_from(3);
    match res {
        Ok(num) => println!("value is {}", num.0),
        Err(()) => println!("not an even number"),
    }

    let num3 = EvenNumber(1);
    println!("value is {}", num3.0);

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    match result {
        Ok(num) => println!("value is {}", num.0),
        _ => println!("Not even number"),
    }
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
*/

// --------------------------------------------------------------------------------
// section 3: To and From String

/* */
use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    // converto to string
    let circle = Circle { radius: 7 };
    println!("{}", circle.to_string());
    println!("{}", circle);

    // parsing a string
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("sum: {:?}", sum);

    let float: f64 = "3.14".parse().unwrap();
    println!("{}", float);
}
