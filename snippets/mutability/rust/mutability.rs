pub fn main() {
    // #region immutable-mutable-vec-compiletime
    // #region immutable-mutable-vec
    // #region immutable-mutable
    // #region immutable
    // Immutable by default
    let a = 0;
    // a = 1;
    // ↑ error[E0384]: cannot assign twice to immutable variable `a`
    // #endregion immutable

    // Make mutable with `mut`
    let mut b = 0;
    b = 1;
    // ↑ This works perfectly fine.
    // #endregion immutable-mutable

    // Immutability of a variable extends to its fields.
    let vec: Vec<i32> = Vec::new();
    // vec.push(1);
    // ↑ error[E0596]: cannot borrow `vec` as mutable, as it is not declared as mutable
    // #endregion immutable-mutable-vec



    // Mutation errors are caught at compile time.
    // #endregion immutable-mutable-vec-compiletime
}
