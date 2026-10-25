// #region enum


pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

impl WeekDay {
    pub fn is_weekend_day(&self) -> bool {
        match self {
            Self::Saturday => true,
            Self::Sunday => true,
            _ => false,
        }
    }
}
// #endregion enum

// #region result-types
pub enum Result<T> {
    Success(T),
    Error(String)
}

impl<T> Result<T> {
    pub fn success(value: T) -> Result<T> {
        Self::Success(value)
    }

    pub fn error(error_message: String) -> Result<T> {
        Self::Error(error_message)
    }
}
// #endregion result-types

pub fn main() {
    // #region result-usage
    let success: Result<i32>
        = Result::success(42);
    let error: Result<i32>
        = Result::error("This has failed.".to_string());

    // A check is required, either match...
    match success {
        Result::Success(v)
            => println!["The answer is: {}", v],
            // printed
        Result::Error(m)
            => println!["{}", m],
            // not printed
    }

    // ... or if.
    if let Result::Success(v) = error {
        println!["The answer is: {}", v];
        // not printed
    }
    if let Result::Error(m) = error {
        println!["{}", m];
        // printed
    }

    // Without checks,
    // neither the value v
    // nor the error message m
    // are accessible.
    // #endregion result-usage
}
