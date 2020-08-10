use std::marker::Sized;
fn binary_search<T: Sized + PartialEq + PartialOrd>(
    arr: &[T],
    target: T,
    mut l_index: usize,
    mut r_index: usize,
) -> i32 {
    let mut m_index: usize;
    while l_index <= r_index {
        m_index = l_index + (r_index - l_index) / 2;
        if arr[m_index] == target {
            return m_index as i32;
        } else if arr[m_index] < target {
            l_index = m_index + 1;
        } else if arr[m_index] > target {
            r_index = m_index - 1;
        }
    }
    -1
}

pub fn main() {
    println!("--------- B BINARY SEARCH -----------");
    let arr = [1, 2, 34, 133, 234, 555, 676, 788, 4224, 5566, 8876];
    println!("{}", binary_search(&arr, 133, 0, arr.len()))
}
