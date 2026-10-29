pub fn main() {
    println!("{:?}", with_optional::parse("12:00".to_owned()));

    println!("{:?}", with_errors::parse("12:00".to_owned()));

    panic::main();
}

mod with_optional {
    #[derive(Debug)]
    // #region with-optional
    pub struct Time { hour: u8, minute: u8 }

    // common pattern in Rust
    pub fn parse(time_string: String) -> Option<Time> {
        // `time_string` is not nullable.



        if !validate_time_string(&time_string) {
            return None;
        }

        let length = time_string.len();

        let hour = time_string[0..(length - 3)].parse::<u8>().ok()?;
        let minute = time_string[(length - 2)..length].parse::<u8>().ok()?;

        if hour > 23 || minute > 59 { // can be simplified, because `u8` does not allow values < 0
            return None;
        }

        Some(Time { hour, minute })
    }

    // Only allows strings in format "6:00" or "18:30"
    fn validate_time_string(time_string: &str) -> bool {
        let length = time_string.len();
        if length != 4 && length != 5 {
            return false;
        }


        return time_string.chars().nth(length - 3) == Some(':');


    }
    // #endregion with-optional
}

mod with_errors {
    use std::num::ParseIntError;

    #[derive(Debug)]
    // #region with-errors
    pub struct Time { hour: u8, minute: u8 }

    // checks are required, all error cases need to be addressed
    pub fn parse(time_string: String) -> Result<Time, TimeParseError> {
        // `time_string` is not nullable.



        if !validate_time_string(&time_string) {
            return Err(TimeParseError::TimeFormat);
        }

        let length = time_string.len();

        let hour = time_string[0..(length - 3)].parse::<u8>()?;
        let minute = time_string[(length - 2)..length].parse::<u8>()?;

        if hour > 23 || minute > 59 { // can be simplified, because `u8` does not allow values < 0
            return Err(TimeParseError::TimeValue);
        }

        Ok(Time { hour, minute })
    }

    // Only allows strings in format "6:00" or "18:30"
    fn validate_time_string(time_string: &str) -> bool {
        let length = time_string.len();
        if length != 4 && length != 5 {
            return false;
        }


        return time_string.chars().nth(length - 3) == Some(':');


    }

    pub enum TimeParseError { TimeFormat, TimeValue }
    impl From<ParseIntError> for TimeParseError {
        fn from(_error: ParseIntError) -> Self { Self::TimeFormat }
    }
    // #endregion with-errors
    impl std::fmt::Debug for TimeParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::TimeFormat => write!(f, "TimeFormat"),
                Self::TimeValue => write!(f, "TimeValue"),
            }
        }
    }
}

// Example taken from here: https://doc.rust-lang.org/rust-by-example/std/panic.html
mod panic {
    // #region panic
    pub fn main() {

        division(10, 0);
        




        division(20, 0); // This will not be executed.



    }

    fn division(dividend: i32, divisor: i32) -> i32 {

        if divisor == 0 {

            panic!("division by zero");

        } else {
            dividend / divisor
        }
    }
    // #endregion panic
}
