// -------------------------------------------------------------
// section 01. unit testing
/*
 */
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[allow(dead_code)]
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have squart root".to_owned())
    }
}

pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("divide by zero error");
    } else if a < b {
        panic!("divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_edd() {
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test]
    fn test_sqrt_err() -> Result<(), String> {
        let x: f64 = -2.0;
        assert_ne!(sqrt(x)?, 0.0);
        Ok(())
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}

// -------------------------------------------------------------
// section 02. documentation testing
/*
 */

// -------------------------------------------------------------
// section 03. integration testing
/*
 */

// -------------------------------------------------------------
// section 04. dev-dependencies
/*
 */

fn main() {}
