use druid::kurbo::Circle;
use druid::text::{ArcStr, FontDescriptor, TextLayout};
use druid::widget::prelude::*;
use druid::{widget::SvgData, Affine, Color, FontFamily, FontWeight, Lens, Rect};
use log::error;

use pleco::core::{sq::SQ, Piece, Player};
use pleco::Board;

#[derive(Lens, Data, Clone, Debug)]
pub struct ChessBoardData {
    board: BoardLogic,
    reversed: bool,
}

#[derive(Clone, Debug)]
struct BoardLogic {
    inner_logic: Board,
}

impl Data for BoardLogic {
    fn same(&self, other: &Self) -> bool {
        let this_fen = self.inner_logic.fen();
        let other_fen = other.inner_logic.fen();
        this_fen == other_fen
    }
}

impl ChessBoardData {
    pub fn new() -> Self {
        Self {
            board: BoardLogic {
                inner_logic: Board::from_fen(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                )
                .unwrap(),
            },
            reversed: false,
        }
    }
}

struct CellCoordinates {
    file: u8,
    rank: u8,
}

struct DragAndDropState {
    active: bool,
    start_cell: Option<CellCoordinates>,
    end_cell: Option<CellCoordinates>,
    moved_piece_value: Option<pleco::Piece>,
    moved_piece_location: Option<(f64, f64)>,
}

impl DragAndDropState {
    fn cancel(&mut self) {
        self.start_cell = None;
        self.end_cell = None;
        self.moved_piece_location = None;
        self.moved_piece_value = None;
        self.active = false;
    }
}

fn coordinates_to_square_algebraic(coordinates: &CellCoordinates) -> String {
    let file_str = (('a' as u8) + coordinates.file) as char;
    let rank_str = (('1' as u8) + coordinates.rank) as char;
    format!(
        "{}{}",
        file_str,
        rank_str,
    )
}

pub struct ChessBoard {
    dnd_state: DragAndDropState,
}

impl ChessBoard {
    pub fn new() -> Self {
        ChessBoard {
            dnd_state: DragAndDropState {
                active: false,
                start_cell: None,
                end_cell: None,
                moved_piece_location: None,
                moved_piece_value: None,
            },
        }
    }

