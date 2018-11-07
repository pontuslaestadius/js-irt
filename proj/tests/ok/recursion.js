/// ```
/// assert_eq!(rec(9, 0), 19);
/// ```
function rec(a, b) {
    if (a > 10) {
        return b;
    }
    return rec(a+1, b+a);
}

