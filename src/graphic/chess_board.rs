use druid::text::{ArcStr, FontDescriptor, TextLayout};
use druid::widget::prelude::*;
use druid::{widget::SvgData, Color, FontFamily, FontWeight, Rect, Affine};
use log::error;

use pleco::core::{sq::SQ, Piece};
use pleco::Board;

#[derive(Data, Clone, Debug)]
pub struct ChessBoardData {
    position: String,
}

impl ChessBoardData {
    pub fn new() -> Self {
        Self {
            position: String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        }
    }
}

pub struct ChessBoard;

impl ChessBoard {
    pub fn new() -> Self {
        Self {}
    }

    fn draw_background(&self, ctx: &mut PaintCtx) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::rgb8(214, 59, 96));
    }

    fn draw_cells(&self, ctx: &mut PaintCtx) {
        let total_size = ctx.size().width;
        let cells_size = total_size * 0.1111;
        for row in 0..8 {
            for col in 0..8 {
                let is_white_cell = (row + col) % 2 > 0;
                let color = if is_white_cell {
                    Color::rgb8(255, 206, 158)
                } else {
                    Color::rgb8(209, 139, 71)
                };
                let x = cells_size * (0.5 + (col as f64));
                let y = cells_size * (7.5 - (row as f64));

                let rect = Rect::new(x, y, x + (cells_size as f64), y + (cells_size as f64));
                ctx.fill(rect, &color);
            }
        }
    }

    fn draw_coordinates(&self, ctx: &mut PaintCtx, env: &Env) {
        let total_size = ctx.size().width;
        let cells_size = total_size * 0.1111;
        let font_size = cells_size * 0.3;

        let files_coordinates = "ABCDEFGH";
        let rank_coordinates = "87654321";

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
        let position = data.position.as_str();

        let board = Board::from_fen(position).expect(
            format!(
                "Could not create board logic from current position {} !",
                position
            )
            .as_str(),
        );
        for row in 0..8 {
            let rank = 7-row;
            for col in 0..8 {
                let file = col;

                let square = SQ((file + 8 * rank) as u8);
                let piece = board.piece_at_sq(square);

                let piece_image_path = match piece {
                    Piece::WhitePawn => Some(include_str!("../merida/wP.svg")),
                    Piece::WhiteKnight =>  Some(include_str!("../merida/wN.svg")),
                    Piece::WhiteBishop =>  Some(include_str!("../merida/wB.svg")),
                    Piece::WhiteRook =>  Some(include_str!("../merida/wR.svg")),
                    Piece::WhiteQueen =>  Some(include_str!("../merida/wQ.svg")),
                    Piece::WhiteKing =>  Some(include_str!("../merida/wK.svg")),
                    Piece::BlackPawn =>  Some(include_str!("../merida/bP.svg")),
                    Piece::BlackKnight =>  Some(include_str!("../merida/bN.svg")),
                    Piece::BlackBishop =>  Some(include_str!("../merida/bB.svg")),
                    Piece::BlackRook =>  Some(include_str!("../merida/bR.svg")),
                    Piece::BlackQueen => Some(include_str! ("../merida/bQ.svg")),
                    Piece::BlackKing =>  Some(include_str!("../merida/bK.svg")),
                    Piece::None => None,
                };
                if let Some(piece_image_path) = piece_image_path {
                    let piece_svg_data = match piece_image_path.parse::<SvgData>(){
                        Ok(svg) => svg,
                        Err(err) => {
                            error!("{}", err);
                            error!("Using an empty SVG instead of {}.", piece_image_path);
                            SvgData::default()
                        }
                    };
                    
                    let ratio = (cells_size as f64) / 45_f64;
                    let x = cells_size * (0.5 + (col as f64));
                    let y = cells_size * (0.5 + (row as f64));
                    let affine_matrix =Affine::translate((x, y)) * Affine::scale(ratio);

                    ctx.with_save(|ctx| {
                        piece_svg_data.to_piet(affine_matrix, ctx);
                    });
                }
            }
        }
    }
}

impl Widget<ChessBoardData> for ChessBoard {
    fn event(
        &mut self,
        _ctx: &mut EventCtx,
        _event: &Event,
        _data: &mut ChessBoardData,
        _env: &Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &ChessBoardData,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &ChessBoardData,
        _data: &ChessBoardData,
        _env: &Env,
    ) {
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
        self.draw_cells(ctx);
        self.draw_coordinates(ctx, env);
        self.draw_pieces(ctx, data);
    }
}
