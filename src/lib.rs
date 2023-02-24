// https://jperm.net/3x3/moves

use std::{collections::BTreeMap, ops::Add};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FaceTurn {
    U,
    URev,
    U2,
    F,
}

impl Add<FaceTurn> for FaceTurn {
    type Output = MoveList;

    fn add(self, rhs: FaceTurn) -> Self::Output {
        let res = match (self, rhs) {
            (FaceTurn::U, FaceTurn::U) => vec![FaceTurn::U2],
            (FaceTurn::U, FaceTurn::URev) => Vec::new(),
            (FaceTurn::U, FaceTurn::U2) => vec![FaceTurn::URev],
            (FaceTurn::URev, FaceTurn::U) => Vec::new(),
            (FaceTurn::URev, FaceTurn::URev) => vec![FaceTurn::U2],
            (FaceTurn::URev, FaceTurn::U2) => vec![FaceTurn::U],
            (FaceTurn::U2, FaceTurn::U) => vec![FaceTurn::URev],
            (FaceTurn::U2, FaceTurn::URev) => vec![FaceTurn::U],
            (FaceTurn::U2, FaceTurn::U2) => Vec::new(),
            (left, right) => vec![left, right],
        };
        res.into()
    }
}

impl FaceTurn {
    pub fn inverse(&self) -> Self {
        Self::URev
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MoveList {
    moves: Vec<FaceTurn>,
}

impl MoveList {
    pub fn simplify(&self) -> Self {
        if self.moves.get(1) == Some(&FaceTurn::F) {
            return self.clone();
        }

        let moves: BTreeMap<u8, _> =
            BTreeMap::from([(1, FaceTurn::U), (2, FaceTurn::U2), (3, FaceTurn::URev)]);

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
        let moves: MoveList = vec![FaceTurn::U, FaceTurn::URev].into();
        let actual = moves.simplify();

        assert_eq!(actual, MoveList::default());
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_simplifies_to_U() {
        let moves: MoveList = vec![FaceTurn::U].into();
        let actual = moves.simplify();

        assert_eq!(actual, moves);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_is_not_the_same_as_U_rev() {
        let u: MoveList = vec![FaceTurn::U].into();
        let u_rev: MoveList = vec![FaceTurn::URev].into();

        assert_ne!(u, u_rev);
    }

    #[test]
    #[allow(non_snake_case)]
    fn two_Us_simplifies_to_U2() {
        let us: MoveList = vec![FaceTurn::U, FaceTurn::U].into();
        let actual = us.simplify();
        let expected: MoveList = vec![FaceTurn::U2].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_then_U2_simplifies_to_URev() {
        let turns: MoveList = vec![FaceTurn::U, FaceTurn::U2].into();
        let actual = turns.simplify();
        let expected: MoveList = vec![FaceTurn::URev].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U2_then_U_simplifies_to_URev() {
        let turns: MoveList = vec![FaceTurn::U2, FaceTurn::U].into();
        let actual = turns.simplify();
        let expected: MoveList = vec![FaceTurn::URev].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn four_Us_simplifies_to_nothing() {
        let turns: MoveList = vec![FaceTurn::U, FaceTurn::U, FaceTurn::U, FaceTurn::U].into();
        let actual = turns.simplify();
        let expected: MoveList = vec![].into();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn U_F_does_not_simplify() {
        let turns: MoveList = vec![FaceTurn::U, FaceTurn::F].into();
        let actual = turns.simplify();
        let expected = turns.clone();

        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn F_U_does_not_simplify() {
        let turns: MoveList = vec![FaceTurn::F, FaceTurn::U].into();
        let actual = turns.simplify();
        let expected = turns.clone();

        assert_eq!(actual, expected);
    }
}
