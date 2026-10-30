public class Shadowing {
    public static void main(String[] args) {
        // MethodLevel.methodLevel();

        ClassLevel.classLevel();
    }

    static class MethodLevel {
        /*
        // #region method
        // not supported


        public static void methodLevel() {
            int counter = 0;
            while (counter++ < 2) {
                int counter = 0; // This is not allowed.
                System.out.println(counter);
            }
        }
        // #endregion method
        */
    }

    static class ClassLevel {
        // #region class
        // supported
        static int counter = 0;

        public static void classLevel() {
            counter = 0;
            while (counter++ < 2) {
                int counter = 0;
                System.out.println(counter);
            }
        }
        // #endregion class
    }
}
