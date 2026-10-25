// #region union
pub enum Union {
    String(String),
    Number(f64)
}

pub fn print_union(union: Union) {
    match union {
        Union::String(v) => println!["The value is a string: {}", v],

        Union::Number(v) => println!["The value is a number: {}", v],
    }
}

pub fn main() {
    // prints: "The value is a string: Test string"
    print_union(Union::String(String::from("Test string")));
    // prints: "The value is a number: 42"
    print_union(Union::Number(42.0));
}
// #endregion union

mod alias_bad {
    // #region alias-bad
    // This is bad:
    pub enum I32Vec {
        Value(Vec<i32>)
    }
    // #endregion alias-bad
}

mod alias_good {
    // #region alias-good
    // Use this instead:
    type I32Vec = Vec<i32>;
    // #endregion alias-good
}
