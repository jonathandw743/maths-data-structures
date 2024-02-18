use std::{
    fmt::Display,
    ops::{Add, Mul},
};

#[derive(Clone)]
pub enum NaturalNumber {
    Zero,
    Successor { n: Box<NaturalNumber> },
}

macro_rules! natural_number {
    (0) => {
        NaturalNumber::Zero
    };
    ((S $n: tt)) => {
        NaturalNumber::Successor {
            n: Box::new(natural_number!($n)),
        }
    };
}

impl NaturalNumber {
    pub fn showoff() {
        let m = natural_number!((S (S (S 0))));
        let n = natural_number!((S (S 0)));
        println!("m: {}", m);
        println!("n: {}", n);
        println!("m + n: {}", m.clone() + n.clone());
        println!("m * n: {}", m * n);
    }
}

impl Display for NaturalNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => {
                write!(f, "0")
            }
            Self::Successor { n } => {
                write!(f, "S{}", n)
            }
        }
    }
}

impl Add for NaturalNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match rhs {
            Self::Zero => self,
            Self::Successor { n } => Self::Successor {
                n: Box::new(self + *n),
            },
        }
    }
}

impl Mul for NaturalNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match rhs {
            Self::Zero => Self::Zero,
            Self::Successor { n } => self.clone() * *n + self.clone(),
        }
    }
}
