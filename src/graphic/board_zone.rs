use super::chess_board::{ChessBoard, ChessBoardData};
use druid::widget::{Flex, Button};
use druid::{Widget};

pub fn game_zone_builder() -> impl Widget<ChessBoardData> {
    let chess_board = ChessBoard::new();

    let button = Button::new("Reverse").on_click(|_, data: &mut ChessBoardData, _: &_| data.set_reversed(true));

    Flex::column().with_child(button).with_flex_child(chess_board, 1.0)
}
