import java.util.Optional;

public class ErrorHandling {
    public static void main(String[] args) {
        try {
            System.out.println(WithOptional.parse("12:00"));
            System.out.println(WithOptional.parse(null));

            System.out.println(WithErrors.parse("12:00"));
            System.out.println(WithErrors.parse(null));
        } catch (Exception e) {
            System.out.println("An exception was caught: " + e.getClass() + " - " + e.getMessage());
        }

        ThrowCatch.main();

        Exit.main();
    }

    static class WithOptional {
        // #region with-optional
        public record Time(byte hour, byte minute) {}

        // uncommon pattern in Java
        public static Optional<Time> parse(String timeString) {
            if (timeString == null) { // Everything is nullable in Java.
                return Optional.empty();
            }
            
            if (!validateTimeFormat(timeString)) {
                return Optional.empty();
            }

            int length = timeString.length();

            byte hour = Byte.parseByte(timeString.substring(0, length - 3));
            byte minute = Byte.parseByte(timeString.substring(length - 2, length));

            if ((hour < 0 || hour > 23) || (minute < 0 || minute > 59) ) {
                return Optional.empty();
            }

            return Optional.of(new Time(hour, minute));
        }

        // Only allows strings in format "6:00" or "18:30"
        private static boolean validateTimeFormat(String timeString) {
            int length = timeString.length();
            if (length != 4 && length != 5) {
                return false;
            }
            return Character.isDigit(timeString.charAt(length - 1)) &&
                   Character.isDigit(timeString.charAt(length - 2)) &&
                   timeString.charAt(length - 3) == ':' &&
                   Character.isDigit(timeString.charAt(length - 4)) &&
                   Character.isDigit(timeString.charAt(0));
        }
        // #endregion with-optional
    }

    static class WithErrors {
        // #region with-errors
        public record Time(byte hour, byte minute) {}

        // checks are not required, possibly fails at runtime
        public static Time parse(String timeString)
            throws NullPointerException, // Everything is nullable in Java.
                   TimeFormatException,
                   TimeValueException {
            
            if (!validateTimeFormat(timeString)) {
                throw new TimeFormatException();
            }

            int length = timeString.length();

            byte hour = Byte.parseByte(timeString.substring(0, length - 3));
            byte minute = Byte.parseByte(timeString.substring(length - 2, length));

            if ((hour < 0 || hour > 23) || (minute < 0 || minute > 59) ) {
                throw new TimeValueException();
            }

            return new Time(hour, minute);
        }

        // Only allows strings in format "6:00" or "18:30"
        private static boolean validateTimeFormat(String timeString) {
            int length = timeString.length();
            if (length != 4 && length != 5) {
                return false;
            }
            return Character.isDigit(timeString.charAt(length - 1)) &&
                   Character.isDigit(timeString.charAt(length - 2)) &&
                   timeString.charAt(length - 3) == ':' &&
                   Character.isDigit(timeString.charAt(length - 4)) &&
                   Character.isDigit(timeString.charAt(0));
        }

        public static class TimeFormatException extends RuntimeException {}

        public static class TimeValueException extends RuntimeException {}
        // #endregion with-errors
    }

    static class ThrowCatch {
        // #region throw-catch
        public static void main() {
            try {
                division(10, 0);
            } catch (IllegalArgumentException e) {
                System.out.println("An exception was caught.");
            }

            try {
                division(20, 0); // This will be executed and will fail too.
            } catch (IllegalArgumentException e) {
                System.out.println("An exception was caught.");
            }
        }

        private static int division(int dividend, int divisor)
            throws IllegalArgumentException {
            if (divisor == 0) {

                throw new IllegalArgumentException("division by zero");

            } else {
                return dividend / divisor;
            }
        }
        // #endregion throw-catch
    }

    static class Exit {
        // #region exit
        public static void main() {

            division(10, 0);





            division(20, 0); // This will not be executed.



        }

        private static int division(int dividend, int divisor) {

            if (divisor == 0) {
                System.out.println("division by zero");
                System.exit(1);
                throw new Error();
            } else {
                return dividend / divisor;
            }
        }
        // #endregion exit
    }
}
