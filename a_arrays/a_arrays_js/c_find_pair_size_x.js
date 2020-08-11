/**
 * @param {number[]} arr
 * @param {number} sum
 * @returns {[number, number]} [indexOfFirstNumber, indexOfSecondNumber]
 */
function findPairs(arr, sum) {
  let left = 0,
    right = arr.length - 1;

  arr = arr.sort((a, z) => a - z);
  while (left < right) {
    const leftRight = arr[left] + arr[right];
    if (leftRight === sum) return [left, right];
    else if (leftRight < sum) left++;
    else right--;
  }

  return null;
}

export default () => {
  console.log(
    "--------- C - FIND THE PAIR OF NUMBERS WHOSE SIZE IS X --------"
  );

  const sum = 66;
  const arr = [1, 22, 33, 13, 9, 44, 12, 11];
  console.log(findPairs(arr, sum));
};
