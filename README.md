# js-irt
JavaScript - Inline Rust Testing


# What is this?
Small and simple program which allows for Rust-like inline tests in .js files.


# Examples
Create a simple.js file.
```javascript
/// ```
/// let a = 50;
/// assert_eq!(add(1, 1), 2);
/// assert_eq!(add(a, 1), 51);
/// assert_eq!(add(a, 50), 100);
/// ```
function add(a, b) {
    return a + b;
}
```

Run the program with the input file.

```bash
cargo run simple.js
```

The result:

```
test simple.js - assert_eq!(add(1, 1), 2); ... ok
test simple.js - assert_eq!(add(99, 1), 100); ... ok
test simple.js - assert_eq!(add(50, 50), 99); ... FAILED
```


# Why?
because.
