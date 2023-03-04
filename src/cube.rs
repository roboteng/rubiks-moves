use std::fmt::Display;

use crate::moves::{Algorithm, FaceTurn, Move};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Side {
    Yellow,
    White,
    Red,
    Orange,
    Blue,
    Green,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube {
    corners: [Corner; 8],
    edges: [Edge; 12],
    centers: [Center; 6],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Corner {
    colors: [Side; 3],
}

impl Corner {
    const fn rotate(self) -> Self {
        Self {
            colors: [self.colors[2], self.colors[0], self.colors[1]],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
    colors: [Side; 2],
}

impl Edge {
    const fn flip(self) -> Self {
        Self {
            colors: [self.colors[1], self.colors[0]],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Center {
    color: Side,
}

impl Cube {
    pub const fn new() -> Self {
        Self {
            corners: [
                Corner {
                    colors: [Side::Yellow, Side::Green, Side::Red],
                },
                Corner {
                    colors: [Side::Yellow, Side::Orange, Side::Green],
                },
                Corner {
                    colors: [Side::White, Side::Green, Side::Orange],
                },
                Corner {
                    colors: [Side::White, Side::Red, Side::Green],
                },
                Corner {
                    colors: [Side::White, Side::Blue, Side::Red],
                },
                Corner {
                    colors: [Side::White, Side::Orange, Side::Blue],
                },
                Corner {
                    colors: [Side::Yellow, Side::Blue, Side::Orange],
                },
                Corner {
                    colors: [Side::Yellow, Side::Red, Side::Blue],
                },
            ],
            edges: [
                Edge {
                    colors: [Side::Yellow, Side::Green],
                },
                Edge {
                    colors: [Side::Yellow, Side::Red],
                },
                Edge {
                    colors: [Side::Yellow, Side::Blue],
                },
                Edge {
                    colors: [Side::Yellow, Side::Orange],
                },
                Edge {
                    colors: [Side::Orange, Side::Green],
                },
                Edge {
                    colors: [Side::Red, Side::Green],
                },
                Edge {
                    colors: [Side::Red, Side::Blue],
                },
                Edge {
                    colors: [Side::Orange, Side::Blue],
                },
                Edge {
                    colors: [Side::White, Side::Green],
                },
                Edge {
                    colors: [Side::White, Side::Red],
                },
                Edge {
                    colors: [Side::White, Side::Blue],
                },
                Edge {
                    colors: [Side::White, Side::Orange],
                },
            ],
            centers: [
                Center {
                    color: Side::Yellow,
                },
                Center { color: Side::White },
                Center { color: Side::Red },
                Center {
                    color: Side::Orange,
                },
                Center { color: Side::Blue },
                Center { color: Side::Green },
            ],
        }
    }

    pub fn apply(&self, moves: Algorithm) -> Self {
        let mut cube = self.clone();
        for m in moves.moves {
            cube = cube.apply_move(m);
        }
        cube
    }

    fn apply_move(&self, m: Move) -> Self {
        let mut cube = self.clone();

        match m {
            Move::FaceTurn(FaceTurn::U(1)) => {
                cube.corners[0] = self.corners[1];
                cube.corners[1] = self.corners[6];
                cube.corners[6] = self.corners[7];
                cube.corners[7] = self.corners[0];
                cube.edges[0] = self.edges[3];
                cube.edges[1] = self.edges[0];
                cube.edges[2] = self.edges[1];
                cube.edges[3] = self.edges[2];
            }
            Move::FaceTurn(FaceTurn::D(1)) => {
                cube.corners[2] = self.corners[3];
                cube.corners[3] = self.corners[4];
                cube.corners[4] = self.corners[5];
                cube.corners[5] = self.corners[2];
                cube.edges[8] = self.edges[9];
                cube.edges[9] = self.edges[10];
                cube.edges[10] = self.edges[11];
                cube.edges[11] = self.edges[8];
            }
            Move::FaceTurn(FaceTurn::R(1)) => {
                cube.corners[0] = self.corners[3].rotate().rotate();
                cube.corners[1] = self.corners[0].rotate();
                cube.corners[2] = self.corners[1].rotate().rotate();
                cube.corners[3] = self.corners[2].rotate();
                cube.edges[0] = self.edges[5];
                cube.edges[4] = self.edges[0];
                cube.edges[8] = self.edges[4];
                cube.edges[5] = self.edges[8];
            }
            Move::FaceTurn(FaceTurn::L(1)) => {
                cube.corners[4] = self.corners[7].rotate().rotate();
                cube.corners[5] = self.corners[4].rotate();
                cube.corners[6] = self.corners[5].rotate().rotate();
                cube.corners[7] = self.corners[6].rotate();
                cube.edges[2] = self.edges[7];
                cube.edges[6] = self.edges[2];
                cube.edges[10] = self.edges[6];
                cube.edges[7] = self.edges[10];
            }
            Move::FaceTurn(FaceTurn::F(1)) => {
                cube.corners[0] = self.corners[7].rotate();
                cube.corners[3] = self.corners[0].rotate().rotate();
                cube.corners[4] = self.corners[3].rotate();
                cube.corners[7] = self.corners[4].rotate().rotate();
                cube.edges[1] = self.edges[6].flip();
                cube.edges[5] = self.edges[1].flip();
                cube.edges[9] = self.edges[5].flip();
                cube.edges[6] = self.edges[9].flip();
            }
            Move::FaceTurn(FaceTurn::B(1)) => {
                cube.corners[1] = self.corners[2].rotate().rotate();
                cube.corners[6] = self.corners[1].rotate();
                cube.corners[5] = self.corners[6].rotate().rotate();
                cube.corners[2] = self.corners[5].rotate();
                cube.edges[3] = self.edges[4].flip();
                cube.edges[7] = self.edges[3].flip();
                cube.edges[11] = self.edges[7].flip();
                cube.edges[4] = self.edges[11].flip();
            }
            Move::FaceTurn(FaceTurn::U(n)) => {
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::U(1)));
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::U(n - 1)));
            }
            Move::FaceTurn(FaceTurn::D(n)) => {
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::D(1)));
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::D(n - 1)));
            }
            Move::FaceTurn(FaceTurn::F(n)) => {
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::F(1)));
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::F(n - 1)));
            }
            Move::FaceTurn(FaceTurn::B(n)) => {
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::B(1)));
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::B(n - 1)));
            }
            Move::FaceTurn(FaceTurn::L(n)) => {
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::L(1)));
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::L(n - 1)));
            }
            Move::FaceTurn(FaceTurn::R(n)) => {
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::R(1)));
                cube = cube.apply_move(Move::FaceTurn(FaceTurn::R(n - 1)));
            }
        };
        cube
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let square = match self {
            Self::Yellow => "🟨",
            Self::White => "⬜",
            Self::Red => "🟥",
            Self::Orange => "🟧",
            Self::Blue => "🟦",
            Self::Green => "🟩",
        };
        write!(f, "{square}")
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "⬛⬛⬛{}{}{}⬛⬛⬛⬛⬛⬛
⬛⬛⬛{}{}{}⬛⬛⬛⬛⬛⬛
⬛⬛⬛{}{}{}⬛⬛⬛⬛⬛⬛
{}{}{}{}{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}{}{}{}{}
⬛⬛⬛{}{}{}⬛⬛⬛⬛⬛⬛
⬛⬛⬛{}{}{}⬛⬛⬛⬛⬛⬛
⬛⬛⬛{}{}{}⬛⬛⬛⬛⬛⬛",
            self.corners[6].colors[0],
            self.edges[3].colors[0],
            self.corners[1].colors[0],
            self.edges[2].colors[0],
            self.centers[0].color,
            self.edges[0].colors[0],
            self.corners[7].colors[0],
            self.edges[1].colors[0],
            self.corners[0].colors[0],
            self.corners[6].colors[1],
            self.edges[2].colors[1],
            self.corners[7].colors[2],
            self.corners[7].colors[1],
            self.edges[1].colors[1],
            self.corners[0].colors[2],
            self.corners[0].colors[1],
            self.edges[0].colors[1],
            self.corners[1].colors[2],
            self.corners[1].colors[1],
            self.edges[3].colors[1],
            self.corners[6].colors[2],
            self.edges[7].colors[1],
            self.centers[4].color,
            self.edges[6].colors[1],
            self.edges[6].colors[0],
            self.centers[2].color,
            self.edges[5].colors[0],
            self.edges[5].colors[1],
            self.centers[5].color,
            self.edges[4].colors[1],
            self.edges[4].colors[0],
            self.centers[3].color,
            self.edges[7].colors[0],
            self.corners[5].colors[2],
            self.edges[10].colors[1],
            self.corners[4].colors[1],
            self.corners[4].colors[2],
            self.edges[9].colors[1],
            self.corners[3].colors[1],
            self.corners[3].colors[2],
            self.edges[8].colors[1],
            self.corners[2].colors[1],
            self.corners[2].colors[2],
            self.edges[11].colors[1],
            self.corners[5].colors[1],
            self.corners[4].colors[0],
            self.edges[9].colors[0],
            self.corners[3].colors[0],
            self.edges[10].colors[0],
            self.centers[1].color,
            self.edges[8].colors[0],
            self.corners[5].colors[0],
            self.edges[11].colors[0],
            self.corners[2].colors[0],
        )
    }
}

