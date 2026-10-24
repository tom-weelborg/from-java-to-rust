public class Control {
    public static void main(String[] args) {
        // #region variable-if-else-switch
        // #region variable-if-else
        // #region variable
        // Variable
        int i = 0;
        // #endregion variable

        // if-else
        if (i == 0) {
            // happens if i == 0
        } else if (i % 2 == 0) {
            // happens if i != 0 and i % 2 == 0
        } else {
            // happens if i != 0 and i % 2 != 0
        }
        // #endregion variable-if-else

        // switch-case
        switch (i) {
            case 0:
                // happens if i equals 0
                break;
            case 1, 2, 3:
                // happens if i equals 1, 2 or 3
                break;



            default:
                // happens if no other case matches
                break;
        }
        // #endregion variable-if-else-switch

        // #region while-for
        // #region while
        // Mutable Variable
        int j = 0;

        // while loop
        while (j < 10) {
            j++;
        }
        // #endregion while

        // for loop
        for (int k = 0; k < 10; k++) {
            // do something 10 times
        }
        // #endregion while-for
    }

    // #region function-constant
    // #region function
    // Function
    static int doubleValue(int i) {
        return i * 2;
    }
    // #endregion function

    // Global Constant
    public static final String APPLICATION_NAME = "Cool Software";
    // #endregion function-constant
}
