use super::chess_board::{ChessBoard, ChessBoardData};
use super::svg_image_button::SvgImageToggleButton;

use druid::widget::Flex;
use druid::{Widget, WidgetExt};

pub fn game_zone_builder() -> impl Widget<ChessBoardData> {
    let chess_board = ChessBoard::new();

    let button_toggle_board_orientation = SvgImageToggleButton::new(
        String::from(include_str!("./vectors/arrowUp.svg")),
        String::from(include_str!("./vectors/arrowDown.svg")),
    )
    .lens(ChessBoardData::reversed);
    let buttons_zone = Flex::row().with_child(button_toggle_board_orientation);

    Flex::column()
        .with_child(buttons_zone)
        .with_flex_child(chess_board, 1.0)
}
