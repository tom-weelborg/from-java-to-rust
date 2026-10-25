pub fn main() {
    greet();
    invalid_greet();

    simple_counter::count();
    expanded_counter::count();
}

fn greet() {
    // #region greet
    let hello = "Hello, ";

    let greeter = |name| {
        let mut string = hello.to_string();
        string.push_str(name);
        string.push_str("!");
        return string;
    };

    println!["{}", greeter("World")];    // prints: Hello, World!
    println!["{}", greeter("Universe")]; // prints: Hello, Universe!
    // #endregion greet
}

fn invalid_greet() {
    // #region invalid-greet
    let hello = "Hello, ".to_string();

    let greeter = |name| {
        let mut string = hello;
        string.push_str(name);
        string.push_str("!");
        return string;
    };

    println!["{}", greeter("World")];
    // println!["{}", greeter("Universe")];
    // ↑ error[E0382]: use of moved value: `greeter`
    //   value used here after move
    //   note: closure cannot be invoked more than once
    //       because it moves the variable `hello` out of its environment
    // #endregion invalid-greet
}

mod simple_counter {
    // #region simple-counter
    struct Counter {
        counter: i32,
    }

    pub(super) fn count() {
        let mut counter = Counter { counter: 0 };

        let mut increment_counter = || counter.counter += 1;

        increment_counter();
        increment_counter();

        println!["{}", counter.counter]; // prints: 2
    }
    // #endregion simple-counter
}

mod expanded_counter {
    /*
    // #region expanded-counter-incorrect
    struct Counter {
        counter: i32,
    }

    pub(super) fn count() {
        let mut counter = Counter { counter: 0 };

        let mut increment_counter = || counter.counter += 1;
        let mut decrement_counter = || counter.counter -= 1;
        // error[E0499]: cannot borrow `counter` as mutable more than once at a time
        increment_counter();
        increment_counter();
        decrement_counter();
        decrement_counter();

        println!["{}", counter.counter];
    }
    // #endregion expanded-counter-incorrect
    */

    // #region expanded-counter-correct
    struct Counter {
        counter: i32,
    }

    pub(super) fn count() {
        let mut counter = Counter { counter: 0 };

        let increment_counter = |counter: &mut Counter| counter.counter += 1;
        let decrement_counter = |counter: &mut Counter| counter.counter -= 1;

        increment_counter(&mut counter);
        increment_counter(&mut counter);
        decrement_counter(&mut counter);
        decrement_counter(&mut counter);
        
        println!["{}", counter.counter]; // prints: 0
    }
    // #endregion expanded-counter-correct
}
