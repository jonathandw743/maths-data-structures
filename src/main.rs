#![allow(dead_code)]

use std::{fmt::Display, usize};

enum FBTree<S> {
    Leaf {
        s: S,
    },
    Branch {
        s: S,
        t0: Box<FBTree<S>>,
        t1: Box<FBTree<S>>,
    },
}

impl<S> FBTree<S>
where
    S: Display,
{
    fn string_with_indent(&self, indent_amount: usize, indent_string: &str) -> String {
        match self {
            Self::Leaf { s } => {
                let mut result = String::new();
                for _ in 0..indent_amount {
                    result += indent_string;
                }
                result += &format!("{}\n", s);
                return result;
            }
            Self::Branch { s, t0, t1 } => {
                let mut result = String::new();
                for _ in 0..indent_amount {
                    result += indent_string;
                }
                result += &format!("{}\n", s);
                result += &t0.string_with_indent(indent_amount + 1, indent_string);
                result += &t1.string_with_indent(indent_amount + 1, indent_string);
                return result;
            }
        }
    }
}

impl<S> Display for FBTree<S>
where
    S: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.string_with_indent(0, "->").strip_suffix("\n").unwrap()
        )
    }
}

fn lvs<S>(tree: &FBTree<S>) -> usize {
    match tree {
        FBTree::Leaf { .. } => 1,
        FBTree::Branch { t0, t1, .. } => lvs(t0) + lvs(t1),
    }
}

fn f_0<S>(tree: &FBTree<S>) -> FBTree<S>
where
    S: PartialEq + Default + Copy,
{
    match tree {
        FBTree::Leaf { .. } => FBTree::Leaf { s: S::default() },
        FBTree::Branch { s, t0, t1 } => FBTree::Branch {
            s: *s,
            t0: Box::new(f_0(t0)),
            t1: Box::new(f_0(t1)),
        },
    }
}

#[derive(Clone)]
enum BTree<S> {
    Null,
    Branch {
        s: S,
        t0: Box<BTree<S>>,
        t1: Box<BTree<S>>,
    },
}

impl<S> BTree<S>
where
    S: Display,
{
    fn string_with_indent(&self, indent_amount: usize, indent_string: &str) -> String {
        match self {
            Self::Null => "".to_string(),
            Self::Branch { s, t0, t1 } => {
                let mut result = String::new();
                for _ in 0..indent_amount {
                    result += indent_string;
                }
                result += &format!("{}\n", s);
                result += &t0.string_with_indent(indent_amount + 1, indent_string);
                result += &t1.string_with_indent(indent_amount + 1, indent_string);
                return result;
            }
        }
    }
}

impl<S> Display for BTree<S>
where
    S: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.string_with_indent(0, "->").strip_suffix("\n").unwrap()
        )
    }
}

fn add_element<S>(tree: &BTree<S>, x: S) -> BTree<S>
where
    S: PartialOrd + Copy,
{
    match tree {
        BTree::Null => BTree::Branch {
            s: x,
            t0: Box::new(BTree::Null),
            t1: Box::new(BTree::Null),
        },
        BTree::Branch { s, t0, t1 } => {
            if x < *s {
                BTree::Branch {
                    s: *s,
                    t0: Box::new(add_element(t0, x)),
                    t1: t1.clone(),
                }
            } else {
                BTree::Branch {
                    s: *s,
                    t0: t0.clone(),
                    t1: Box::new(add_element(t1, x)),
                }
            }
        }
    }
}

fn search<S>(tree: &BTree<S>, x: S) -> bool
where
    S: PartialOrd,
{
    match tree {
        BTree::Null => false,
        BTree::Branch { s, t0, t1 } => {
            if *s == x {
                return true;
            } else if x < *s {
                return search(t0, x);
            } else {
                return search(t1, x);
            }
        }
    }
}

#[derive(Clone)]
enum List<S> {
    Empty,
    NonEmpty { s: S, suffix: Box<List<S>> },
}

impl<S> Display for List<S>
where
    S: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "[]")
            }
            Self::NonEmpty { s, suffix } => {
                write!(f, "{}:{}", s, suffix)
            }
        }
    }
}

fn concatenate<S>(l0: &List<S>, l1: &List<S>) -> List<S>
where
    S: Copy,
{
    match l0 {
        List::Empty => l1.clone(),
        List::NonEmpty { s, suffix } => List::NonEmpty {
            s: *s,
            suffix: Box::new(concatenate(suffix, l1)),
        },
    }
}

fn rev<S>(list: &List<S>) -> List<S>
where
    S: Copy,
{
    match list {
        List::Empty => List::Empty,
        List::NonEmpty { s, suffix } => concatenate(
            &rev(suffix),
            &List::NonEmpty {
                s: *s,
                suffix: Box::new(List::Empty),
            },
        ),
    }
}

// fn map<S, T>(f: impl Fn(&S) -> T) -> impl Fn(&List<S>) -> List<T> {
//     |list: &List<S>| -> List<T> {
//         match list {
//             List::Empty => List::Empty,
//             List::NonEmpty { s, suffix } => {
//                 let s = f(s);
//                 List::NonEmpty {
//                     s,
//                     suffix: Box::new(map(f)(suffix)),
//                 }
//             }
//         }
//     }
// }

fn main() {
    let list = List::NonEmpty {
        s: 1,
        suffix: Box::new(List::NonEmpty {
            s: 2,
            suffix: Box::new(List::NonEmpty {
                s: 2,
                suffix: Box::new(List::Empty),
            }),
        }),
    };
    println!("list: {}", list);
    let list2 = List::NonEmpty {
        s: 7,
        suffix: Box::new(List::NonEmpty {
            s: 8,
            suffix: Box::new(List::NonEmpty {
                s: 9,
                suffix: Box::new(List::Empty),
            }),
        }),
    };
    println!("concatenate: {}", concatenate(&list, &list2));
    println!("rev: {}", rev(&list));

    let fb_tree = FBTree::Branch {
        s: 1,
        t0: Box::new(FBTree::Branch {
            s: 2,
            t0: Box::new(FBTree::Branch {
                s: 12,
                t0: Box::new(FBTree::Leaf { s: 13 }),
                t1: Box::new(FBTree::Leaf { s: 14 }),
            }),
            t1: Box::new(FBTree::Branch {
                s: 12,
                t0: Box::new(FBTree::Leaf { s: 13 }),
                t1: Box::new(FBTree::Leaf { s: 14 }),
            }),
        }),
        t1: Box::new(FBTree::Leaf { s: 5 }),
    };
    println!("fb_tree:\n{}", fb_tree);
    println!("lvs: {}", lvs(&fb_tree));
    println!("f_0: {}", f_0(&fb_tree));

    let b_tree = BTree::Null;
    let b_tree = add_element(&b_tree, 1);
    let b_tree = add_element(&b_tree, 2);
    let b_tree = add_element(&b_tree, -1);
    let b_tree = add_element(&b_tree, 3);

    println!("b_tree:\n{}", b_tree);
    println!("search: {}", search(&b_tree, -1));
}
