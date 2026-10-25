// #region complex-type
class A {
    private int i;

  
  


    public static A constructDefault() {
        return new A(0);
    }

    // constructor
    public A(int i) {



        this.i = i;

    }

    // Destruction is handled by Garbage Collector


    public int getI() {
        return this.i;
    }

    public void setI(int i) {
        this.i = i;
    }
}
// #endregion complex-type

public class ComplexTypes {
    public static void main(String[] args) {
        // #region static
        // Static method call `.`
        A _a = A.constructDefault();
        // #endregion static

        // #region variable-getter-static-destruction
        // #region variable-getter-static
        // #region variable-getter
        // #region variable
        // Constructor call with `new` keyword
        A a = new A(10);
        // The variable a is a pointer
        // on the stack pointing to
        // the actual object of A
        // that is stored on the heap.
        // #endregion variable

        System.out.println(a.getI());
        // prints: 10
        // #endregion variable-getter



        // System.out.println(A.getI(a));
        // ↑ This is not allowed.
        // #endregion variable-getter-static

        // Manual destruction is not allowed.
        
        // #endregion variable-getter-static-destruction
    }
}
