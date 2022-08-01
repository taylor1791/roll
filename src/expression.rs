use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Expression {
    n: usize,
    sides: u64,
    modifier: i64,
}

#[derive(Debug, PartialEq)]
pub struct Roll {
    pub value: u64,
    pub sides: u64,
}

impl Expression {
    pub fn exec(&self, seed: u64) -> (Vec<Roll>, i64) {
        let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(seed);
        let mut rolls = Vec::with_capacity(self.n);

        let mut sum = 0;
        for _ in 0..self.n {
            let roll = rand::RngCore::next_u64(&mut rng) % self.sides + 1;

            rolls.push(Roll {
                value: roll,
                sides: self.sides,
            });
            sum += roll;
        }

        (
            rolls,
            TryInto::<i64>::try_into(sum).unwrap() + self.modifier,
        )
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        if self.n != 0 {
            let mut string = format!("{}d{}", self.n, self.sides);

            match self.modifier.cmp(&0) {
                std::cmp::Ordering::Less => {
                    string.push_str(" - ");
                    string.push_str(&self.modifier.abs().to_string());
                }
                std::cmp::Ordering::Greater => {
                    string.push_str(" + ");
                    string.push_str(&self.modifier.to_string());
                }
                std::cmp::Ordering::Equal => {}
            }

            string
        } else {
            self.modifier.to_string()
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("expression invalid")]
    InvalidExpression,

    #[error("expected number after token 'd'")]
    MissingDiceSides,

    #[error("unexpected end of expression")]
    MissingModifier,

    #[error("unexpected token '{0}' at position {1}")]
    UnexpectedToken(char, usize),
}

impl std::str::FromStr for Expression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = Tokens::None;

        for (i, c) in s.to_lowercase().chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                tokens.take_digit(digit.into());
            } else if c == 'd' {
                tokens.take_d(i)?;
            } else if c == '+' {
                tokens.take_plus(i)?;
            } else if c == '-' {
                tokens.take_minus(i)?;
            } else if c.is_whitespace() {
                // Do nothing
            } else {
                return Err(Error::UnexpectedToken(c, i));
            }
        }

        tokens.try_into()
    }
}

enum Tokens {
    MinusModifier(i64, i64, i64),
    PlusModifier(i64, i64, i64),
    Minus(i64, i64),
    Plus(i64, i64),
    Dice(i64, i64),
    NDice(i64),
    Constant(i64),
    None,
}

impl Tokens {
    fn take_digit(&mut self, d: i64) {
        match self {
            Tokens::MinusModifier(_n, _sides, modifier) => *modifier = *modifier * 10 + d,
            Tokens::PlusModifier(_n, _sides, modifier) => *modifier = *modifier * 10 + d,
            Tokens::Minus(n, sides) => *self = Tokens::MinusModifier(*n, *sides, d),
            Tokens::Plus(n, sides) => *self = Tokens::PlusModifier(*n, *sides, d),
            Tokens::Dice(_n, sides) => *sides = *sides * 10 + d,
            Tokens::NDice(n) => *self = Tokens::Dice(*n, d),
            Tokens::Constant(value) => *value = *value * 10 + d,
            Tokens::None => *self = Tokens::Constant(d),
        }
    }

    fn take_d(&mut self, position: usize) -> Result<(), Error> {
        match self {
            Tokens::MinusModifier(..)
            | Tokens::PlusModifier(..)
            | Tokens::Minus(..)
            | Tokens::Plus(..)
            | Tokens::Dice(..)
            | Tokens::NDice(..) => return Err(Error::UnexpectedToken('d', position)),
            Tokens::Constant(value) => *self = Tokens::NDice(*value),
            Tokens::None => *self = Tokens::NDice(1),
        }

        Ok(())
    }

    fn take_plus(&mut self, position: usize) -> Result<(), Error> {
        match self {
            Tokens::Dice(n, sides) => *self = Tokens::Plus(*n, *sides),
            Tokens::None => *self = Tokens::Constant(0),
            Tokens::MinusModifier(..)
            | Tokens::PlusModifier(..)
            | Tokens::Minus(..)
            | Tokens::Plus(..)
            | Tokens::NDice(..)
            | Tokens::Constant(..) => return Err(Error::UnexpectedToken('+', position)),
        }

        Ok(())
    }

    fn take_minus(&mut self, position: usize) -> Result<(), Error> {
        match self {
            Tokens::Dice(n, sides) => *self = Tokens::Minus(*n, *sides),
            Tokens::None => *self = Tokens::Minus(0, 0),
            Tokens::MinusModifier(..)
            | Tokens::PlusModifier(..)
            | Tokens::Minus(..)
            | Tokens::Plus(..)
            | Tokens::NDice(..)
            | Tokens::Constant(..) => return Err(Error::UnexpectedToken('-', position)),
        }

        Ok(())
    }
}

impl TryFrom<Tokens> for Expression {
    type Error = Error;

    fn try_from(tokens: Tokens) -> Result<Self, Self::Error> {
        let (n, sides, modifier) = match tokens {
            Tokens::MinusModifier(n, sides, modifier) => (n, sides, -modifier),
            Tokens::PlusModifier(n, sides, modifier) => (n, sides, modifier),
            Tokens::Minus(..) | Tokens::Plus(..) => return Err(Error::MissingModifier),
            Tokens::Dice(n, sides) => (n, sides, 0),
            Tokens::NDice(..) => return Err(Error::MissingDiceSides),
            Tokens::Constant(modifier) => (0, 0, modifier),
            Tokens::None => return Err(Error::InvalidExpression),
        };

        let n = TryInto::<usize>::try_into(n).map_err(|_err| Error::InvalidExpression)?;
        let sides = TryInto::<u64>::try_into(sides).map_err(|_err| Error::InvalidExpression)?;
        Ok(Expression { n, sides, modifier })
    }
}
