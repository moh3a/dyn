// ==========================================================
//
// GRID TRAVELER
//
// ==========================================================

/**
 *
 * Say that you are a traveler on a 2D grid.
 * You begin in the top-left corner and your goal
 * is to travel to the bottom-right corner.
 * You may only move down or right.
 *
 * In how many ways can you travel to the goal
 * on a grid with dimensions m * s ?
 *
 * */

const gridTraveler = (m, n) => {
  if (m === 1 && n === 1) return 1;
  if (m === 0 || n === 0) return 0;
  return gridTraveler(m - 1, n) + gridTraveler(m, n - 1);
};

console.time("Grid traveler (6,6)");
console.log(gridTraveler(6, 6));
console.timeEnd("Grid traveler (6,6)");
// console.log(gridTraveler(18, 18)); <- very slow! memoization is required

// =========================================================
//
// GRID TRAVELER WITH MEMOIZATION
//
// =========================================================

const memo_gridTraveler = (m, n, memo = {}) => {
  const key = `${m},${n}`;
  if (key in memo) return memo[key];
  if (m === 1 && n === 1) return 1;
  if (m === 0 || n === 0) return 0;
  memo[key] =
    memo_gridTraveler(m - 1, n, memo) + memo_gridTraveler(m, n - 1, memo);
  return memo[key];
};

console.time("Memoized grid traveler (6,6)");
console.log(memo_gridTraveler(6, 6));
console.timeEnd("Memoized grid traveler (6,6)");
console.log("Memoized grid traveler (18, 18):" + memo_gridTraveler(18, 18));
