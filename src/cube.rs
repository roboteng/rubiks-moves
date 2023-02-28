use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
enum Side {
    Yellow,
    White,
    Red,
    Orange,
    Blue,
    Green,
}

#[derive(Debug, PartialEq, Eq)]
struct Cube {
    corners: [Corner; 8],
    edges: [Edge; 12],
    centers: [Center; 6],
}

#[derive(Debug, PartialEq, Eq)]
struct Corner {
    colors: [Side; 3],
}

#[derive(Debug, PartialEq, Eq)]
struct Edge {
    colors: [Side; 2],
}

#[derive(Debug, PartialEq, Eq)]
struct Center {
    color: Side,
}

impl Cube {
    fn new() -> Self {
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
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let square = match self {
            Side::Yellow => "🟨",
            Side::White => "⬜",
            Side::Red => "🟥",
            Side::Orange => "🟧",
            Side::Blue => "🟦",
            Side::Green => "🟩",
        };
        write!(f, "{}", square)
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
            self.edges[1].colors[0],
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
    fn sexy_has_order_6() {
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
}
