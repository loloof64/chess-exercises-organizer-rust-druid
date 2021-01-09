use super::chess_board::{ChessBoard, ChessBoardData};
use druid::widget::Flex;
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

pub fn launch() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .title("Chess exercises organizer")
        .window_size((600.0, 400.0));
    let data = ChessBoardData::new();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<ChessBoardData> {
    let chess_board = ChessBoard::new();

    Flex::column().with_flex_child(chess_board, 1.0)
}
