// #region defooify-usage
// #region defooify
// Will this always work?
pub fn defooify(s: &str) -> String {
    s.replace("foo", "")
}
// #endregion defooify

// Yes!
pub fn main() {
    defooify("test");   // returns: test
    defooify("foobar"); // returns: bar
    // defooify(null);  // error[E0425]: cannot find value `null` in this scope
                        // There is no `null` in Rust.

                        // Please ignore std::ptr::null.
                        // It cannot be used here anyway:
                        // error[E0308]: mismatched types
                        // expected `&str`, found fn item
}
// #endregion defooify-usage

pub fn option() {
    // #region option
    let option: Option<String> = None;

    // Direct access to value is prohibited.
    if let Some(value) = option {
        println!["{}", value]; // never reached, but no error
    }
    // #endregion option
}

// #region never
pub trait Data<T> {
    fn get_value(self) -> T;
}

pub struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }
}

impl<T> Data<T> for Container<T> {
    fn get_value(self) -> T {
        self.value
    }
}

pub struct EmptyData;
impl Data<()> for EmptyData {
    fn get_value(self) -> () {
        () // "unit" type is used when there is never a value present.
    }
}
// #endregion never
