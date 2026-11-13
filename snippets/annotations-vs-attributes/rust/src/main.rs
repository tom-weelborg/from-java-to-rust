use attributes::data_holder;

pub fn main() {
    old_method();
    new_method();

    method();

    without_derive::main();
    with_derive::main();

    stringify::stringify();

    copy::main();
    clone::main();
}

// #region deprecated

/// Prints "Hello, World!".
/// 
#[deprecated(note="
    This was the old way of doing that.
    Use `new_method` instead.",

    since="0.0.0")]
pub fn old_method() {
    println!("Hello, World!");
}


/// Prints "Hello, World!".
/// 
pub fn new_method() {
    println!("Hello, World!");
}
// #endregion deprecated

// #region usage
#[data_holder("Example for `method`")]
pub fn method() {
    println!("This function has an attribute.");
}
// #endregion usage

pub mod without_derive {
    // #region equals-noderive
    pub struct Example {
        a: i32,
        b: String,
    }

    // allows comparisons with == and !=
    // implies symmetry, transitivity, consistency
    impl PartialEq for Example {
        fn eq(&self, other: &Self) -> bool {



            self.a == other.a && self.b == other.b
        }
    }

    // like PartialEq, but also implies reflexivity
    impl Eq for Example {}
    // #endregion equals-noderive

    pub fn main() {
        let e1 = Example {
            a: 1,
            b: "Hello".to_string()
        };
        let mut e2 = Example {
            a: 1,
            b: "World".to_string()
        };

        println!("{}", e1 == e2);

        e2.b = "Hello".to_string();
        println!("{}", e1 == e2);
    }
}

pub mod with_derive {
    // #region equals-derive
    // requires every field to alos implement the used traits
    #[derive(PartialEq, Eq)]
    pub struct Example {
        a: i32,
        b: String,
    }
    // #endregion equals-derive

    pub fn main() {
        let e1 = Example {
            a: 1,
            b: "Hello".to_string()
        };
        let mut e2 = Example {
            a: 1,
            b: "World".to_string()
        };

        println!("{}", e1 == e2);

        e2.b = "Hello".to_string();
        println!("{}", e1 == e2);
    }
}

pub mod stringify {
    use std::{
        fmt::{
            Debug,
            Display,
            Formatter,
            Result,
        },
        string::ToString
    };

    // #region stringify
    // log this instance for the developer
    #[derive(Debug)]
    pub struct Example {
        a: i32,
        b: String,
    }

    // show this instance to the user
    // This automatically implements a trait called `ToString`.
    impl Display for Example {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}: {}", self.b, self.a)
        }
    }

    pub fn stringify() {
        let e = Example {
            a: 42,
            b: "Test".to_string()
        };

        println!("{:?}", e);           // prints: Example { a: 42, b: "Test" }
        println!("{}", e);             // prints: Test: 42
        println!("{}", e.to_string()); // prints: Test: 42
    }
    // #endregion stringify
}

pub mod copy {
    // #region copy
    #[derive(Clone, Copy)]
    pub struct Example {
        a: i32,
        // b: String, // `String` does not implement `Copy`.
    }









    pub fn main() {
        let e1 = Example {
            a: 42,

        };
        let mut e2 = e1; // Implicit copy

        // `e1` is still valid.
        println!("{}", e1.a); // prints: 42
        println!("{}", e2.a); // prints: 42

        e2.a = 10;
        println!("{}", e1.a); // prints: 42
        println!("{}", e2.a); // prints: 10
    }
    // #endregion copy
}

pub mod clone {
    // #region clone
    #[derive(Clone)]
    pub struct Example {
        a: i32,
        b: String,
    }









    pub fn main() {
        let e1 = Example {
            a: 42,
            b: "Test".to_string()
        };
        let mut e2 = e1.clone(); // explicit clone

        // `e1` is still valid.
        println!("{}", e1.a); // prints: 42
        println!("{}", e2.a); // prints: 42

        e2.a = 10;
        println!("{}", e1.a); // prints: 42
        println!("{}", e2.a); // prints: 10
    }
    // #endregion clone
}
