import java.util.Optional;

public class Nullability {
    // #region defooify-usage
    // #region defooify
    // Will this always work?
    public static String defooify(String s) {
        return s.replaceAll("foo", "");
    }
    // #endregion defooify

    // No!
    public static void main(String[] args) {
        defooify("test");   // returns: test
        defooify("foobar"); // returns: bar
        defooify(null);     // throws: NullPointerException at runtime
    }
    // #endregion defooify-usage

    public static void optional() {
        // #region optional
        Optional<String> optional = Optional.empty();

        optional.get();        // throws: NoSuchElementException at runtime
        // #endregion optional
    }

    // #region never
    public interface Data<T> {
        T getValue();
    }

    public class Container<T> implements Data<T> {
        private final T value;



        public Container(T value) {

            this.value = value;

        }


        @Override
        public T getValue() {
            return this.value;
        }
    }

    public class EmptyData implements Data<Object> {
        @Override
        public Object getValue() {
            return null;
        }
    }
    // #endregion never
}
