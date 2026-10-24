// Taken from this video https://www.youtube.com/watch?v=MtxP2pyCvYA at 2:25
// by Fireship (https://www.youtube.com/@Fireship)
int main() {
    // #region invalid-pointer
    int *p;  // This does not point anywhere.
    *p = 10; // This tries to write to an undefined memory address.
    // #endregion invalid-pointer

    return 1; // This program is error-prone.
}
