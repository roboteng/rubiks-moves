// https://jperm.net/3x3/moves

use std::ops::Add;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::map,
    multi::{separated_list0, separated_list1},
    IResult,
};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FaceTurn {
    U(u8),
    D(u8),
    F(u8),
    B(u8),
    L(u8),
    R(u8),
}

impl Add<FaceTurn> for FaceTurn {
    type Output = MoveList;

    fn add(self, rhs: FaceTurn) -> Self::Output {
        let res = match (self, rhs) {
            (FaceTurn::U(a), FaceTurn::U(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![FaceTurn::U(t)],
            },
            (FaceTurn::D(a), FaceTurn::D(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![FaceTurn::D(t)],
            },
            (FaceTurn::F(a), FaceTurn::F(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![FaceTurn::F(t)],
            },
            (FaceTurn::B(a), FaceTurn::B(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![FaceTurn::B(t)],
            },
            (FaceTurn::L(a), FaceTurn::L(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![FaceTurn::L(t)],
            },
            (FaceTurn::R(a), FaceTurn::R(b)) => match (a + b) % 4 {
                0 => Vec::new(),
                t => vec![FaceTurn::R(t)],
            },

            (FaceTurn::U(u), FaceTurn::D(d)) => vec![FaceTurn::D(d), FaceTurn::U(u)],
            (FaceTurn::L(l), FaceTurn::R(r)) => vec![FaceTurn::R(r), FaceTurn::L(l)],
            (FaceTurn::F(f), FaceTurn::B(b)) => vec![FaceTurn::B(b), FaceTurn::F(f)],

            (left, right) => vec![left, right],
        };
        res.into()
    }
}

impl FaceTurn {
    pub fn inverse(&self) -> Self {
        fn inv(t: &u8) -> u8 {
            (t * 3) % 4
        }
        match self {
            FaceTurn::U(t) => FaceTurn::U(inv(t)),
            FaceTurn::D(t) => FaceTurn::D(inv(t)),
            FaceTurn::F(t) => FaceTurn::F(inv(t)),
            FaceTurn::B(t) => FaceTurn::B(inv(t)),
            FaceTurn::L(t) => FaceTurn::L(inv(t)),
            FaceTurn::R(t) => FaceTurn::R(inv(t)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MoveList {
    moves: Vec<FaceTurn>,
}

impl MoveList {
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
        self.moves
            .iter()
            .fold(MoveList::default(), |mut moves, &m| {
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

    pub fn from(s: &str) -> Result<Self, MoveParseError> {
        let (s, m) = separated_list0(
            space1,
            alt((u_moves, d_moves, f_moves, b_moves, l_moves, r_moves)),
        )(s)?;
        Ok(m.into())
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

#[derive(Debug, Error)]
pub enum MoveParseError {
    #[error("Unknown Symbol {0}")]
    UnknownSymbol(String),
}

impl From<nom::Err<nom::error::Error<&str>>> for MoveParseError {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        MoveParseError::UnknownSymbol(value.to_string())
    }
}

impl From<Vec<FaceTurn>> for MoveList {
    fn from(moves: Vec<FaceTurn>) -> Self {
        Self { moves }
    }
}

#[cfg(test)]
mod simplification_tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[allow(non_snake_case)]
    fn U_then_U_rev_does_nothing() {
        let moves: MoveList = vec![FaceTurn::U(1), FaceTurn::U(3)].into();
        let actual = moves.simplify();

        assert_eq!(actual, MoveList::default());
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_simplifies_to_U() {
        let moves: MoveList = vec![FaceTurn::U(1)].into();
        let actual = moves.simplify();

        assert_eq!(actual, moves);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_is_not_the_same_as_U_rev() {
        let u: MoveList = vec![FaceTurn::U(1)].into();
        let u_rev: MoveList = vec![FaceTurn::U(3)].into();

        assert_ne!(u, u_rev);
    }

    #[test]
    #[allow(non_snake_case)]
    fn two_Us_simplifies_to_U2() {
        let us: MoveList = vec![FaceTurn::U(1), FaceTurn::U(1)].into();
        let actual = us.simplify();
        let expected: MoveList = vec![FaceTurn::U(2)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_then_U2_simplifies_to_URev() {
        let turns: MoveList = vec![FaceTurn::U(1), FaceTurn::U(2)].into();
        let actual = turns.simplify();
        let expected: MoveList = vec![FaceTurn::U(3)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U2_then_U_simplifies_to_URev() {
        let turns: MoveList = vec![FaceTurn::U(2), FaceTurn::U(1)].into();
        let actual = turns.simplify();
        let expected: MoveList = vec![FaceTurn::U(3)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn four_Us_simplifies_to_nothing() {
        let turns: MoveList = vec![
            FaceTurn::U(1),
            FaceTurn::U(1),
            FaceTurn::U(1),
            FaceTurn::U(1),
        ]
        .into();
        let actual = turns.simplify();
        let expected: MoveList = vec![].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_F_does_not_simplify() {
        let turns: MoveList = vec![FaceTurn::U(1), FaceTurn::F(1)].into();
        let actual = turns.simplify();
        let expected = turns.clone();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn F_U_does_not_simplify() {
        let turns: MoveList = vec![FaceTurn::F(1), FaceTurn::U(1)].into();
        let actual = turns.simplify();
        let expected = turns.clone();

        assert_eq!(actual, expected);
    }

    #[test]
    fn complicated_move_simplification() {
        let turns: MoveList = vec![
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
        let expected = MoveList::default();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U2_D2_U2_simplifies_the_two_outside_U2s() {
        let moves: MoveList = vec![FaceTurn::U(2), FaceTurn::D(2), FaceTurn::U(2)].into();
        let actual = moves.simplify();
        let expected = vec![FaceTurn::D(2)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn D2_U2_D2_simplifies_the_two_outside_D2s() {
        let moves: MoveList = vec![FaceTurn::D(2), FaceTurn::U(2), FaceTurn::D(2)].into();
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
        let actual = MoveList::from(s).unwrap();
        let expected = vec![FaceTurn::U(1)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn F() {
        let s = "F";
        let actual = MoveList::from(s).unwrap();
        let expected = vec![FaceTurn::F(1)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U2() {
        let s = "U2";
        let actual = MoveList::from(s).unwrap();
        let expected = vec![FaceTurn::U(2)].into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn t_perm_spaces() {
        let s = "R U R' U' R' F R2 U' R' U' R U R' F'";
        let actual = MoveList::from(s).unwrap();
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
}
