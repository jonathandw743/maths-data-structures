use std::fmt::Display;

pub enum Regex<S> {
    Symbol(S),
    Concat {
        lhs: Box<Regex<S>>,
        rhs: Box<Regex<S>>,
    },
    KleeneStar(Box<Regex<S>>),
    Alternative(Box<Regex<S>>, Box<Regex<S>>),
}

pub fn showoff() {
    let alternative = Regex::Alternative(Box::new(Regex::Symbol(0)), Box::new(Regex::Symbol(1)));
    let concat = Regex::Concat {
        lhs: Box::new(Regex::Concat {
            lhs: Box::new(Regex::Symbol(0)),
            rhs: Box::new(Regex::Symbol(1)),
        }),
        rhs: Box::new(Regex::Symbol(2)),
    };
    let kleene_star = Regex::KleeneStar(Box::new(Regex::Symbol(2)));
    let nested = Regex::KleeneStar(Box::new(Regex::Alternative(
        Box::new(Regex::Symbol(0)),
        Box::new(Regex::KleeneStar(Box::new(Regex::Symbol(1)))),
    )));
    let regex = Regex::Concat {
        lhs: Box::new(alternative),
        rhs: Box::new(Regex::Concat {
            lhs: Box::new(concat),
            rhs: Box::new(Regex::Concat {
                lhs: Box::new(kleene_star),
                rhs: Box::new(nested),
            }),
        }),
    };
    println!("regex: {}", regex);
}

impl<S> Display for Regex<S>
where
    S: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(s) => write!(f, "{}", s),
            Self::Concat { lhs, rhs } => write!(f, "{}{}", lhs, rhs),
            Self::KleeneStar(s) => write!(f, "({})*", s),
            Self::Alternative(s0, s1) => write!(f, "({}|{})", s0, s1),
        }
    }
}
