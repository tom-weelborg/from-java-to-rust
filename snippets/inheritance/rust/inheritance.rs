mod sorting {
    mod invalid {
        /*
        // #region sorting-invalid
        pub trait SortingStrategy {
            fn sort(&self, arr: &mut [i32]); // no access modifier allowed -> taken from trait
        }

        pub struct BubbleSort;
        impl SortingStrategy for BubbleSort {
            fn sort(&self, arr: &mut [i32]) {
                // ...
            }
        }

        pub struct QuickSort;
        impl SortingStrategy for QuickSort {
            fn sort(&self, arr: &mut [i32]) {
                // ...
            }
        }

        pub struct SortingMachine {
            strategy: SortingStrategy, // This does not compile.
        }

        impl SortingMachine {
            pub fn new(strategy: SortingStrategy) -> Self {
                Self {
                    strategy,
                }
            }

            pub fn sort(&self, arr: &mut [i32]) {
                self.strategy.sort(arr);
            }
        }
        // #endregion sorting-invalid
        */
    }

    mod option1 {
        // #region sorting-option1
        pub trait SortingStrategy {
            fn sort(&self, arr: &mut [i32]);
        }

        pub struct BubbleSort;
        impl SortingStrategy for BubbleSort {
            fn sort(&self, arr: &mut [i32]) {
                // ...
            }
        }

        pub struct QuickSort;
        impl SortingStrategy for QuickSort {
            fn sort(&self, arr: &mut [i32]) {
                // ...
            }
        }

        pub struct SortingMachine {
            strategy: Box<dyn SortingStrategy>, // Option 1: Pointer to the heap
        }

        impl SortingMachine {
            pub fn new(strategy: Box<dyn SortingStrategy>) -> Self {
                Self {
                    strategy,
                }
            }

            pub fn sort(&self, arr: &mut [i32]) {
                self.strategy.sort(arr);
            }
        }
        // #endregion sorting-option1

        pub fn main() {
            let mut arr = [
                5,
                4,
                3,
                2,
                1,
            ];

            let sorting_machine_with_bubble_sort = SortingMachine::new(Box::new(BubbleSort {}));
            sorting_machine_with_bubble_sort.sort(&mut arr);

            let sorting_machine_with_quick_sort = SortingMachine::new(Box::new(QuickSort {}));
            sorting_machine_with_quick_sort.sort(&mut arr);
        }
    }

    mod option2 {
        // #region sorting-option2
        pub trait SortingStrategy {
            fn sort(&self, arr: &mut [i32]);
        }

        pub struct BubbleSort;
        impl SortingStrategy for BubbleSort {
            fn sort(&self, arr: &mut [i32]) {
                // ...
            }
        }

        pub struct QuickSort;
        impl SortingStrategy for QuickSort {
            fn sort(&self, arr: &mut [i32]) {
                // ...
            }
        }

        pub struct SortingMachine<T: SortingStrategy> {
            strategy: T, // Option 2: Generic
        }

        impl<T: SortingStrategy> SortingMachine<T> {
            pub fn new(strategy: T) -> Self {
                Self {
                    strategy,
                }
            }

            pub fn sort(&self, arr: &mut [i32]) {
                self.strategy.sort(arr);
            }
        }
        // #endregion sorting-option2

        pub fn main() {
            let mut arr = [
                5,
                4,
                3,
                2,
                1,
            ];

            let sorting_machine_with_bubble_sort = SortingMachine::new(BubbleSort {});
            sorting_machine_with_bubble_sort.sort(&mut arr);

            let sorting_machine_with_quick_sort = SortingMachine::new(QuickSort {});
            sorting_machine_with_quick_sort.sort(&mut arr);
        }
    }

    pub fn main() {
        option1::main();
        option2::main();
    }
}

mod constructor {
    pub struct Color;

    // #region constructor
    pub trait Vehicle {



        fn new(color: Color) -> Self;



        fn get_color(&self) -> &Color;


    }

    pub struct Car { color: Color }
    impl Vehicle for Car {
        fn new(color: Color) -> Self {
            Self { color }
        }
        
        fn get_color(&self) -> &Color {
            &self.color
        }
    }

    pub struct Truck { color: Color }
    impl Vehicle for Truck {
        fn new(color: Color) -> Self {
            Self { color }
        }
        
        fn get_color(&self) -> &Color {
            &self.color
        }
    }
    // #endregion constructor
}

mod self_referencing {
    // #region self-referencing
    // You can just make it return `&Self`
    pub trait Builder {
        fn name(&self, name: String) -> &Self;
    }

    pub struct PersonBuilder;
    impl Builder for PersonBuilder {

        fn name(&self, name: String) -> &PersonBuilder {
            // store `name`
            self
        }
    }



    
    pub struct MachineBuilder;
    impl Builder for MachineBuilder {
        // Only return values `&Self` or `&MachineBuilder` will compile here.
        fn name(&self, name: String) -> &MachineBuilder {
            // store `name`
            self


        }
    }
    // #endregion self-referencing
}

pub fn main() {
    sorting::main();
}
