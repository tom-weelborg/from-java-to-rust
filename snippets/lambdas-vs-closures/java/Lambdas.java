import java.util.function.Function;

public class Lambdas {
    public static void main(String[] args) {
        greet();

        SimpleCounter.count();
        ExpandedCounter.count();
    }

    private static void greet() {
        // #region greet
        final String hello = "Hello, ";

        final Function<String, String> greeter = name -> hello + name + "!";






        System.out.println(greeter.apply("World"));    // prints: Hello, World!
        System.out.println(greeter.apply("Universe")); // prints: Hello, Universe!
        // #endregion greet
    }

    private static class SimpleCounter {
        // #region simple-counter
        private static class Counter {
            private int counter = 0;
        }

        private static void count() {
            final Counter counter = new Counter();

            final Runnable incrementCounter = () -> counter.counter++;

            incrementCounter.run();
            incrementCounter.run();

            System.out.println(counter.counter); // prints: 2
        }
        // #endregion simple-counter
    }

    private static class ExpandedCounter {
        // #region expanded-counter
        private static class Counter {
            private int counter = 0;
        }

        private static void count() {
            final Counter counter = new Counter();

            final Runnable incrementCounter = () -> counter.counter++;
            final Runnable decrementCounter = () -> counter.counter--;

            incrementCounter.run();
            incrementCounter.run();
            decrementCounter.run();
            decrementCounter.run();

            System.out.println(counter.counter); // prints: 0
        }
        // #endregion expanded-counter
    }
}
