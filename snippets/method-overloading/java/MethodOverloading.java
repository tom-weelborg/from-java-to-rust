public class MethodOverloading {
    public static void main(String[] args) {
        method(10);

        method(10, 10);
    }

    // #region methods
    // supported
    
    static void method(int a) {
        System.out.println(a);
    }

    static void method(int a, int b) {
        System.out.println(a + b);
    }
    // #endregion methods
}
