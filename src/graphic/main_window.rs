use super::chess_board::{ChessBoardData};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

use super::board_zone::game_zone_builder;

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
    game_zone_builder()
}
