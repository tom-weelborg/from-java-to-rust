public class Enums {
    // #region compile-time


    public enum WeekDay {
        MONDAY,
        TUESDAY,
        WEDNESDAY,
        THURSDAY,
        FRIDAY,
        SATURDAY(true),
        SUNDAY(true);

        private final boolean isWeekend;

        WeekDay() {
            this(false);
        }

        WeekDay(boolean isWeekend) {
            this.isWeekend = isWeekend;
        }

        public boolean isWeekendDay() {
            return this.isWeekend;
        }
    }
    // #endregion compile-time

    /*
    // #region runtime
    // This is a simplfication.
    // In reality, names are resolved by the compiler and not via anonymous subclasses.
    public class WeekDay extends Enum<WeekDay> {
        public static final WeekDay MONDAY = new WeekDay() {
            public String name() {
                return "MONDAY";
            }

            // ...
        };
        public static final WeekDay TUESDAY = new WeekDay();      // ...
        public static final WeekDay WEDNESDAY = new WeekDay();    // ...
        public static final WeekDay THURSDAY = new WeekDay();     // ...
        public static final WeekDay FRIDAY = new WeekDay();       // ...
        public static final WeekDay SATURDAY = new WeekDay(true); // ...
        public static final WeekDay SUNDAY = new WeekDay(true) {
            public String name() {
                return "SUNDAY";
            }

            // ...
        };

        private final boolean isWeekend;

        private WeekDay() {
            this(false);
        }

        private WeekDay(boolean isWeekend) {
            this.isWeekend = isWeekend;
        }

        public boolean isWeekendDay() {
            return this.isWeekend;
        }
    }
    // #endregion runtime
    */
   
    // #region result-types
    public enum ResultType {
        SUCCESS,
        ERROR
    }

    public static class Result<T> {
        public static <T> Result<T> success(T value) {
            return new Result<>(ResultType.SUCCESS, value, null);
        }

        public static <T> Result<T> error(String errorMessage) {
            return new Result<>(ResultType.ERROR, null, errorMessage);
        }

        private final ResultType result;
        private final T value;
        private final String errorMessage;

        private Result(ResultType result, T value, String errorMessage) {
            this.result = result;
            this.value = value;
            this.errorMessage = errorMessage;
        }

        public ResultType getResult() {
            return this.result;
        }

        public T getValue() {
            return this.value;
        }

        public String getErrorMessage() {
            return this.errorMessage;
        }
    }
    // #endregion result-types

    public static void main(String[] args) {
        // #region result-usage
        Result<Integer> success
            = Result.success(42);
        Result<Integer> error
            = Result.error("This has failed.");

        // valid calls, but always null
        String errorMessage
            = success.getErrorMessage();
        System.out.println(errorMessage);

        Integer value
            = error.getValue();
        System.out.println(value);
        // #endregion result-usage
    }
}
