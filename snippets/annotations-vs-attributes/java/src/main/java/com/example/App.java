package com.example;

import lombok.EqualsAndHashCode;
import lombok.ToString;

public class App {
    public static void main(String[] args) throws Exception {
        oldMethod();
        newMethod();

        method();

        WithoutLombok.main(args);
        WithLombok.main(args);

        Stringify.stringify();

        Clone.main(args);
    }

    // #region deprecated
    /**
     * Prints "Hello, World!".
     * 
     * @deprecated
     * This was the old way of doing that.
     * Use {@link #newMethod()} instead.
     */
    @Deprecated(since = "0.0.0")
    public static void oldMethod() {
        System.out.println("Hello, World!");
    }

    /**
     * Prints "Hello, World!".
     */
    public static void newMethod() {
        System.out.println("Hello, World!");
    }
    // #endregion deprecated

    // #region definition

    public @interface DataHolder {
        String value() default "";
    }
    // #endregion definition

    // #region usage
    @DataHolder("Example for `method`")
    public static void method() {
        System.out.println("This method has an annotation.");
    }
    // #endregion usage

    public static class WithoutLombok {
        // #region equals-nolombok
        public static class Example {
            int a;
            String b;




            @Override
            public boolean equals(Object obj) {
                if (!(obj instanceof Example e)) {
                    return false;
                }
                return a == e.a && b.equals(e.b);
            }
        }
        // #endregion equals-nolombok

        public static void main(String[] args) {
            Example e1 = new Example();
            e1.a = 1;
            e1.b = "Hello";

            Example e2 = new Example();
            e2.a = 1;
            e2.b = "World";


            System.out.println(e1.equals(e2));

            e2.b = "Hello";
            System.out.println(e1.equals(e2));
        }
    }

    public static class WithLombok {
        // #region equals-lombok

        @EqualsAndHashCode
        public static class Example {
            int a;
            String b;
        }
        // #endregion equals-lombok

        public static void main(String[] args) {
            Example e1 = new Example();
            e1.a = 1;
            e1.b = "Hello";

            Example e2 = new Example();
            e2.a = 1;
            e2.b = "World";


            System.out.println(e1.equals(e2));

            e2.b = "Hello";
            System.out.println(e1.equals(e2));
        }
    }

    public static class Stringify {
        // #region stringify

        @ToString
        public static class Example {
            int a;
            String b;
        }









        public static void stringify() {
            Example e = new Example();
            e.a = 42;
            e.b = "Test";


            System.out.println(e); // prints: App.Stringify.Example(a=42, b=Test)


        }
        // #endregion stringify
    }

    public static class Copy {
        // #region copy
        // no equivalent in Java
        // #endregion copy
    }

    public static class Clone {
        // #region clone
        
        public static class Example {
            int a;
            String b;

            @Override
            protected Example clone() throws CloneNotSupportedException {
                Example e = new Example();
                e.a = a;
                e.b = b;
                return e;
            }
        }

        public static void main(String[] args) throws CloneNotSupportedException {
            Example e1 = new Example();
            e1.a = 42;
            e1.b = "Hello";

            Example e2 = e1.clone();

            System.out.println(e1.a);
            System.out.println(e2.a);
        }
        // #endregion clone
    }
}
