pub fn main() {
    // #region range-read-success
    // #region range-read-error
    // #region range
    // #region reference
    // #region invalid-read
    // #region read
    // #region declaration
    let mut arr: [i32; 5]; // note the length is part of the type
    arr = [
        0,
        1,
        2,
        3,
        4,
    ];
    arr = Default::default(); // equivalent to `[0,0,0,0,0]`
    // #endregion declaration

    let mut index: usize = 3; // type `usize` (not `i32` as would be equivalent to Java)
    let mut value: i32 = arr[index];
    println!("{}", value); // prints: 0
    // #endregion read

    index = 6; // valid `usize` value, but outside array length
    // value = arr[index]; // fails at compile time, try uncommenting
    // ↑ error: this operation will panic at runtime
    //   index out of bounds: the length is 5 but the index is 6
    // #endregion invalid-read

    let mut arr2: &[i32] = &arr; // slice, notice that there is no length
    // #endregion reference

    arr2 = &arr[1..4];
    println!("{:?}", arr2); // prints: [0, 0, 0]
    // #endregion range

    index = 3;
    // value = arr2[index]; // unfortunately also fails at runtime, try uncommenting
    // ↑ thread 'main' (36784) panicked
    //   index out of bounds: the len is 3 but the index is 3
    // #endregion range-read-error

    let v: Option<&i32> = arr2.get(index); // safe
    println!("{:?}", v); // prints: None
    // #endregion range-read-success
}
