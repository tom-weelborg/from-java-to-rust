trait Long {
    fn variable_long() {
        // #region variable-long
        // Variable
        let i: i32 = 0;
        // #endregion variable-long
    }

    // #region function-long
    // Function
    fn double_value(i: i32) -> i32 {
        return i * 2;
    }
    // #endregion function-long
}

trait Short {
    fn variable_short() {
        // #region variable-short
        // Variable
        let i = 0; // uses type inference
        // #endregion variable-short
    }

    // #region function-short
    // Function
    fn double_value(i: i32) -> i32 {
        i * 2 // This return is idiomatic.
    }
    // #endregion function-short
}

pub fn main() {
    // #region variable-if-else-switch
    // #region variable-if-else
    // #region variable
    // Variable
    let i = 0;
    // #endregion variable

    // if-else
    if i == 0 {
        // happens if i == 0
    } else if i % 2 == 0 {
        // happens if i != 0 and i % 2 == 0
    } else {
        // happens if i != 0 and i % 2 != 0
    }
    // #endregion variable-if-else

    // match
    match i {
        0 => {
            // happens if i equals 0
        },
        1 | 2 | 3 => {
            // happens if i equals 1, 2 or 3
        },
        4..10 => {
            // happens if i equals 4, 5, 6, 7, 8 or 9
        },
        _ => {
            // happens if no other case matches
        }
    }
    // #endregion variable-if-else-switch
    
    // #region while-for
    // #region while
    // Mutable Variable
    let mut j = 0;

    // while loop
    while j < 10 {
        j += 1;
    }
    // #endregion while

    // for loop
    for k in 0..10 {
        // do something 10 times
    }
    // #endregion while-for
}

// #region function-constant
// #region function
// Function
fn double_value(i: i32) -> i32 {
    i * 2
}
// #endregion function

// Global Constant
const APPLICATION_NAME: &str = "Cool Software";
// #endregion function-constant
