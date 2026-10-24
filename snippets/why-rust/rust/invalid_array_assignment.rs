pub fn main() {
    // #region invalid-array-assignment
    let mut arr: [i32; 1] = [0];
    arr[10] = 10;
    // ↑ error: this operation will panic at runtime
    //   This does not compile as it is obvious to the compiler
    //   that this will fail at runtime.
    // #endregion invalid-array-assignment
}
