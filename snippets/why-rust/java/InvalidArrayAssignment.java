public class InvalidArrayAssignment {
    public static void main(String[] args) {
        // #region invalid-array-assignment
        int[] arr = null;
        arr[10] = 10;
        // ↑ throws NullPointerException at runtime
        //   The failure happens at runtime,
        //   even though it is obvious at compile time that it will not work.
        // #endregion invalid-array-assignment
    }
}
