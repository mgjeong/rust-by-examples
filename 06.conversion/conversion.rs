/* section 01 - From and Into */

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn section01() {
    println!("-- section 01 ------");
    let num = Number::from(30);
    println!("My number is {:?}", num);
    println!("number: {}", num.value);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

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

fn section02() {
    println!("-- section 02 ------");
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

/* section 03 - To and from Strings */

use std::fmt;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn section03() {
    println!("-- section 03 ------");
    let circle: Circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    println!("{:?}", circle);
    println!("{}", circle);
}

/* section 04 - parsing a string */

fn section04() {
    println!("-- section 04 ------");
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("sum: {}", sum);

    // activity
    let num: f32 = "5.5".parse().unwrap();
    println!("num = {}", num);
    // let num: i32 = "5.5".parse().unwrap(); // error!
    // let num: i32 = "5.5".parse::<i32>().unwrap(); // error!
    // let num: i32 = "5.5".parse::<f32>().unwrap(); // error!
    println!("num = {}", num);
}

fn main() {
    println!("-- main ---------");
    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("my_str: {}, my_string: {}", my_str, my_string);

    section01();
    section02();
    section03();
    section04();
}
