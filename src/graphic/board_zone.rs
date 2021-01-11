use super::chess_board::{ChessBoard, ChessBoardData, TOGGLE_ORIENTATION};
use super::svg_image_button::{SvgImageButton, SvgImageButtonData};

use druid::widget::{Controller, Flex};
use druid::{Env, UpdateCtx, Widget, WidgetExt, Data, Lens};

#[derive(Data, Lens, Clone)]
struct BoardZoneData {
    board_reversed: bool,
}

struct BoardZoneDataController;

impl<W: Widget<ChessBoardData>> Controller<ChessBoardData, W> for BoardZoneDataController {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &ChessBoardData,
        data: &ChessBoardData,
        env: &Env,
    ) {
        ctx.submit_command(TOGGLE_ORIENTATION);
        child.update(ctx, old_data, data, env);
    }
}

pub fn game_zone_builder() -> impl Widget<BoardZoneData> {
    let chess_board = ChessBoard::new();

    let button_toggle_board_orientation =
        SvgImageButton::new(String::from(include_str!("./vectors/reverseArrows.svg")));
    let buttons_zone = Flex::row().with_child(button_toggle_board_orientation);

    Flex::column()
        .with_flex_child(chess_board, 1.0)
        .controller(BoardZoneDataController)
}
