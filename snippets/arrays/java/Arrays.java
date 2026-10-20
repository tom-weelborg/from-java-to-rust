import java.util.Optional;

import static java.util.Optional.empty;
import static java.util.Optional.of;

public class Arrays {
    public static void main(String[] args) {
        // #region range-read-success
        // #region range-read-error
        // #region range
        // #region reference
        // #region invalid-read
        // #region read
        // #region declaration
        int[] arr;
        arr = new int[] {
            0,
            1,
            2,
            3,
            4,
        };
        arr = new int[5]; // equivalent to `new int[] {0,0,0,0,0}`
        // #endregion declaration

        int index = 3;
        int value = arr[index];
        System.out.println(value); // prints: 0
        // #endregion read

        index = 6; // valid `int` value, but outside array length
        // value = arr[index]; // fails at runtime, try uncommenting
        // ↑ throws ArrayIndexOutOfBoundsException
        //   Index 6 out of bounds for length 5
        // #endregion invalid-read
        
        int[] arr2 = null; // reference type
        // #endregion reference

        arr2 = java.util.Arrays.copyOfRange(arr, 1, 4);
        System.out.println(java.util.Arrays.toString(arr2)); // prints: [0, 0, 0]
        // #endregion range

        index = 3;
        // value = arr2[index]; // fails at runtime, try uncommenting
        // ↑ throws ArrayIndexOutOfBoundsException
        //   Index 3 out of bounds for length 3
        // #endregion range-read-error

        Optional<Integer> v = index < arr2.length ? of(arr2[index]) : empty(); // safe
        System.out.println(v); // prints: Optional.empty
        // #endregion range-read-success
    }
}
