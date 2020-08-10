use std::cmp::PartialEq;
use std::fmt::Display;

fn linear_search_2<T: PartialEq>(v: &Vec<T>, target: T) -> i32 {
    for (i, ele) in v.iter().enumerate() {
        if *ele == target {
            return i as i32;
        }
    }
    -1
}

#[derive(Debug, Clone)]
pub enum LinkedList<T: Display> {
    Cons(T, Box<LinkedList<T>>),
    Nil,
}

impl<T: Display + PartialEq> LinkedList<T> {
    fn new() -> LinkedList<T> {
        Nil
    }

    fn prepend(self, elem: T) -> LinkedList<T> {
        Cons(elem, Box::new(self))
    }

    // fn append<'a>(&'a mut self, elem: &'a mut LinkedList<&mut T>) -> &'a mut LinkedList<&mut T> {
    //    self = match self {
    //        Cons(_, list) => *list.append(elem),
    //         Nil => elem
    //    }
    // }

    fn search(&self, i: T) -> &LinkedList<T>
    where
        T: PartialEq + Copy + Clone,
    {
        match self {
            Cons(val, list) => {
                if *val == i {
                    return self;
                }
                list.search(i)
            }
            Nil => {
                let a = &Nil;
                a
            }
        }
    }
    fn len(self) -> usize {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => format!("{{ {}, {} }}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}
use LinkedList::{Cons, Nil};

pub fn main() {
    println!("------- A-LINEAR_SEARCH -----------");

    let mut v = vec![1, 2, 42, 2, 11, 10];
    println!("{}", linear_search_2(&v, 11));

    let mut list: LinkedList<i32> = LinkedList::new();
    v.reverse();
    for i in v {
        list = list.prepend(i);
    }

    const TO_SEARCH: i32 = 42;
    println!(
        "Searching for: {} and got: {}!",
        TO_SEARCH,
        list.search(TO_SEARCH).stringify()
    );
    println!("length: {}", list.len());
}
