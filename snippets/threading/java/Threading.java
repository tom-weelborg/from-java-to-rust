import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class Threading {
    public static class NotThreadSafe {
        // #region not-thread-safe
        static int counter = 0;

        static final int THREADS = 4;
        static final int ITERATIONS = 100_000;

        public static void main(String[] args) throws InterruptedException {
            ExecutorService pool = Executors.newFixedThreadPool(THREADS);



            for (int t = 0; t < THREADS; t++) {
                pool.submit(() -> {
                    for (int i = 0; i < ITERATIONS; i++) {


                        counter++;



                    }
                });
            }

            pool.shutdown();
            pool.awaitTermination(1, TimeUnit.MINUTES);


            System.out.println("Expected: " + THREADS * ITERATIONS);
            System.out.println("Actual: " + counter);
        }
        // #endregion not-thread-safe
    }

    public static class ThreadSafe {
        // #region thread-safe
        static int counter = 0;

        static final int THREADS = 4;
        static final int ITERATIONS = 100_000;

        public static void main(String[] args) throws InterruptedException {
            ExecutorService pool = Executors.newFixedThreadPool(THREADS);

            Lock mutex = new ReentrantLock();

            for (int t = 0; t < THREADS; t++) {
                pool.submit(() -> {
                    for (int i = 0; i < ITERATIONS; i++) {
                        mutex.lock();
                        try {
                            counter++;
                        } finally {
                            mutex.unlock();
                        }
                    }
                });
            }

            pool.shutdown();
            pool.awaitTermination(1, TimeUnit.MINUTES);

            
            System.out.println("Expected: " + THREADS * ITERATIONS);
            System.out.println("Actual: " + counter);
        }
        // #endregion thread-safe
    }

    public static void main(String[] args) throws InterruptedException {
        System.out.println("First run is not thread-safe");
        NotThreadSafe.main(args);

        System.out.println("Second run is thread-safe");
        ThreadSafe.main(args);
    }
}
