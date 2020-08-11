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

fn binary_search_vec<T: std::fmt::Display + PartialEq + PartialOrd>(
    arr: Vec<T>,
    target: T,
    mut l_index: usize,
    mut r_index: usize,
) -> i32 {
    let mut m_index: usize;
    while l_index <= r_index {
        m_index = l_index + (r_index - l_index) / 2;
        let a = arr.get(2).unwrap();
        if let Some(t) = arr.get(m_index) {
            if *t == target {
                return m_index as i32;
            } else if *t < target {
                l_index = m_index + 1;
            } else if *t > target {
                r_index = m_index - 1;
            }
        }
        println!("{}", a);
    }
    -1
}

pub fn main() {
    println!("--------- B BINARY SEARCH -----------");
    let arr = [1, 2, 34, 133, 234, 555, 676, 788, 4224, 5566, 8876];
    println!("{}", binary_search(&arr, 133, 0, arr.len()));

    let vec = vec![1, 2, 33, 131, 133, 134, 13414];
    println!("{}", binary_search_vec(vec, 133, 0, arr.len()));
}
