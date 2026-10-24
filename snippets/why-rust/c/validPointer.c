#include <stdlib.h>

int main() {
    // #region valid-pointer
    int *p = malloc(sizeof(int));
    *p = 10;
    // #endregion valid-pointer

    return 0;
}
