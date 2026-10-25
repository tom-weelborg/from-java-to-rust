mod compile_time {
    // #region compile-time
    // At compile time
    pub struct Container<T> {
        value: T,
    }

    impl<T> Container<T> {
        pub fn new(value: T) -> Self {
            Self {
                value,
            }
        }

        pub fn get_value(&self) -> &T {
            &self.value
        }

        pub fn set_value(&mut self, value: T) {
            self.value = value;
        }
    }

    pub fn main() {
        let string: Container<String> = Container::new(String::from("string"));
        let integer: Container<i32> = Container::new(10);

        println!("{}", string.get_value());  // prints: string
        println!("{}", integer.get_value()); // prints: 10
    }
    // #endregion compile-time
}

mod runtime {
    // #region runtime
    // At runtime

    // not actually named that
    pub struct StringContainer {
        value: String,
    }

    // not actually named that
    pub struct I32Container {
        value: i32,
    }

    // There are also `impl` blocks created for both structs.







    
    pub fn main() {
        let string: StringContainer = StringContainer::new(String::from("string"));
        let integer: I32Container = I32Container::new(10);

        println!("{}", string.get_value());  // prints: string
        println!("{}", integer.get_value()); // prints: 10
    }
    // #endregion runtime

    impl StringContainer {
        pub fn new(value: String) -> Self {
            Self { value }
        }

        pub fn get_value(&self) -> &String {
            &self.value
        }
    }

    impl I32Container {
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        pub fn get_value(&self) -> &i32 {
            &self.value
        }
    }
}

mod compile_time_extra_method {
    // #region compile-time-extra-method
    // At compile time
    pub struct Container<T> {
        value: T,
    }

    impl<T> Container<T> {
        pub fn new(value: T) -> Self {
            Self {
                value,
            }
        }

        pub fn get_value(&self) -> &T {
            &self.value
        }

        pub fn set_value(&mut self, value: T) {
            self.value = value;
        }
    }

    impl Container<i32> {
        pub fn add_value(&mut self, value: i32) {
            self.value += value;
        }
    }


    pub fn main() {
        let mut string: Container<String> = Container::new(String::from("string"));
        // string.add_value(10);
        // ↑ error[E0599]: no method named `add_value` found
        //   for struct `Container<String>` in the current scope

        let mut integer: Container<i32> = Container::new(10);
        integer.add_value(10);

        println!("{}", string.get_value());  // prints: string
        println!("{}", integer.get_value()); // prints: 20
    }
    // #endregion compile-time-extra-method
}

pub fn main() {
    compile_time::main();
    runtime::main();
    compile_time_extra_method::main();
    type_safe();
}

// #region no-issues
fn type_safe() {
    let mut vec: Vec<i32> = Vec::new();

    add_elements(&mut vec);

    for e in vec.iter() {
        println!("{}", e);
    }
    // prints:
    // 1
    // 2
    // 3



    vec.iter().for_each(|e| println!("{}", e));
    // prints:
    // 1
    // 2
    // 3
}

// Using just `Vec` is not allowed and will not compile:
// error[E0107]: missing generics for struct `Vec`
fn add_elements(vec: &mut Vec<i32>) {
    vec.push(1);
    vec.push(2);
    vec.push(3);
    // vec.push("Hello, world!");
    // ↑ error[E0308]: mismatched types
    //   expected `i32`, found `&str`
}
// #endregion no-issues
