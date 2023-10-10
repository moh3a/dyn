// ==========================================================
//
// FIBORNACCI SEQUENCE
//
// ==========================================================

const fib = (n) => {
  if (n <= 2) return 1;
  return fib(n - 1) + fib(n - 2);
};

console.log(fib(6));
console.log(fib(7));
console.log(fib(8));

// for small numbers it works fine
// but if we want a larger fibornacci number
// console.log(fib(50)); // like the 50th number in the sequence
// it becomes very slow

// ===========================================================
//
// FIB. WITH MEMOIZATION
//
// ===========================================================

const memo_fib = (n, memo = {}) => {
  if (n in memo) return memo[n];
  if (n <= 2) return 1;
  memo[n] = memo_fib(n - 1, memo) + memo_fib(n - 2, memo);
  return memo[n];
};

console.log(memo_fib(6));
console.log(memo_fib(50));
