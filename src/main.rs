mod widgets;

use druid::widget::{Flex, Label, WidgetExt, Align};
use druid::{AppLauncher, Data, Lens, Widget, WindowDesc};

use widgets::Grid;

#[derive(Clone, Default, Data, Lens)]
struct AppState {
    selected_month: u8,
    selected_day: u8,
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("<     Marzo 2020     >").center(), 1.)
        .with_child(Grid::new(1).center(), 6.)
}

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((750., 750.))
        .resizable(false);
    let data = AppState {
        selected_month: 1,
        selected_day: 1,
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Launch failed");
}
