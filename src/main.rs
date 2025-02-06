use macroquad::prelude::*;

const TILE_SIZE: f32 = 100.0; // Size of tiles on the screen
const BOARD_SIZE: usize = 8; // Size of the chess board (8x8)
const PIECE_SIZE: f32 = 64.0; // Size of piece sprites on the spritesheet

#[macroquad::main("Chess")]
async fn main() {
    let mut board = Board::new();
    board.load_texture("assets/pieces.png").await.unwrap();
    let mut selected_piece = None;
    let mut orig_piece_x = 0;
    let mut orig_piece_y = 0;

    board.draw();
    loop {
        clear_background(WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_tile_x = (mouse_position().0 / TILE_SIZE) as usize;
            let mouse_tile_y = (mouse_position().1 / TILE_SIZE) as usize;

            if selected_piece.is_none() {
                selected_piece = board.squares[mouse_tile_x][mouse_tile_y];
                orig_piece_x = mouse_tile_x;
                orig_piece_y = mouse_tile_y;
            } else {
                if board.is_valid_move(orig_piece_x, orig_piece_y, mouse_tile_x, mouse_tile_y) {
                    board.squares[mouse_tile_x][mouse_tile_y] = selected_piece;
                    board.squares[orig_piece_x][orig_piece_y] = None;
                    board.switch_player();
                }

                selected_piece = None;
            }
        }

        board.draw();

        next_frame().await;
    }
}

struct Board {
    squares: [[Option<ChessPiece>; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
    pieces_texture: Texture2D,
}

impl Board {
    fn new() -> Self {
        let mut squares = [[None; BOARD_SIZE]; BOARD_SIZE];

        for (x, row) in squares.iter_mut().enumerate() {
            row[6] = Some(ChessPiece {
                piece: PieceType::Pawn,
                color: Player::White,
            });

            row[1] = Some(ChessPiece {
                piece: PieceType::Pawn,
                color: Player::Black,
            });

            let piece = match x {
                0 | 7 => PieceType::Rook,
                1 | 6 => PieceType::Knight,
                2 | 5 => PieceType::Bishop,
                3 => PieceType::Queen,
                4 => PieceType::King,
                _ => unreachable!(),
            };

            row[7] = Some(ChessPiece {
                piece,
                color: Player::White,
            });
            row[0] = Some(ChessPiece {
                piece,
                color: Player::Black,
            });
        }

        Board {
            squares,
            current_player: Player::White,
            pieces_texture: Texture2D::empty(),
        }
    }

    async fn load_texture(&mut self, path: &str) -> Result<(), &'static str> {
        match load_texture(path).await {
            Ok(texture) => {
                self.pieces_texture = texture;
                Ok(())
            }
            Err(_) => Err("failed to load texture"),
        }
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };
    }

    fn is_valid_move(&self, start_x: usize, start_y: usize, dest_x: usize, dest_y: usize) -> bool {
        if start_x == dest_x && start_y == dest_y {
            return false; // Can't not move
        }

        if let Some(start) = &self.squares[start_x][start_y] {
            if start.color != self.current_player {
                return false; // Can't move if it's not your turn
            }

            let capture = if let Some(dest) = &self.squares[dest_x][dest_y] {
                if start.color == dest.color {
                    return false; // Can't capture own piece
                }

                true
            } else {
                false
            };

            let abs_dx = (dest_x as isize - start_x as isize).abs();

            let dy = dest_y as isize - start_y as isize;
            let abs_dy = dy.abs();
            let dy_sig = dy.signum();

            let is_clear = self.is_path_clear(start_x, start_y, dest_x, dest_y);

            let can_piece_move = match start.piece {
                PieceType::Pawn => {
                    let dir = match start.color {
                        Player::White => -1,
                        Player::Black => 1,
                    };

                    let start_row = match start.color {
                        Player::White => 6,
                        Player::Black => 1,
                    };

                    dy_sig == dir // Can only move in right direction
                        && ((abs_dy <= if start_y == start_row { 2 } else { 1 } && !capture && is_clear) // Can
                    // move forward if theres no piece in the way
                            || (capture && abs_dx == 1 && abs_dy == 1)) // Can capture diagonally
                }
                PieceType::Knight => (abs_dx == 2 && abs_dy == 1) || (abs_dx == 1 && abs_dy == 2),
                PieceType::Bishop => is_clear && abs_dx == abs_dy,
                PieceType::Rook => is_clear && abs_dx == 0 || abs_dy == 0,
                PieceType::Queen => is_clear && (abs_dx == abs_dy || abs_dx == 0 || abs_dy == 0),
                PieceType::King => abs_dx <= 1 && abs_dy <= 1,
            };

            return can_piece_move;
        }

        false
    }

    fn is_path_clear(&self, start_x: usize, start_y: usize, dest_x: usize, dest_y: usize) -> bool {
        let dx = (dest_x as isize - start_x as isize).signum();
        let dy = (dest_y as isize - start_y as isize).signum();

        let mut x = start_x as isize + dx;
        let mut y = start_y as isize + dy;

        while (x != dest_x as isize || y != dest_y as isize)
            && x < BOARD_SIZE as isize
            && y < BOARD_SIZE as isize
        {
            if self.squares[x as usize][y as usize].is_some() {
                return false;
            }
            x += dx;
            y += dy;
        }

        true
    }

    fn draw(&self) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let color = if (x + y + 1) % 2 == 0 {
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

                if let Some(square) = self.squares[x][y] {
                    let piece_rect = get_piece_rect(square.piece, square.color);
                    draw_texture_ex(
                        &self.pieces_texture,
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        WHITE,
                        DrawTextureParams {
                            source: Some(piece_rect),
                            dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}

fn get_piece_rect(piece: PieceType, color: Player) -> Rect {
    let row_offset = if color == Player::White {
        0.0
    } else {
        PIECE_SIZE
    };
    let col_offset = match piece {
        PieceType::King => 0.0,
        PieceType::Queen => PIECE_SIZE,
        PieceType::Bishop => PIECE_SIZE * 2.0,
        PieceType::Knight => PIECE_SIZE * 3.0,
        PieceType::Rook => PIECE_SIZE * 4.0,
        PieceType::Pawn => PIECE_SIZE * 5.0,
    };

    Rect::new(col_offset, row_offset, PIECE_SIZE, PIECE_SIZE)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum PieceType {
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
struct ChessPiece {
    piece: PieceType,
    color: Player,
}