    fn draw_background(&self, ctx: &mut PaintCtx) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::rgb8(214, 59, 96));
    }

    fn draw_cells(&self, ctx: &mut PaintCtx, data: &ChessBoardData) {
        let total_size = ctx.size().width;
        let cells_size = total_size * 0.1111;
        for row in 0..8 {
            for col in 0..8 {
                let is_white_cell = (row + col) % 2 > 0;
                let is_start_cell = self.is_start_cell(data, col, row);
                let is_end_cell = self.is_end_cell(data, col, row);

                let color = if is_end_cell {
                    Color::rgb8(112, 209, 35)
                } else if is_start_cell {
                    Color::rgb8(178, 46, 230)
                } else if is_white_cell {
                    Color::rgb8(255, 206, 158)
                } else {
                    Color::rgb8(209, 139, 71)
                };
                let x = cells_size * (0.5 + (col as f64));
                let y = cells_size * (0.5 + (row as f64));

                let rect = Rect::new(x, y, x + (cells_size as f64), y + (cells_size as f64));
                ctx.fill(rect, &color);
            }
        }
    }

    fn draw_coordinates(&self, ctx: &mut PaintCtx, data: &ChessBoardData, env: &Env) {
        let total_size = ctx.size().width;
        let cells_size = total_size * 0.1111;
        let font_size = cells_size * 0.3;

        let files_coordinates = if data.reversed {
            "HGFEDCBA"
        } else {
            "ABCDEFGH"
        };
        let rank_coordinates = if data.reversed {
            "12345678"
        } else {
            "87654321"
        };

        for (index, current_coord) in files_coordinates.chars().enumerate() {
            let x = cells_size * ((index as f64) + 0.9);
            let y1 = cells_size * 0.08;
            let y2 = cells_size * 8.58;
            let current_str = format!("{}", current_coord);

            let color = Color::rgb(255.0, 199.0, 0.0);
            let mut label = TextLayout::<ArcStr>::from_text(current_str);
            label.set_font(
                FontDescriptor::new(FontFamily::SANS_SERIF)
                    .with_size(font_size)
                    .with_weight(FontWeight::BOLD),
            );
            label.set_text_color(color);
            label.rebuild_if_needed(ctx.text(), env);

            ctx.with_save(|ctx| {
                label.draw(ctx, (x, y1));
                label.draw(ctx, (x, y2));
            });
        }

        for (index, current_coord) in rank_coordinates.chars().enumerate() {
            let y = cells_size * ((index as f64) + 0.85);
            let x1 = cells_size * 0.15;
            let x2 = cells_size * 8.65;
            let current_str = format!("{}", current_coord);

            let color = Color::rgb(255.0, 199.0, 0.0);
            let mut label = TextLayout::<ArcStr>::from_text(current_str);
            label.set_font(
                FontDescriptor::new(FontFamily::SANS_SERIF)
                    .with_size(font_size)
                    .with_weight(FontWeight::BOLD),
            );
            label.set_text_color(color);
            label.rebuild_if_needed(ctx.text(), env);

            ctx.with_save(|ctx| {
                label.draw(ctx, (x1, y));
                label.draw(ctx, (x2, y));
            });
        }
    }

    fn draw_pieces(&self, ctx: &mut PaintCtx, data: &ChessBoardData) {
        let total_size = ctx.size().width;
        let cells_size = total_size * 0.1111;

        for row in 0..8 {
            let rank = if data.reversed { row } else { 7 - row };
            for col in 0..8 {
                let file = if data.reversed { 7 - col } else { col };

                if let Some(start_cell_coordinates) = &self.dnd_state.start_cell {
                    let is_start_cell_piece =
                        file == start_cell_coordinates.file && rank == start_cell_coordinates.rank;
                    if is_start_cell_piece {
                        continue;
                    }
                }

                let square = SQ((file + 8 * rank) as u8);
                let piece = data.board.inner_logic.piece_at_sq(square);

                let piece_image_raw_data = ChessBoard::get_piece_image_raw_data(piece);
                if let Some(piece_image_raw_data) = piece_image_raw_data {
                    let piece_svg_data = match piece_image_raw_data.parse::<SvgData>() {
                        Ok(svg) => svg,
                        Err(err) => {
                            error!("{}", err);
                            error!("Using an empty SVG instead of {}.", piece_image_raw_data);
                            SvgData::default()
                        }
                    };
                    let ratio = (cells_size as f64) / 45_f64;
                    let x = cells_size * (0.5 + (col as f64));
                    let y = cells_size * (0.5 + (row as f64));
                    let affine_matrix = Affine::translate((x, y)) * Affine::scale(ratio);

                    ctx.with_save(|ctx| {
                        piece_svg_data.to_piet(affine_matrix, ctx);
                    });
                }
            }
        }
    }

    fn draw_moved_piece(&self, ctx: &mut PaintCtx) {
        if let Some(moved_piece) = self.dnd_state.moved_piece_value {
            let total_size = ctx.size().width;
            let cells_size = total_size * 0.1111;
            let ratio = (cells_size as f64) / 45_f64;
            let piece_image_raw_data = ChessBoard::get_piece_image_raw_data(moved_piece);
            if let Some(piece_image_raw_data) = piece_image_raw_data {
                let piece_svg_data = match piece_image_raw_data.parse::<SvgData>() {
                    Ok(svg) => svg,
                    Err(err) => {
                        error!("{}", err);
                        error!("Using an empty SVG instead of {}.", piece_image_raw_data);
                        SvgData::default()
                    }
                };

                if let Some(piece_location) = self.dnd_state.moved_piece_location {
                    let x = piece_location.0;
                    let y = piece_location.1;
                    let affine_matrix = Affine::translate((x, y)) * Affine::scale(ratio);

                    ctx.with_save(|ctx| {
                        piece_svg_data.to_piet(affine_matrix, ctx);
                    });
                }
            }
        }
    }

    fn draw_player_turn(&self, ctx: &mut PaintCtx, data: &ChessBoardData) {
        let total_size = ctx.size().width;
        let cells_size = total_size * 0.1111;

        let location = cells_size * 8.7625;
        let color = if data.board.inner_logic.turn() == Player::White {
            Color::WHITE
        } else {
            Color::BLACK
        };
        let radius = cells_size * 0.2;

        let circle = Circle::new((location, location), radius);
        ctx.fill(circle, &color);
    }

    fn is_start_cell(&self, data: &ChessBoardData, col: u8, row: u8) -> bool {
        if let Some(start_cell_coordinates) = &self.dnd_state.start_cell {
            let start_cell_col = if data.reversed {
                7 - start_cell_coordinates.file
            } else {
                start_cell_coordinates.file
            };
            let start_cell_row = if data.reversed {
                start_cell_coordinates.rank
            } else {
                7 - start_cell_coordinates.rank
            };
            start_cell_col == col && start_cell_row == row
        } else {
            false
        }
    }

    fn is_end_cell(&self, data: &ChessBoardData, col: u8, row: u8) -> bool {
        if let Some(end_cell_coordinates) = &self.dnd_state.end_cell {
            let end_cell_col = if data.reversed {
                7 - end_cell_coordinates.file
            } else {
                end_cell_coordinates.file
            };
            let end_cell_row = if data.reversed {
                end_cell_coordinates.rank
            } else {
                7 - end_cell_coordinates.rank
            };
            end_cell_col == col && end_cell_row == row
        } else {
            false
        }
    }

    fn get_piece_image_raw_data(piece: pleco::Piece) -> Option<&'static str> {
        match piece {
            Piece::WhitePawn => Some(include_str!("../merida/wP.svg")),
            Piece::WhiteKnight => Some(include_str!("../merida/wN.svg")),
            Piece::WhiteBishop => Some(include_str!("../merida/wB.svg")),
            Piece::WhiteRook => Some(include_str!("../merida/wR.svg")),
            Piece::WhiteQueen => Some(include_str!("../merida/wQ.svg")),
            Piece::WhiteKing => Some(include_str!("../merida/wK.svg")),
            Piece::BlackPawn => Some(include_str!("../merida/bP.svg")),
            Piece::BlackKnight => Some(include_str!("../merida/bN.svg")),
            Piece::BlackBishop => Some(include_str!("../merida/bB.svg")),
            Piece::BlackRook => Some(include_str!("../merida/bR.svg")),
            Piece::BlackQueen => Some(include_str!("../merida/bQ.svg")),
            Piece::BlackKing => Some(include_str!("../merida/bK.svg")),
            Piece::None => None,
        }
    }
}

