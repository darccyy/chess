macro_rules! piece {
    ($color:ident $kind:ident) => {
        Piece {
            kind: PieceKind::$kind,
            color: PieceColor::$color,
        }
    };
}

pub struct Board(Grid);

type Grid = [[TileState; 8]; 8];

impl Default for Board {
    fn default() -> Self {
        let mut grid: Grid = Default::default();

        for x in 0..8 {
            grid[1][x] = Some(piece!(White Pawn));
        }
        for x in 0..8 {
            grid[6][x] = Some(piece!(Black Pawn));
        }

        grid[0][1] = Some(piece!(White Rook));
        grid[0][6] = Some(piece!(White Rook));
        grid[7][1] = Some(piece!(Black Rook));
        grid[7][6] = Some(piece!(Black Rook));

        Board(grid)
    }
}

impl Board {
    pub fn tile_at_coords(&self, x: i32, y: i32) -> TileState {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        *self.0.get(y)?.get(x)?
    }
}

type TileState = Option<Piece>;

#[derive(Clone, Copy)]
pub struct Piece {
    kind: PieceKind,
    color: PieceColor,
}

#[derive(Clone, Copy)]
enum PieceKind {
    Pawn,
    Rook,
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
            // Self::Pawn => ("♙", "♟"),
            // Self::Rook => ("♖", "♜"),
        };

        if self.color == PieceColor::White {
            white
        } else {
            black
        }
    }
}
