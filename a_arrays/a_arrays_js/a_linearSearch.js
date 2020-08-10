/** * @param {number[]} arr Array to search
 * @param {number} target target
 */
const linearSearch = (arr, target) => {
  for (let i = 0; i < arr.length; i++) if (target === arr[i]) return i;
  return -1;
};

/**
 * @typedef {{ data: number, next: Head }} Head
 * @param {?Head} head
 * @param {number} target
 */
const linearSearchLinkedList = (head, target) => {
  if (head != null) {
    while (head != null) {
      if (head.data === target) return head;
      head = head.next;
    }
  }
  return head;
};

function main() {
  console.log("---Implementing Linear Search-----");

  console.log(linearSearch([1, 2, 3, 4], 3));

  const LinkedList = {
    data: 1,
    next: {
      data: 12,
      next: {
        data: 123,
        next: {
          data: 33,
          next: {
            data: 101,
            next: null,
          },
        },
      },
    },
  };

  console.log(linearSearchLinkedList(LinkedList, 33));
}

export default main;
