mod utils;
mod widgets;

use druid::widget::WidgetExt;
use druid::{AppLauncher, Data, Lens, Widget, WindowDesc};

use widgets::Calendar;

#[derive(Clone, Default, Data, Lens)]
struct AppState {
    selected_month: u8,
    selected_day: u8,
}

fn ui_builder() -> impl Widget<AppState> {
    Calendar::new(3).lens(AppState::selected_month)
}

fn main() {
    let main_window = WindowDesc::new(ui_builder);
    let data = AppState::default();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Launch failed");
}
