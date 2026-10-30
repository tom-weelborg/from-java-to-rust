pub fn main() {
    method_level::method_level();

    // unsafe { global_level::global_level(); }
}

mod method_level {
    // #region method
    // supported


    pub fn method_level() {
        let mut counter = 0;
        while {let c = counter; counter += 1; c < 2} {
            let counter = 0; // This is immutable.
            println!("{}", counter);
        }
    }
    // #endregion method
}

mod global_level {
    /*
    // #region global
    // not supported
    static mut counter: i32 = 0;

    pub unsafe fn global_level() {
        counter = 0;
        while {let c = counter; counter += 1; c < 2} {
            let counter = 0; // This is not allowed.
            println!("{}", counter);
        }
    }
    // #endregion global
    */
}
