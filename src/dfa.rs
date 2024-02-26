use std::fmt::{Debug, Display};

// Q is a finite, non-empty set of states,
// q is the start state
// here, S is the set of labels
pub struct DFA<S, Q> {
    q: Q,
    f: Vec<Q>,
    // this returns Option<Q> not a Q to simplify dump states
    d: Box<dyn Fn(Q, S) -> Option<Q>>,
}

pub fn showoff() {
    enum S {
        A,
        B,
        C,
    }
    #[derive(PartialEq)]
    enum Q {
        Zero,
        One,
        Two,
    }
    let dfa = DFA::<S, Q> {
        q: Q::Zero,
        f: vec![Q::Two],
        d: Box::new(|q, s| match (q, s) {
            (Q::Zero, S::A) => Some(Q::One),
            (Q::Zero, S::B) => None,
            (Q::Zero, S::C) => None,
            (Q::One, S::A) => Some(Q::Two),
            (Q::One, S::B) => Some(Q::Zero),
            (Q::One, S::C) => None,
            (Q::Two, S::A) => None,
            (Q::Two, S::B) => Some(Q::Zero),
            (Q::Two, S::C) => Some(Q::One),
        }),
    };
    // let s_str = "aacacbaacbaacbaacabaacbaaca";
    let s_str = "aaca";
    let s = s_str
        .chars()
        .map(|c| match c {
            'a' => S::A,
            'b' => S::B,
            'c' => S::C,
            _ => panic!("invalid letter"),
        })
        .collect();
    println!("dfa matches {}: {}", s_str, dfa.matches(s));
}

impl<S, Q> DFA<S, Q>
where
    Q: PartialEq,
{
    pub fn matches(self, s: Vec<S>) -> bool {
        if s.len() == 0 {
            return self.f.contains(&self.q);
        }
        let mut s_iter = s.into_iter();
        let mut q = match (self.d)(self.q, s_iter.next().unwrap()) {
            Some(q) => q,
            None => {
                return false;
            }
        };
        let _ = s_iter.next();
        for c in s_iter {
            match (self.d)(q, c) {
                Some(next_q) => {
                    q = next_q;
                }
                None => {
                    return false;
                }
            }
        }
        self.f.contains(&q)
    }
}
