use std::fmt::Display;

#[derive(Clone)]
pub enum BTree<S> {
    Null,
    Branch {
        s: S,
        t0: Box<BTree<S>>,
        t1: Box<BTree<S>>,
    },
}

impl<S> BTree<S> {
    pub fn showoff() {
        let b_tree = BTree::Null;
        let b_tree = add_element(&b_tree, 1);
        let b_tree = add_element(&b_tree, 2);
        let b_tree = add_element(&b_tree, -1);
        let b_tree = add_element(&b_tree, 3);

        println!("b_tree:\n{}", b_tree);
        println!("search: {}", search(&b_tree, -1));
    }
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

pub fn add_element<S>(tree: &BTree<S>, x: S) -> BTree<S>
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

pub fn search<S>(tree: &BTree<S>, x: S) -> bool
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
