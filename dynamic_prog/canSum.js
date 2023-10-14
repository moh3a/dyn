// =======================================================
//
// CAN SUM
//
// =======================================================

/**
 *
 * Write a function that takes in a targetSum and an
 * array of numbers as arguments.
 *
 * The function should return a boolean indicating wether
 * or not it is possible to generate the targetSum using
 * numbers from the array.
 *
 */
const canSum = (targetSum, numbers) => {
  if (targetSum === 0) return true;
  if (targetSum < 0) return false;

  for (let num of numbers) {
    const remainder = targetSum - num;
    if (canSum(remainder, numbers)) return true;
  }

  return false;
};

console.time("Can sum [5, 3, 4, 7] to target 10");
canSum(10, [5, 3, 4, 7]);
console.timeEnd("Can sum [5, 3, 4, 7] to target 10");
// console.log(canSum(300, [7, 14])) too slow!

// ======================================================
//
// MEMOIZED CAN SUM
//
// ======================================================

const memo_canSum = (targetSum, numbers, memo = {}) => {
  if (targetSum in memo) return memo[targetSum];
  if (targetSum === 0) return true;
  if (targetSum < 0) return false;

  for (let num of numbers) {
    const remainder = targetSum - num;
    if (memo_canSum(remainder, numbers, memo)) {
      memo[targetSum] = true;
      return true;
    }
  }

  memo[targetSum] = false;
  return false;
};

console.time("Memoized can sum [5, 3, 4, 7] to target 10");
memo_canSum(10, [5, 3, 4, 7]);
console.timeEnd("Memoized can sum [5, 3, 4, 7] to target 10");
console.log(
  "Memoized can sum [7, 14] to target 300:",
  memo_canSum(300, [7, 14]),
);
