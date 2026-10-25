public class Inheritance {
    public static class Sorting {
        // #region sorting
        public interface SortingStrategy {
            void sort(int[] arr); // no access modifier -> public
        }

        public static class BubbleSort implements SortingStrategy {
            @Override
            public void sort(int[] arr) {
                // ...
            }
        }

        public static class QuickSort implements SortingStrategy {
            @Override
            public void sort(int[] arr) {
                // ...
            }
        }

        public static class SortingMachine {
            private final SortingStrategy strategy;



            public SortingMachine(SortingStrategy strategy) {

                this.strategy = strategy;

            }

            public void sort(int[] arr) {
                this.strategy.sort(arr);
            }
        }
        // #endregion sorting

        public static void main(String[] args) {
            int[] arr = new int[] {
                5,
                4,
                3,
                2,
                1,
            };

            SortingMachine sortingMachineWithBubbleSort = new SortingMachine(new BubbleSort());
            sortingMachineWithBubbleSort.sort(arr);

            SortingMachine sortingMachineWithQuickSort = new SortingMachine(new QuickSort());
            sortingMachineWithQuickSort.sort(arr);
        }
    }

    public static class Constructor {
        public static class Color {
            public static final Color RED = new Color();
        };

        // #region constructor
        public abstract class Vehicle {
            protected final Color color;

            // We want every subclass to have a constructor like this.
            protected Vehicle(Color color) {
                this.color = color;
            }

            public Color getColor() {
                return this.color;
            }
        }

        public class Car extends Vehicle {

            public Car(Color color) {
                super(color);
            }




        }

        public class Truck extends Vehicle {
            // Nothing stops us from doing this.
            public Truck() {
                super(Color.RED);
            }




        }
        // #endregion constructor
    }

    public static class SelfReferencing {
        // #region self-referencing
        // `T extends Builder<T>` is complicated to read and understand.
        public interface Builder<T extends Builder<T>> {
            T name(String name);
        }

        public class PersonBuilder implements Builder<PersonBuilder> {

            @Override
            public PersonBuilder name(String name) {
                // store `name`
                return this;
            }
        }

        // The implemenation is wrong here.
        // It should be `implements Builder<MachineBuilder>`,
        // but nothing stops us from doing this:
        public class MachineBuilder implements Builder<PersonBuilder> {

            @Override
            public PersonBuilder name(String name) {
                // store `name`
                // return this;
                // ↑ This is an error here.
                return null;
            }
        }
        // #endregion self-referencing
    }

    public static void main(String[] args) {
        Sorting.main(args);
    }
}
