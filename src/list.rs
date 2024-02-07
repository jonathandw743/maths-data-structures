use std::fmt::Display;

#[derive(Clone)]
pub enum List<S> {
    Empty,
    NonEmpty { s: S, suffix: Box<List<S>> },
}

impl<S> List<S> {
    pub fn showoff() {
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
        println!("map: {}", map(|x| x * 2)(&list));
    }
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

pub fn concatenate<S>(l0: &List<S>, l1: &List<S>) -> List<S>
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

pub fn rev<S>(list: &List<S>) -> List<S>
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

pub fn map<S, T>(f: impl Fn(&S) -> T + Clone) -> impl Fn(&List<S>) -> List<T> {
    move |list: &List<S>| -> List<T> {
        match list {
            List::Empty => List::Empty,
            List::NonEmpty { s, suffix } => {
                let s = f(s);
                List::NonEmpty {
                    s,
                    suffix: Box::new(map(f.clone())(suffix)),
                }
            }
        }
    }
}
