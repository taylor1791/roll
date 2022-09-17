use super::precedence;

#[derive(Debug)]
pub enum Operator {
    Binary(Binary),
    Unary(Unary),
}

impl Operator {
    pub fn precedence(&self) -> u64 {
        match self {
            Operator::Binary(Binary { precedence, .. }) => *precedence,
            Operator::Unary(Unary { precedence, .. }) => *precedence,
        }
    }
}

#[derive(Debug)]
pub struct Binary {
    pub assoc: precedence::Assoc,
    pub precedence: u64,
    pub symbol: &'static str,
    pub space: bool,
}

#[derive(Debug)]
pub struct Unary {
    pub precedence: u64,
    pub symbol: &'static str,
}

pub const DICE: Binary = Binary {
    assoc: precedence::Assoc::Left,
    precedence: 2,
    symbol: "d",
    space: false,
};

pub const D: Unary = Unary {
    precedence: 2,
    symbol: "d",
};

pub const MINUS: Unary = Unary {
    precedence: 3,
    symbol: "-",
};

pub const PLUS: Unary = Unary {
    precedence: 3,
    symbol: "+",
};

pub const EXPONENT: Binary = Binary {
    assoc: precedence::Assoc::Right,
    precedence: 4,
    symbol: "**",
    space: true,
};

pub const IDIVISION: Binary = Binary {
    assoc: precedence::Assoc::Left,
    precedence: 5,
    symbol: "/",
    space: true,
};

pub const PRODUCT: Binary = Binary {
    assoc: precedence::Assoc::Left,
    precedence: 5,
    symbol: "*",
    space: true,
};

pub const DIFFERENCE: Binary = Binary {
    assoc: precedence::Assoc::Left,
    precedence: 6,
    symbol: "-",
    space: true,
};

pub const SUM: Binary = Binary {
    assoc: precedence::Assoc::Left,
    precedence: 6,
    symbol: "+",
    space: true,
};
