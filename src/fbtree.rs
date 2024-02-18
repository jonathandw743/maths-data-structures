use std::fmt::Display;

pub enum FBTree<S> {
    Leaf {
        s: S,
    },
    Branch {
        s: S,
        t0: Box<FBTree<S>>,
        t1: Box<FBTree<S>>,
    },
}

macro_rules! fb_tree {
    ([$s: expr, ($t0: tt, $t1: tt)]) => {
        FBTree::Branch {
            s: $s,
            t0: Box::new(fb_tree!($t0)),
            t1: Box::new(fb_tree!($t1)),
        }
    };
    ($s: expr) => {
        FBTree::Leaf { s: $s }
    };
}

impl<S> FBTree<S> {
    pub fn showoff() {
        let fb_tree = fb_tree!([1, ([2, ([12, (13, 14)], [12, (13, 14)])], 5)]);
        println!("fb_tree:\n{}", fb_tree);
        println!("lvs: {}", lvs(&fb_tree));
        println!("f_0:\n{}", f_0(&fb_tree));
    }
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

pub fn lvs<S>(tree: &FBTree<S>) -> usize {
    match tree {
        FBTree::Leaf { .. } => 1,
        FBTree::Branch { t0, t1, .. } => lvs(t0) + lvs(t1),
    }
}

pub fn f_0<S>(tree: &FBTree<S>) -> FBTree<S>
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
