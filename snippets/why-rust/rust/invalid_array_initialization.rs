pub fn main() {
    // #region invalid-array-initialization
    let mut arr: [i32; 1] = None;
    // ↑ error[E0308]: expected `[i32; 1]`, found `Option<_>`
    //   This does not compile as types are not nullable in Rust.
    arr[10] = 10;
    // #endregion invalid-array-initialization
}
