mod widgets;

use druid::widget::{Flex, Label, WidgetExt, Align};
use druid::{AppLauncher, Data, Lens, Widget, WindowDesc};
use std::sync::Arc;

use chrono::{Date, Utc};

use widgets::Grid;

#[derive(Clone, Data, Lens)]
struct AppState {
    selected_date: Arc<Date<Utc>>,
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
            .with_child(Label::new("<").align_right(), 3.)
            .with_child(Label::new("Marzo 2020").center(), 1.)
            .with_child(Label::new(">").align_left(), 3.),
        1.)
        .with_child(Grid::new().center(), 6.)
}

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((750., 750.))
        .resizable(false);
    let data = AppState {
        selected_date: Arc::new(Utc::today())
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Launch failed");
}