#[cfg(test)]
mod cube_tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    fn basic_cube_display() {
        let cube = Cube::new();
        let actual = format!("{cube}");
        let expected = "⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
🟦🟦🟦🟥🟥🟥🟩🟩🟩🟧🟧🟧
🟦🟦🟦🟥🟥🟥🟩🟩🟩🟧🟧🟧
🟦🟦🟦🟥🟥🟥🟩🟩🟩🟧🟧🟧
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛";

        assert_str_eq!(actual, expected);
    }

    #[test]
    fn u_turn() {
        let cube = Cube::new();
        let cube = cube.apply(Algorithm::from("U").unwrap());
        let actual = format!("{cube}");
        let expected = "⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
🟥🟥🟥🟩🟩🟩🟧🟧🟧🟦🟦🟦
🟦🟦🟦🟥🟥🟥🟩🟩🟩🟧🟧🟧
🟦🟦🟦🟥🟥🟥🟩🟩🟩🟧🟧🟧
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛";

        assert_str_eq!(actual, expected);
    }

    #[test]
    fn d_turn() {
        let cube = Cube::new();
        let cube = cube.apply(Algorithm::from("D").unwrap());
        let actual = format!("{cube}");
        let expected = "⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
🟦🟦🟦🟥🟥🟥🟩🟩🟩🟧🟧🟧
🟦🟦🟦🟥🟥🟥🟩🟩🟩🟧🟧🟧
🟧🟧🟧🟦🟦🟦🟥🟥🟥🟩🟩🟩
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛";

        assert_str_eq!(actual, expected);
    }

    #[test]
    fn r_turn() {
        let cube = Cube::new();
        let cube = cube.apply(Algorithm::from("R").unwrap());
        let actual = format!("{cube}");
        let expected = "⬛⬛⬛🟨🟨🟥⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟥⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟥⬛⬛⬛⬛⬛⬛
🟦🟦🟦🟥🟥⬜🟩🟩🟩🟨🟧🟧
🟦🟦🟦🟥🟥⬜🟩🟩🟩🟨🟧🟧
🟦🟦🟦🟥🟥⬜🟩🟩🟩🟨🟧🟧
⬛⬛⬛⬜⬜🟧⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜🟧⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜🟧⬛⬛⬛⬛⬛⬛";

        assert_str_eq!(actual, expected);
    }

    #[test]
    fn l_turn() {
        let cube = Cube::new();
        let cube = cube.apply(Algorithm::from("L").unwrap());

        let actual = format!("{cube}");
        let expected = "⬛⬛⬛🟧🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟧🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟧🟨🟨⬛⬛⬛⬛⬛⬛
🟦🟦🟦🟨🟥🟥🟩🟩🟩🟧🟧⬜
🟦🟦🟦🟨🟥🟥🟩🟩🟩🟧🟧⬜
🟦🟦🟦🟨🟥🟥🟩🟩🟩🟧🟧⬜
⬛⬛⬛🟥⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟥⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟥⬜⬜⬛⬛⬛⬛⬛⬛";

        assert_str_eq!(actual, expected);
    }

    #[test]
    fn f_turn() {
        let cube = Cube::new();
        let cube = cube.apply(Algorithm::from("F").unwrap());

        let actual = format!("{cube}");
        let expected = "⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟦🟦🟦⬛⬛⬛⬛⬛⬛
🟦🟦⬜🟥🟥🟥🟨🟩🟩🟧🟧🟧
🟦🟦⬜🟥🟥🟥🟨🟩🟩🟧🟧🟧
🟦🟦⬜🟥🟥🟥🟨🟩🟩🟧🟧🟧
⬛⬛⬛🟩🟩🟩⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛";

        assert_str_eq!(actual, expected);
    }

    #[test]
    fn b_turn() {
        let cube = Cube::new();

        let cube = cube.apply(Algorithm::from("B").unwrap());
        let actual = format!("{cube}");
        let expected = "⬛⬛⬛🟩🟩🟩⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟨🟨🟨⬛⬛⬛⬛⬛⬛
🟨🟦🟦🟥🟥🟥🟩🟩⬜🟧🟧🟧
🟨🟦🟦🟥🟥🟥🟩🟩⬜🟧🟧🟧
🟨🟦🟦🟥🟥🟥🟩🟩⬜🟧🟧🟧
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛⬜⬜⬜⬛⬛⬛⬛⬛⬛
⬛⬛⬛🟦🟦🟦⬛⬛⬛⬛⬛⬛";

        assert_str_eq!(actual, expected);
    }

    #[test]
    fn rotate_corner() {
        let corner = Corner {
            colors: [Side::Yellow, Side::Green, Side::Red],
        };
        let actual = corner.rotate();
        let expected = Corner {
            colors: [Side::Red, Side::Yellow, Side::Green],
        };

        assert_eq!(actual, expected);
    }
}
