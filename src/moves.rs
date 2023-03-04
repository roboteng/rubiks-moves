//! Representations for algorithms taht can be performed
use std::ops::Add;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    multi::separated_list0, IResult,
};
use thiserror::Error;

use crate::cube::Cube;

/// Defines all possible single face turns
///
/// A clockwise quarter turn (like U or F) is denoted as U(1) or F(1)
/// A counter-clockwise turn (like U' or F') is denoted as U(3) or F(2)
/// A double turn (like U2 or F2) is denoted as U(2) or F(2)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FaceTurn {
    U(u8),
    D(u8),
    F(u8),
    B(u8),
    L(u8),
    R(u8),
}

/// A wrapper type that defines any possible move, including face turns, wide turn, cube rotations, and slice moves
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    FaceTurn(FaceTurn),
}

/// Represents a series of moves you can perform on a cube
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Algorithm {
    pub(crate) moves: Vec<Move>,
}

/// Occurs when a string cannot be read as a [`Algorithm`]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum MoveParseError {
    #[error("Unknown Symbol {0}")]
    UnknownSymbol(String),
}

impl FaceTurn {
    /// This creates the move that will undo a given move
    #[must_use]
    pub const fn inverse(&self) -> Self {
        const fn inv(t: u8) -> u8 {
            (t * 3) % 4
        }
        match self {
            Self::U(t) => Self::U(inv(*t)),
            Self::D(t) => Self::D(inv(*t)),
            Self::F(t) => Self::F(inv(*t)),
            Self::B(t) => Self::B(inv(*t)),
            Self::L(t) => Self::L(inv(*t)),
            Self::R(t) => Self::R(inv(*t)),
        }
    }
}

impl Move {
    /// This creates the move that will undo a given move
    #[must_use]
    pub const fn inverse(&self) -> Self {
        match self {
            Self::FaceTurn(t) => Self::FaceTurn(t.inverse()),
        }
    }
}

impl Algorithm {
    /// Creates a shorter set of moves that still leaves the cube in the same state at the end
    ///
    /// For example, combining U U into U2, or U U' into nothing
    #[must_use]
    pub fn simplify(&self) -> Self {
        let mut prev_ter = self.clone();
        let mut current_iter = self.single_pass();
        while prev_ter != current_iter {
            prev_ter = current_iter.clone();
            current_iter = current_iter.single_pass();
        }
        current_iter
    }

    fn single_pass(&self) -> Self {
        self.moves.iter().fold(Self::default(), |mut moves, &m| {
            if moves.moves.is_empty() {
                vec![m].into()
            } else {
                let last_move = moves.moves.pop().expect("list is not empty");
                let ms = last_move + m;
                let k = [moves.moves, ms.moves].concat();
                k.into()
            }
        })
    }

    /// Creates a [`Algorithm`] from a `&str`, mostly a convience function, or a way to take input from the user
    ///
    /// # Errors
    ///
    /// This errors when it is not given a space seperated list of single face turns e.g. U, F', or D2
    pub fn from(s: &str) -> Result<Self, MoveParseError> {
        let (s, m) = separated_list0(
            space1,
            alt((u_moves, d_moves, f_moves, b_moves, l_moves, r_moves)),
        )(s)?;
        if s.is_empty() {
            Ok(m.into())
        } else {
            Err(MoveParseError::UnknownSymbol(s.to_string()))
        }
    }

    /// Calulates the inverse for a whole algorthm at once.
    ///
    /// If a given [`Algorithm`] is performed on a cube, then if `Algorithm::inverse()` is perfermed, the cube will return to its origonal state
    #[must_use]
    pub fn inverse(&self) -> Self {
        Self {
            moves: self.moves.iter().rev().map(Move::inverse).collect(),
        }
    }

    /// Combines two [`Algorithm`]s in the form of ABA'B'
    #[must_use]
    pub fn commute(&self, other: &Self) -> Self {
        self.clone() + other + &self.inverse() + &other.inverse()
    }

    /// Combines two [`Algorithm`]s in the form of ABA'
    #[must_use]
    pub fn permute(&self, other: &Self) -> Self {
        self.clone() + other + &self.inverse()
    }

    /// A sample [`Algorithm`] that is used often in speedcubing
    #[must_use]
    pub fn sexy() -> Self {
        Self::from("R U R' U'").expect("this doesn't panic")
    }

