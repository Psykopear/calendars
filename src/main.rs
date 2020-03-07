mod widgets;

use druid::widget::{Button, Flex, Label, WidgetExt};
use druid::{theme,Color, AppLauncher, Data, Env, EventCtx, Lens, Widget, WindowDesc};
use std::sync::Arc;

use chrono::{Date, Datelike, Duration, NaiveDate, Utc};

use widgets::Grid;

#[derive(Clone, Data, Lens)]
struct AppState {
    selected_date: Arc<Date<Utc>>,
}

fn month_days(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1))
        .pred()
        .day()
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(
                    Button::new(
                        "<",
                        |_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                            let date: Date<Utc> = *data.selected_date;
                            let date =
                                date - Duration::days(month_days(date.year(), date.month()) as i64);
                            data.selected_date = Arc::new(date);
                        },
                    )
                    .align_right(),
                    2.,
                )
                .with_child(
                    Button::new(
                        |data: &AppState, _env: &_| data.selected_date.format("%B %Y").to_string(),
                        |_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                            data.selected_date = Arc::new(Utc::today());
                        },
                    )
                    .center(),
                    1.,
                )
                .with_child(
                    Button::new(
                        ">",
                        |_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
                            let date: Date<Utc> = *data.selected_date;
                            let date =
                                date + Duration::days(month_days(date.year(), date.month()) as i64);
                            data.selected_date = Arc::new(date);
                        },
                    )
                    .align_left(),
                    2.,
                ),
            1.,
        )
        .with_child(Grid::new().center(), 6.)
}

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((750., 750.))
        .show_titlebar(false)
        .resizable(false);
    let data = AppState {
        selected_date: Arc::new(Utc::today()),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .configure_env(|env, _| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::rgb8(0x29, 0x29, 0x29));
            env.set(theme::BUTTON_DARK, Color::rgb8(0x29, 0x29, 0x29));
            env.set(theme::BUTTON_LIGHT, Color::rgb8(0x29, 0x29, 0x29));
            env.set(theme::BUTTON_BORDER_RADIUS, 0.);
            env.set(theme::BUTTON_BORDER_WIDTH, 0.);
        })
        .launch(data)
        .expect("Launch failed");
}
