/**
 * @template T
 * @param {Object} o
 * @param {T[]} o.arr
 * @param {number} o.l_index
 * @param {number} o.r_index
 * @param {T} o.target
 * @return {number} the index of the found element
 */

function binarySearch({ arr, l_index = 0, r_index, target }) {
  if (r_index === undefined) r_index = arr.length - 1;
  let m_index;
  while (l_index <= r_index) {
    m_index = Math.floor(l_index + (r_index - l_index) / 2);
    if (arr[m_index] === target) return m_index;
    else if (arr[m_index] < target) l_index = m_index - 1;
    else if (arr[m_index] > target) r_index = m_index + 1;
  }
  return -1;
}

export default () => {
  console.log("----------- B. BINARY_SEARCH ----------");
  const arr = [1, 2, 4, 11, 13, 44, 133, 1354, 4424, 6333];

  console.log(binarySearch({ arr, target: 11 }));
};
