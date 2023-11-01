// -------------------------------------------------------------------
// section 01. overview

// greeting....
// 수고가 많다... 고생하고 있다... 다 알고 있다...
//
// 지난 번에 변화에 대해서 잠깐 얘기했었는데, 어떤 변화를 생각하고 있는지?
// 해외 branch 줄이면, 투자자가 안좋게 본다.
//
// 개발팀과 일부 pmo 인원들과 애기를 좀 나누었다.. 그 상황을 좀 공유하고 싶다.
// 조금씩은 개인별로 그리고 시니어와 주니어별로 다르긴 하지만, 주로 공통적인 생각들이 있고,
// 어떤이는 대표 앞에서 얘기할 수 있지만, 어떤이는 앞에서는 얘기하지 않는다..
//
// 1. 우선 현재 대표가 투자 유치활둉 하는 것은 다 응원하고 있다.
// 2. 동시에, 조속히 실행되지 않을 경우 법적인 절차를 진행하는 것은 응원하는 마음과는 별도이다. 모두 그럴것이다.
// 3. 많은 사람들이 지난 달 까지를 마지막으로 생각하고 있었다.
//    이미 사기가 다 떨어졌다.. 급여 지급만이 해결책이다.
// 4. 그럼에도 불구하고, 투자 유치하고 밀린 급여 이체되면 많은 사람들이 떠날 것이다.
//    ( 물론 아닐수도 있지만, 그건 결과론이고, 지금은 사람들이 다 그렇게 생각하고/말하고 있다.)
// 5. 어떻게 하면 계속 함께할 수 있는지를 물어보면, 한결같은 대답이 돌아온다.
//    회사의 운영 방법이 변화해야 한다.
//    이 부분에서, 스타트업은 대표의 의기가 중요하고, 대표가 방향을 잡으니까,
//    지금까지는 맘에 들지 않더라도 다 따랐다, 그런데 이번 상황을 겪고 나서는,
//    본인의 생각과 부합하지 않는다면, 이직할 것이라는 얘기들을 한다.
//    - in detail,,,
//        . biz-oriented 개발( product이 제일 우선이다.. ) // 이걸 pitching에 활용한다.
//          실제 product에 반영한다고 했을 때 WoW 할 수 있는 것들을 구성원들이 이해해야 한다.
//        . 비용 효율화는 왜 안하는지... ( office shutdown )
// 6. 8월 이전으로 돌아가는 건 불가능하다, 모두 다 함께 갈 수 없다.
// 7. 왜 이런 상황에 처하게 되었는지, 원인을 분석해서, 문제를 찾고, 그 문제를 해결하기 위해 어떻게 해야하는지는 방법을 찾아야 한다.
// 8. 의사 결정 협의체(Jin, DJ, Soong, MJ,,) 구성해서,
//    향후 회사의 운영 방안데 대해서 논의하고 이를 투자유치가 성공할 때 전체 공유해야 한다.
//    - in detail,,,
//        . 누구를 꼭 잡아야 하는지...
//        . 비용 효율화는 어떻게 할지...
//        . Product 전략은 어떻게 할지...( 지금 biz는 working 하지 않는다.. )
//    그래야 지금까지 함께 해온 사람들( knowhow )이 계속 함께 갈 수 있다.
// 9. 지금 어떻게 해야 한다를 얘기하는 건 아니니까,, 그렇게 할 수 있게, rebuilding을 할 수 있게, direction 주어야 한다.

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
 */

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
