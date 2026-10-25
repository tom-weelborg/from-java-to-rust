// #region content

public class AccessModifiers {

    // Visible everywhere and to everyone
    public void publicMethod() {
        System.out.println("This is public.");
    }

    // Visible to this class, its package and to every subclass 
    protected void protectedMethod() {
        System.out.println("This is protected.");
    }

    // Visible to this class and its package (default)
    void packagePrivateMethod() {
        System.out.println("This is package-private.");
    }

    // Visible to this class
    private void privateMethod() {
        System.out.println("This is private.");
    }
}
// #endregion content
