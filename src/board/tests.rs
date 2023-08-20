use super::*;

#[test]
fn start_fen_works() {
    let board = Board::from_fen(START_FEN);

    assert_eq!(
        board,
        Board([
            [
                Some(piece!(Black Rook)),
                Some(piece!(Black Knight)),
                Some(piece!(Black Bishop)),
                Some(piece!(Black Queen)),
                Some(piece!(Black King)),
                Some(piece!(Black Bishop)),
                Some(piece!(Black Knight)),
                Some(piece!(Black Rook)),
            ],
            [
                Some(piece!(Black Pawn)),
                Some(piece!(Black Pawn)),
                Some(piece!(Black Pawn)),
                Some(piece!(Black Pawn)),
                Some(piece!(Black Pawn)),
                Some(piece!(Black Pawn)),
                Some(piece!(Black Pawn)),
                Some(piece!(Black Pawn)),
            ],
            [None, None, None, None, None, None, None, None,],
            [None, None, None, None, None, None, None, None,],
            [None, None, None, None, None, None, None, None,],
            [None, None, None, None, None, None, None, None,],
            [
                Some(piece!(White Pawn)),
                Some(piece!(White Pawn)),
                Some(piece!(White Pawn)),
                Some(piece!(White Pawn)),
                Some(piece!(White Pawn)),
                Some(piece!(White Pawn)),
                Some(piece!(White Pawn)),
                Some(piece!(White Pawn)),
            ],
            [
                Some(piece!(White Rook)),
                Some(piece!(White Knight)),
                Some(piece!(White Bishop)),
                Some(piece!(White Queen)),
                Some(piece!(White King)),
                Some(piece!(White Bishop)),
                Some(piece!(White Knight)),
                Some(piece!(White Rook)),
            ],
        ])
    );
}
