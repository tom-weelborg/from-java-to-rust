public class StringExample {
    public static void main(String[] args) {
        a();

        b();

        concatenation();
    }
    
    private static void a() {
        // #region a
        String literal = "Hello, world!";
        // stored on the heap (object)
        // or in the JVM's string pool (literal)

        String string = new String(literal);
        // stored on the heap
        


        System.out.println(literal);
        // prints: "Hello, world!"

        System.out.println(string);
        // prints: "Hello, world!"
        // #endregion a
    }
    
    private static void b() {
        // #region b
        String string = "Hello, world!";


        System.out.println(string);
        // prints: "Hello, world!"
        // #endregion b
    }
    
    private static void concatenation() {
        // #region concatenation
        // Concatenation
        String string = "Hello, ";
        string += "world";
        string = string + "!";
        System.out.println(string);
        // prints: "Hello, world!"
        // #endregion concatenation
    }
}