impl Widget<ChessBoardData> for ChessBoard {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut ChessBoardData, _env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                if !self.dnd_state.active {
                    let x = mouse_event.pos.x;
                    let y = mouse_event.pos.y;

                    let total_size = ctx.size().width;
                    let cells_size = total_size * 0.1111;

                    let col = ((x - cells_size * 0.5) / cells_size).floor() as i32;
                    let row = ((y - cells_size * 0.5) / cells_size).floor() as i32;

                    let out_of_bounds = (col < 0) || (col > 7) || (row < 0) || (row > 7);
                    if out_of_bounds {
                        return;
                    }

                    let file = if data.reversed { 7 - col } else { col } as u8;
                    let rank = if data.reversed { row } else { 7 - row } as u8;

                    let square = SQ((file + 8 * rank) as u8);
                    let piece = data.board.inner_logic.piece_at_sq(square);
                    if piece == Piece::None {
                        return;
                    }

                    // The moved piece must be around the mouse cursor.
                    let x = x - cells_size * 0.5;
                    let y = y - cells_size * 0.5;

                    self.dnd_state.moved_piece_location = Some((x, y));
                    self.dnd_state.moved_piece_value = Some(piece);
                    self.dnd_state.start_cell = Some(CellCoordinates { file, rank });
                    self.dnd_state.active = true;
                    ctx.request_update();
                }
            }
            Event::MouseUp(_mouse_event) => {
                if self.dnd_state.active {
                    let start_square_algebraic = if let Some(ref start_cell) = self.dnd_state.start_cell {
                        coordinates_to_square_algebraic(start_cell)
                    }
                    else {
                        "".to_string()
                    };
                    let end_square_algebraic = if let Some(ref end_cell) = self.dnd_state.end_cell {
                        coordinates_to_square_algebraic(end_cell)
                    }
                    else {
                        "".to_string()
                    };
                    let promotion_piece = "";
                    let move_to_play = format!(
                        "{}{}{}",
                        start_square_algebraic,
                        end_square_algebraic,
                        promotion_piece,
                    );
                    data.board.inner_logic.apply_uci_move(&move_to_play);
                    
                    self.dnd_state.cancel();
                    ctx.request_update();
                }
            }
            Event::MouseMove(mouse_event) => {
                if self.dnd_state.active {
                    let x = mouse_event.pos.x;
                    let y = mouse_event.pos.y;

                    let total_size = ctx.size().width;
                    let cells_size = total_size * 0.1111;

                    let col = ((x - cells_size * 0.5) / cells_size).floor() as i32;
                    let row = ((y - cells_size * 0.5) / cells_size).floor() as i32;

                    let out_of_bounds = (col < 0) || (col > 7) || (row < 0) || (row > 7);
                    if out_of_bounds {
                        return;
                    }

                    let file = if data.reversed { 7 - col } else { col } as u8;
                    let rank = if data.reversed { row } else { 7 - row } as u8;

                    // The moved piece must be around the mouse cursor.
                    let x = x - cells_size * 0.5;
                    let y = y - cells_size * 0.5;

                    self.dnd_state.moved_piece_location = Some((x, y));
                    self.dnd_state.end_cell = Some(CellCoordinates { file, rank });
                    ctx.request_update();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        _data: &ChessBoardData,
        _env: &Env,
    ) {
        match event {
            LifeCycle::HotChanged(false) => {
                if self.dnd_state.active {
                    self.dnd_state.cancel();
                }
            }
            _ => {}
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &ChessBoardData,
        _data: &ChessBoardData,
        _env: &Env,
    ) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &ChessBoardData,
        _env: &Env,
    ) -> Size {
        let max_size = bc.max();
        let max_width = max_size.width;
        let max_height = max_size.height;

        let new_common_size = if max_width < max_height {
            max_width
        } else {
            max_height
        };
        bc.constrain(Size::new(new_common_size, new_common_size))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &ChessBoardData, env: &Env) {
        self.draw_background(ctx);
        self.draw_cells(ctx, data);
        self.draw_coordinates(ctx, data, env);
        self.draw_pieces(ctx, data);
        self.draw_moved_piece(ctx);
        self.draw_player_turn(ctx, data);
    }
}
