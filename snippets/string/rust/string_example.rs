pub fn main() {
    a();

    b();

    concatenation();
}

fn a() {
    // #region a
    let slice: &str = "Hello, world!";
    // stored in the program's binary
    

    let string: String = String::from(slice);
    // stored on the stack
    // with a reference to the heap
    // containing a copy of the slice

    println!("{}", slice);
    // prints: "Hello, world!"

    println!("{}", string);
    // prints: "Hello, world!"
    // #endregion a
}

fn b() {
    // #region b
    // let string: String = "Hello, world!";
    // ↑ error[E0308]: mismatched types
    //   expected `String`, found `&str`
    // println!("{}", string);

    // #endregion b
}

fn concatenation() {
    // #region concatenation
    // Concatenation
    let mut string: String = String::from("Hello, ");
    string.push_str("world");
    string.push_str("!");
    println!("{}", string);
    // prints: "Hello, world!"
    // #endregion concatenation
}