    /// Determines how many times an algorithm needs to be repeated, to return to its origonal state
    #[must_use]
    pub fn order(&self) -> u32 {
        let solved_cube = Cube::new();
        let mut cube = Cube::new();
        cube = cube.apply(self.clone());
        let mut count = 1;

        while cube != solved_cube {
            cube = cube.apply(self.clone());
            count += 1;
        }
        count
    }
}

impl Add<Self> for FaceTurn {
    type Output = Algorithm;

    fn add(self, rhs: Self) -> Self::Output {
        let new_moves = match (self, rhs) {
            (Self::U(a), Self::U(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![Self::U(t)],
            },
            (Self::D(a), Self::D(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![Self::D(t)],
            },
            (Self::F(a), Self::F(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![Self::F(t)],
            },
            (Self::B(a), Self::B(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![Self::B(t)],
            },
            (Self::L(a), Self::L(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![Self::L(t)],
            },
            (Self::R(a), Self::R(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![Self::R(t)],
            },

            (Self::U(u), Self::D(d)) => vec![Self::D(d), Self::U(u)],
            (Self::L(l), Self::R(r)) => vec![Self::R(r), Self::L(l)],
            (Self::F(f), Self::B(b)) => vec![Self::B(b), Self::F(f)],

            (left, right) => vec![left, right],
        };
        new_moves.into()
    }
}

impl From<FaceTurn> for Move {
    fn from(value: FaceTurn) -> Self {
        Self::FaceTurn(value)
    }
}

impl Add<Self> for Move {
    type Output = Algorithm;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::FaceTurn(a), Self::FaceTurn(b)) => a + b,
        }
    }
}

impl Add<&Self> for Algorithm {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            moves: [self.moves, rhs.moves.clone()].concat(),
        }
    }
}

macro_rules! move_parser {
    ($fn_name: ident,   $dir: ident, $d: expr ) => {
        fn $fn_name(input: &str) -> IResult<&str, FaceTurn> {
            let (input, t) = alt((
                map(tag(format!("{}2", $d).as_str()), |_| 2),
                map(tag(format!("{}'", $d).as_str()), |_| 3),
                map(tag(format!("{}", $d).as_str()), |_| 1),
            ))(input)?;
            Ok((input, FaceTurn::$dir(t)))
        }
    };
}

move_parser!(u_moves, U, "U");
move_parser!(d_moves, D, "D");
move_parser!(f_moves, F, "F");
move_parser!(b_moves, B, "B");
move_parser!(l_moves, L, "L");
move_parser!(r_moves, R, "R");

impl From<nom::Err<nom::error::Error<&str>>> for MoveParseError {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::UnknownSymbol(value.to_string())
    }
}

impl<T> From<Vec<T>> for Algorithm
where
    T: Into<Move> + Copy,
{
    fn from(moves: Vec<T>) -> Self {
        Self {
            moves: moves.iter().map(|&m| m.into()).collect(),
        }
    }
}

