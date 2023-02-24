#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FaceTurn {
    U,
    URev,
    U2,
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
        match self
            .moves
            .iter()
            .map(|m| match m {
                FaceTurn::U => 1,
                FaceTurn::URev => 3,
                FaceTurn::U2 => 2,
            })
            .sum::<u8>()
            % 4
        {
            0 => Self::default(),
            1 => vec![FaceTurn::U].into(),
            2 => vec![FaceTurn::U2].into(),
            3 => vec![FaceTurn::URev].into(),
            _ => panic!("unknown number of turns"),
        }
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
}
