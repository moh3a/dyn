const fib = (n) => {
  if (n <= 2) return 1;
  return fib(n - 1) + fib(n - 2);
};

console.log(fib(6));
console.log(fib(7));
console.log(fib(8));

// for small numbers it works fine
// but if we want a larger fibornacci number
console.log(fib(50)); // like the 50th number in the sequence
// it becomes very slow
