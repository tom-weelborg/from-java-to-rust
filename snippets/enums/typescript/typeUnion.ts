// #region union
type Union =
    | string
    | number
;

function printUnion(union: Union) {
    if (typeof union === 'string') {
        console.log(`The value is a string: ${union}`);
    } else {
        console.log(`The value is a number: ${union}`);
    }
}

function main() {
    // prints: "The value is a string: Test string"
    printUnion("Test string");
    // prints: "The value is a number: 42"
    printUnion(42);
}
// #endregion union

main();

// #region alias
// Alias:
type NumberArray = Array<number>;
// #endregion alias
