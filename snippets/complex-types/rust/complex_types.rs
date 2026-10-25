// #region complex-type
struct A {
    i: i32,
}

impl A {
    // no `self` as parameter
    // -> associated function (`static`)
    pub fn construct_default() -> Self {
        Self::new(0)
    }

    // "constructor" is just an associated function
    pub fn new(i: i32) -> Self {
        Self {
            // like in JavaScript,
            // this is short for `i: i`
            i
        }
    }

    // destructor
    pub fn destroy(self) -> () {}

    pub fn get_i(&self) -> i32 {
        self.i
    }

    pub fn set_i(&mut self, i: i32) -> () {
        self.i = i;
    }
}
// #endregion complex-type

pub fn main() {
    // #region static
    // Associated function call `::`
    let _a = A::construct_default();
    // #endregion static

    // #region variable-getter-static-destruction
    // #region variable-getter-static
    // #region variable-getter
    // #region variable
    // "Constructor" call `::`
    let mut a: A = A::new(10);
    // The variable a contains
    // the data of the instance of A
    // that is stored on the stack.

    // #endregion variable

    println!("{}", a.get_i());
    // prints: 10
    // #endregion variable-getter

    // Instance methods can be invoked
    // using associated function call
    println!("{}", A::get_i(&a));
    // prints: 10
    // #endregion variable-getter-static

    // Manually destroy `a`
    a.destroy();
    // #endregion variable-getter-static-destruction
}
