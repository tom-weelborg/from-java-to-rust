pub fn main() {
    // #region valid-pointer
    let mut p: Box<i32> = Box::new(0);
    *p = 10;
    // #endregion valid-pointer
}
