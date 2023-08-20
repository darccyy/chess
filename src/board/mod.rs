macro_rules! piece {
    ($color:ident $kind:ident) => {
        Piece {
            kind: PieceKind::$kind,
            color: PieceColor::$color,
        }
    };
}

#[cfg(test)]
mod tests;

use crate::Coords;

#[derive(Debug, PartialEq)]
pub struct Board(Grid);

type Grid = [[Option<Piece>; 8]; 8];

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

impl Default for Board {
    fn default() -> Self {
        Self::from_fen(START_FEN)
        // let mut grid: Grid = Default::default();
        //
        // for x in 0..8 {
        //     grid[1][x] = Some(piece!(White Pawn));
        // }
        // for x in 0..8 {
        //     grid[6][x] = Some(piece!(Black Pawn));
        // }
        //
        // grid[0][0] = Some(piece!(White Rook));
        // grid[0][1] = Some(piece!(White Knight));
        // grid[0][2] = Some(piece!(White Bishop));
        // grid[0][3] = Some(piece!(White Queen));
        // grid[0][4] = Some(piece!(White King));
        // grid[0][5] = Some(piece!(White Bishop));
        // grid[0][6] = Some(piece!(White Knight));
        // grid[0][7] = Some(piece!(White Rook));
        //
        // grid[7][0] = Some(piece!(Black Rook));
        // grid[7][1] = Some(piece!(Black Knight));
        // grid[7][2] = Some(piece!(Black Bishop));
        // grid[7][3] = Some(piece!(Black Queen));
        // grid[7][4] = Some(piece!(Black King));
        // grid[7][5] = Some(piece!(Black Bishop));
        // grid[7][6] = Some(piece!(Black Knight));
        // grid[7][7] = Some(piece!(Black Rook));
        //
        // Self(grid)
    }
}

impl Board {
    pub fn tile_at_coords(&self, coords: Coords) -> Option<Option<Piece>> {
        let x: usize = coords.x.try_into().ok()?;
        let y: usize = coords.y.try_into().ok()?;
        Some(*self.0.get(y)?.get(x)?)
    }
    pub fn tile_at_coords_mut(&mut self, coords: Coords) -> Option<&mut Option<Piece>> {
        let x: usize = coords.x.try_into().ok()?;
        let y: usize = coords.y.try_into().ok()?;
        Some(self.0.get_mut(y)?.get_mut(x)?)
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut grid: Grid = Default::default();

        let mut x: usize = 0;
        let mut y: usize = 0;

        for ch in fen.chars() {
            let piece = match ch {
                '1'..='8' => {
                    x += ch.to_digit(10).unwrap() as usize;
                    continue;
                }
                '/' => {
                    x = 0;
                    y += 1;
                    continue;
                }

                'p' => piece!(Black Pawn),
                'P' => piece!(White Pawn),
                'r' => piece!(Black Rook),
                'R' => piece!(White Rook),
                'n' => piece!(Black Knight),
                'N' => piece!(White Knight),
                'b' => piece!(Black Bishop),
                'B' => piece!(White Bishop),
                'q' => piece!(Black Queen),
                'Q' => piece!(White Queen),
                'k' => piece!(Black King),
                'K' => piece!(White King),

                _ => panic!("Invalid character: '{}'", ch),
            };

            if x >= 8 {
                panic!("File greater than 8");
            }
            if y >= 8 {
                panic!("Rank greater than 8");
            }

            grid[y][x] = Some(piece);
            x += 1;
        }

        Self(grid)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    kind: PieceKind,
    color: PieceColor,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceColor {
    White,
    Black,
}

impl Piece {
    pub fn symbol(&self) -> &'static str {
        let (black, white) = match self.kind {
            PieceKind::Pawn => ("p", "P"),
            PieceKind::Rook => ("r", "R"),
            PieceKind::Knight => ("n", "N"),
            PieceKind::Bishop => ("b", "B"),
            PieceKind::Queen => ("q", "Q"),
            PieceKind::King => ("k", "K"),
            // PieceKind::Pawn => ("♙", "♟"),
            // PieceKind::Rook => ("♖", "♜"),
            // PieceKind::Knight => ("♘", "♞"),
            // PieceKind::Bishop => ("♗", "♝"),
            // PieceKind::Queen => ("♕", "♛"),
            // PieceKind::King => ("♔", "♚"),
        };

        if self.is_black() {
            black
        } else {
            white
        }
    }

    pub fn is_white(&self) -> bool {
        self.color == PieceColor::White
    }
    pub fn is_black(&self) -> bool {
        self.color == PieceColor::Black
    }
}
