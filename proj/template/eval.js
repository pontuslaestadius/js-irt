assert_eq = (a, b) => {return a == b};
assert_ne = (a, b) => {return !assert_eq(a, b)};
assert = (a) => {return a == true};
