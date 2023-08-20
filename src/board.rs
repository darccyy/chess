use crate::Coords;

macro_rules! piece {
    ($color:ident $kind:ident) => {
        Piece {
            kind: PieceKind::$kind,
            color: PieceColor::$color,
        }
    };
}

pub struct Board(Grid);

type Grid = [[Option<Piece>; 8]; 8];

impl Default for Board {
    fn default() -> Self {
        let mut grid: Grid = Default::default();

        for x in 0..8 {
            grid[1][x] = Some(piece!(White Pawn));
        }
        for x in 0..8 {
            grid[6][x] = Some(piece!(Black Pawn));
        }

        grid[0][0] = Some(piece!(White Rook));
        grid[0][1] = Some(piece!(White Knight));
        grid[0][2] = Some(piece!(White Bishop));
        grid[0][3] = Some(piece!(White Queen));
        grid[0][4] = Some(piece!(White King));
        grid[0][5] = Some(piece!(White Bishop));
        grid[0][6] = Some(piece!(White Knight));
        grid[0][7] = Some(piece!(White Rook));

        grid[7][0] = Some(piece!(Black Rook));
        grid[7][1] = Some(piece!(Black Knight));
        grid[7][2] = Some(piece!(Black Bishop));
        grid[7][3] = Some(piece!(Black Queen));
        grid[7][4] = Some(piece!(Black King));
        grid[7][5] = Some(piece!(Black Bishop));
        grid[7][6] = Some(piece!(Black Knight));
        grid[7][7] = Some(piece!(Black Rook));

        Board(grid)
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
}

#[derive(Clone, Copy)]
pub struct Piece {
    kind: PieceKind,
    color: PieceColor,
}

#[derive(Clone, Copy)]
enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq)]
enum PieceColor {
    White,
    Black,
}

impl Piece {
    pub fn symbol(&self) -> &'static str {
        let (white, black) = match self.kind {
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

        if self.color == PieceColor::White {
            white
        } else {
            black
        }
    }
}
