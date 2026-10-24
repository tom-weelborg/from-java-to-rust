import java.util.ArrayList;
import java.util.List;

public class Mutability {
    public static void main(String[] args) {
        // #region mutable-immutable-list-unmodifiablelist
        // #region mutable-immutable-list
        // #region mutable-immutable
        // #region mutable
        // Mutable by default
        int a = 0;
        a = 1;
        // ↑ This works perfectly fine.
        // #endregion mutable

        // Make immutable with `final`
        final int b = 0;
        // b = 1;
        // ↑ error: cannot assign a value to final variable b
        // #endregion mutable-immutable

        /// Mutation of `final` objects is allowed.
        final List<Integer> list = new ArrayList<>();
        list.add(1);
        // ↑ This is allowed.
        // #endregion mutable-immutable-list

        final List<Integer> undmofiableList = List.of();
        undmofiableList.add(1);
        // ↑ throws UnsupportedOperationException at runtime
        // #endregion mutable-immutable-list-unmodifiablelist
    }
}
