use macroquad::prelude::*;

const TILE_SIZE: f32 = 64.0;
const BOARD_SIZE: usize = 8;
const PIECE_SIZE: f32 = 64.0;

#[macroquad::main("Chess")]
async fn main() {
    let mut board = Board::new().await;

    loop {
        clear_background(WHITE);
        draw_board(&board);

        next_frame().await;
    }
}

struct Board {
    squares: [[Option<ChessSquare>; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
    pieces_texture: Texture2D,
}

impl Board {
    async fn new() -> Self {
        Board {
            squares: [
                // Black pieces
                [
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Rook),
                        color: Some(Player::Black),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Knight),
                        color: Some(Player::Black),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Bishop),
                        color: Some(Player::Black),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Queen),
                        color: Some(Player::Black),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::King),
                        color: Some(Player::Black),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Bishop),
                        color: Some(Player::Black),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Knight),
                        color: Some(Player::Black),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Rook),
                        color: Some(Player::Black),
                    }),
                ],
                // Black pawns
                [Some(ChessSquare {
                    piece: Some(ChessPiece::Pawn),
                    color: Some(Player::Black),
                }); BOARD_SIZE],
                // Empty rows
                [None; BOARD_SIZE],
                [None; BOARD_SIZE],
                [None; BOARD_SIZE],
                [None; BOARD_SIZE],
                // White pawns
                [Some(ChessSquare {
                    piece: Some(ChessPiece::Pawn),
                    color: Some(Player::White),
                }); BOARD_SIZE],
                // White pieces
                [
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Rook),
                        color: Some(Player::White),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Knight),
                        color: Some(Player::White),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Bishop),
                        color: Some(Player::White),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Queen),
                        color: Some(Player::White),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::King),
                        color: Some(Player::White),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Bishop),
                        color: Some(Player::White),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Knight),
                        color: Some(Player::White),
                    }),
                    Some(ChessSquare {
                        piece: Some(ChessPiece::Rook),
                        color: Some(Player::White),
                    }),
                ],
            ],
            current_player: Player::White,
            pieces_texture: load_texture("assets/pieces.png").await.unwrap(),
        }
    }

    fn get_piece_rect(piece: ChessPiece, color: Player) -> Rect {
        let row_offset = if color == Player::White {
            0.0
        } else {
            PIECE_SIZE
        };
        let col_offset = match piece {
            ChessPiece::King => 0.0,
            ChessPiece::Queen => PIECE_SIZE,
            ChessPiece::Rook => PIECE_SIZE * 2.0,
            ChessPiece::Bishop => PIECE_SIZE * 3.0,
            ChessPiece::Knight => PIECE_SIZE * 4.0,
            ChessPiece::Pawn => PIECE_SIZE * 5.0,
        };

        Rect::new(col_offset, row_offset, PIECE_SIZE, PIECE_SIZE)
    }
}

fn draw_board(board: &Board) {
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let color = if (x + y) % 2 == 0 {
                DARKGRAY
            } else {
                LIGHTGRAY
            };

            draw_rectangle(
                x as f32 * TILE_SIZE,
                y as f32 * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                color,
            );

            if let Some(square) = board.squares[x][y] {
                if let Some(piece) = square.piece {
                    let piece_rect = Board::get_piece_rect(piece, square.color.unwrap());
                    draw_texture_ex(
                        &board.pieces_texture,
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        WHITE,
                        DrawTextureParams {
                            source: Some(piece_rect),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum ChessPiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Player {
    White,
    Black,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct ChessSquare {
    piece: Option<ChessPiece>,
    color: Option<Player>,
}
