// https://jperm.net/3x3/moves

use std::ops::Add;

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
}

impl From<Vec<FaceTurn>> for MoveList {
    fn from(moves: Vec<FaceTurn>) -> Self {
        Self { moves }
    }
}

#[cfg(test)]
mod test {
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
}
