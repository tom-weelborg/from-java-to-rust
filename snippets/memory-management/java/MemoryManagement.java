public class MemoryManagement {
    // #region content
    public static void main(String[] args) {
        String example = new String("example");


        example = example1(example);


        example2(example);


        example1(example);

        
        System.out.println(example);
        // You can still use `example` here.


        doNothing();
    }

    static String example1(String example) {
        System.out.println(example);
        return example;
        
    }

    static void example2(String example) {
        System.out.println(example);
    }

    static void doNothing() {
        // The variable `example` is still alive.
    }
    // #endregion content
}
