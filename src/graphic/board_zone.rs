use super::chess_board::{ChessBoard, ChessBoardData};
use super::svg_image_button::SvgImageToggleButton;

use druid::widget::Flex;
use druid::{Widget, WidgetExt, Size};

pub fn game_zone_builder() -> impl Widget<ChessBoardData> {
    let chess_board = ChessBoard::new();

    let button_toggle_board_orientation = SvgImageToggleButton::new(
        Size::new(490.667, 490.667),
        String::from(include_str!("./vectors/reverseArrows.svg")),
        String::from(include_str!("./vectors/reverseArrows.svg")),
    )
    .lens(ChessBoardData::reversed);
    let buttons_zone = Flex::row().with_child(button_toggle_board_orientation).padding(1.0);

    Flex::column()
        .with_flex_child(buttons_zone, 0.1)
        .with_flex_child(chess_board, 1.0)
}
