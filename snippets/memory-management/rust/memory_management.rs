pub fn main() {
    let mut example: String = String::from("example");
    // This variable is owned by `main`

    example = example1(example);
    // Ownership of `example` is transferred to function `example1` and then transferred back.

    example2(&example);
    // The variable `example` is borrowed to `example2`. No reassignment is required.

    example1(example);
    // Ownership of `example` is transferred to function `example1` and not transferred back.

    // println!("{}", example);
    // ↑ error[E0382]: borrow of moved value: `example`
    //   This is now illegal as `example` is no longer a valid variable here.

    do_nothing();
}

fn example1(example: String) -> String {
    println!("{}", example);
    return example;
    // Ownership of `example` is transferred back to the caller.
}

fn example2(example: &String) {
    println!("{}", example);
}

fn do_nothing() {
    // The variable `example` is long gone.
}
