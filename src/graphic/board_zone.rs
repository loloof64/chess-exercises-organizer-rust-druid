use super::chess_board::{ChessBoard, ChessBoardData};
use druid::widget::{Button, Flex};
use druid::Widget;

pub fn game_zone_builder() -> impl Widget<ChessBoardData> {
    let chess_board = ChessBoard::new();

    let button_toggle_board_orientation = Button::new("Toggle")
        .on_click(|_event_ctx, data: &mut ChessBoardData, _env: &_| data.set_reversed(!data.is_reversed()));
    let buttons_zone = Flex::row()
        .with_child(button_toggle_board_orientation);

    Flex::column()
        .with_child(buttons_zone)
        .with_flex_child(chess_board, 1.0)
}
