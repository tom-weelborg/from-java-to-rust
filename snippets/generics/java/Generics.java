import java.util.ArrayList;
import java.util.List;

public class Generics {
    public static class CompileTime {
        // #region compile-time
        // At compile time
        public static class Container<T> {
            private T value;



            public Container(T value) {

                this.value = value;

            }

            public T getValue() {
                return this.value;
            }

            public void setValue(T value) {
                this.value = value;
            }
        }

        public static void main(String[] args) {
            Container<String> string = new Container<>("string");
            Container<Integer> integer = new Container<>(10);

            System.out.println(string.getValue());  // prints: string
            System.out.println(integer.getValue()); // prints: 10
        }
        // #endregion compile-time
    }

    public static class Runtime {
        // #region runtime
        // At runtime
        public static class Container {
            private Object value;



            public Container(Object value) {

                this.value = value;

            }

            public Object getValue() {
                return this.value;
            }

            public void setValue(Object value) {
                this.value = value;
            }
        }

        public static void main(String[] args) {
            Container string = new Container("string");
            Container integer = new Container(10);

            System.out.println(string.getValue());  // prints: string
            System.out.println(integer.getValue()); // prints: 10
        }
        // #endregion runtime
    }

    public static class CompileTimeExtraMethod {
        // #region compile-time-extra-method
        // At compile time
        public static class Container<T> {
            private T value;



            public Container(T value) {

                this.value = value;

            }

            public T getValue() {
                return this.value;
            }

            public void setValue(T value) {
                this.value = value;
            }

            @SuppressWarnings("unchecked")
            public void addValue(int value) {
                if (this.value instanceof Integer i) {
                    this.value = (T) Integer.valueOf(i + value);
                }
            }
        }

        public static void main(String[] args) {
            Container<String> string = new Container<>("string");
            string.addValue(10);
            // Does nothing.
            // (Could also be changed to throw exception.)

            Container<Integer> integer = new Container<>(10);
            integer.addValue(10);
            
            System.out.println(string.getValue());  // prints: string
            System.out.println(integer.getValue()); // prints: 20
        }
        // #endregion compile-time-extra-method
    }

    public static void main(String[] args) {
        CompileTime.main(args);
        Runtime.main(args);
        CompileTimeExtraMethod.main(args);
        notTypeSafe();
    }

    // #region issues
    private static void notTypeSafe() {
        List<Integer> list = new ArrayList<>();

        addElements(list);

        for (Object e : list) {
            System.out.println(e);
        }
        // prints:
        // 1
        // 2
        // 3
        // Hello, world!
        // This works with any type.

        list.forEach(System.out::println);
        // ↑ throws ClassCastException at runtime

     
        
    }


    @SuppressWarnings({"rawtypes", "unchecked"}) // These are just warnings, not errors.
    private static void addElements(List list) {
        list.add(1);
        list.add(2);
        list.add(3);
        list.add("Hello, world!");
        list.add(new Object() {
            @Override
            public String toString() {
                return "This works with any type.";
            }
        });
    }
    // #endregion issues
}
