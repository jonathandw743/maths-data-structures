use std::fmt::Display;

#[derive(Clone)]
pub enum ListRef<'a, S> {
    Empty,
    NonEmpty { s: S, suffix: &'a ListRef<'a, S> },
}

macro_rules! listref {
    (()) => {
        ListRef::Empty
    };
    (($s: expr, $suffix: tt)) => {
        ListRef::NonEmpty {
            s: $s,
            suffix: &listref!($suffix),
        }
    };
}

impl<'a, S> ListRef<'a, S> {
    pub fn showoff() {
        let x = listref!((1, (2, (3, ()))));
        println!("{}", x);
    }
}

impl<'a, S> Display for ListRef<'a, S>
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

pub fn concatenate<'a, S>(l0: &ListRef<'a, S>, l1: &ListRef<'a, S>) -> ListRef<'a, S>
where
    S: Copy,
{
    match l0 {
        ListRef::Empty => l1.clone(),
        ListRef::NonEmpty { s, suffix } => ListRef::NonEmpty {
            s: *s,
            suffix: &concatenate(suffix, l1),
        },
    }
}

pub fn rev<'a, S>(list: &ListRef<'a, S>) -> ListRef<'a, S>
where
    S: Copy,
{
    match list {
        ListRef::Empty => ListRef::Empty,
        ListRef::NonEmpty { s, suffix } => concatenate(
            &rev(suffix),
            &ListRef::NonEmpty {
                s: *s,
                suffix: &ListRef::Empty,
            },
        ),
    }
}

// pub fn map<'a, S, T>(f: impl Fn(&S) -> T + Clone) -> impl Fn(&ListRef<'a, S>) -> ListRef<'a, T> {
//     move |list: &ListRef<'a, S>| -> ListRef<'a, T> {
//         match list {
//             ListRef::Empty => ListRef::Empty,
//             ListRef::NonEmpty { s, suffix } => {
//                 let s = f(s);
//                 let suffix_f = map(f.clone());
//                 let suffix = suffix_f(suffix);
//                 ListRef::NonEmpty { s, suffix: &suffix }
//             }
//         }
//     }
// }
