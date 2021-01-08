use druid::text::{ArcStr, FontDescriptor, TextLayout};
use druid::widget::prelude::*;
use druid::{Color, FontFamily, FontWeight, Rect};

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
            label.set_font(FontDescriptor::new(FontFamily::SANS_SERIF).with_size(font_size).with_weight(FontWeight::BOLD));
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
            label.set_font(FontDescriptor::new(FontFamily::SANS_SERIF).with_size(font_size).with_weight(FontWeight::BOLD));
            label.set_text_color(color);
            label.rebuild_if_needed(ctx.text(), env);

            ctx.with_save(|ctx| {
                label.draw(ctx, (x1, y));
                label.draw(ctx, (x2, y));
            });
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

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &ChessBoardData, env: &Env) {
        self.draw_background(ctx);
        self.draw_cells(ctx);
        self.draw_coordinates(ctx, env);
    }
}