#[cfg(test)]
mod simplification_tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[allow(non_snake_case)]
    fn U_then_U_rev_does_nothing() {
        let moves: Algorithm = vec![FaceTurn::U(1), FaceTurn::U(3)].into();
        let actual = moves.simplify();

        assert_eq!(actual, Algorithm::default());
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_simplifies_to_U() {
        let moves: Algorithm = vec![FaceTurn::U(1)].into();
        let actual = moves.simplify();

        assert_eq!(actual, moves);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_is_not_the_same_as_U_rev() {
        let u: Algorithm = vec![FaceTurn::U(1)].into();
        let u_rev: Algorithm = vec![FaceTurn::U(3)].into();

        assert_ne!(u, u_rev);
    }

    #[test]
    #[allow(non_snake_case)]
    fn two_Us_simplifies_to_U2() {
        let us: Algorithm = vec![FaceTurn::U(1), FaceTurn::U(1)].into();
        let actual = us.simplify();
        let expected: Algorithm = vec![FaceTurn::U(2)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_then_U2_simplifies_to_URev() {
        let turns: Algorithm = vec![FaceTurn::U(1), FaceTurn::U(2)].into();
        let actual = turns.simplify();
        let expected: Algorithm = vec![FaceTurn::U(3)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U2_then_U_simplifies_to_URev() {
        let turns: Algorithm = vec![FaceTurn::U(2), FaceTurn::U(1)].into();
        let actual = turns.simplify();
        let expected: Algorithm = vec![FaceTurn::U(3)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn four_Us_simplifies_to_nothing() {
        let turns: Algorithm = vec![
            FaceTurn::U(1),
            FaceTurn::U(1),
            FaceTurn::U(1),
            FaceTurn::U(1),
        ]
        .into();
        let actual = turns.simplify();
        let expected: Algorithm = Algorithm { moves: Vec::new() };

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_F_does_not_simplify() {
        let turns: Algorithm = vec![FaceTurn::U(1), FaceTurn::F(1)].into();
        let actual = turns.simplify();
        let expected = turns.clone();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn F_U_does_not_simplify() {
        let turns: Algorithm = vec![FaceTurn::F(1), FaceTurn::U(1)].into();
        let actual = turns.simplify();
        let expected = turns;

        assert_eq!(actual, expected);
    }

    #[test]
    fn complicated_move_simplification() {
        let turns: Algorithm = vec![
            FaceTurn::U(1),
            FaceTurn::R(3),
            FaceTurn::F(1),
            FaceTurn::F(3),
            FaceTurn::R(1),
            FaceTurn::U(2),
            FaceTurn::L(2),
            FaceTurn::D(1),
            FaceTurn::D(3),
            FaceTurn::L(2),
            FaceTurn::U(1),
        ]
        .into();
        let actual = turns.simplify();
        let expected = Algorithm::default();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U2_D2_U2_simplifies_the_two_outside_U2s() {
        let moves: Algorithm = vec![FaceTurn::U(2), FaceTurn::D(2), FaceTurn::U(2)].into();
        let actual = moves.simplify();
        let expected = vec![FaceTurn::D(2)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn D2_U2_D2_simplifies_the_two_outside_D2s() {
        let moves: Algorithm = vec![FaceTurn::D(2), FaceTurn::U(2), FaceTurn::D(2)].into();
        let actual = moves.simplify();
        let expected = vec![FaceTurn::U(2)].into();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod parsing_tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    #[allow(non_snake_case)]
    fn U() {
        let s = "U";
        let actual = Algorithm::from(s).unwrap();
        let expected = vec![FaceTurn::U(1)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn F() {
        let s = "F";
        let actual = Algorithm::from(s).unwrap();
        let expected = vec![FaceTurn::F(1)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U2() {
        let s = "U2";
        let actual = Algorithm::from(s).unwrap();
        let expected = vec![FaceTurn::U(2)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn t_perm_spaces() {
        let s = "R U R' U' R' F R2 U' R' U' R U R' F'";
        let actual = Algorithm::from(s).unwrap();
        let expected = vec![
            FaceTurn::R(1),
            FaceTurn::U(1),
            FaceTurn::R(3),
            FaceTurn::U(3),
            FaceTurn::R(3),
            FaceTurn::F(1),
            FaceTurn::R(2),
            FaceTurn::U(3),
            FaceTurn::R(3),
            FaceTurn::U(3),
            FaceTurn::R(1),
            FaceTurn::U(1),
            FaceTurn::R(3),
            FaceTurn::F(3),
        ]
        .into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn errors_on_unknown_input() {
        let s = "R U foobar R' U'";
        let actual = Algorithm::from(s).unwrap_err();
        let expected = MoveParseError::UnknownSymbol(" foobar R' U'".to_string());

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod inverse_tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    #[allow(non_snake_case)]
    fn U_moves_to_U_rev() {
        let m = Algorithm::from("U").unwrap();
        let actual = m.inverse();
        let expected = Algorithm::from("U'").unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn sexy_inverses_to_inverse_sexy() {
        let m = Algorithm::from("R U R' U'").unwrap();
        let actual = m.inverse();
        let expected = Algorithm::from("U R U' R'").unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn R_and_U_commutes_to_sexy() {
        let r = Algorithm::from("R").unwrap();
        let u = Algorithm::from("U").unwrap();
        let actual = r.commute(&u);
        let expected = Algorithm::sexy();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn F_and_sexy_permute() {
        let r = Algorithm::from("F").unwrap();
        let sexy = Algorithm::sexy();
        let actual = r.permute(&sexy);
        let expected = Algorithm::from("F R U R' U' F'").unwrap();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod order_tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    #[allow(non_snake_case)]
    fn order_of_U_is_4() {
        let m = Algorithm::from("U").unwrap();

        let actual = m.order();
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn order_of_U2_is_2() {
        let m = Algorithm::from("U2").unwrap();

        let actual = m.order();
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn order_of_sexy_is_6() {
        let m = Algorithm::sexy();

        let actual = m.order();
        let expected = 6;
        assert_eq!(actual, expected);
    }
}
