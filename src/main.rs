mod widgets;

use druid::widget::{Button, Flex, WidgetExt};
use druid::{theme,Color, LocalizedString, AppLauncher, Data, Env, EventCtx, Lens, Widget, WindowDesc};
use std::sync::Arc;

use chrono::{Date, Datelike, Duration, NaiveDate, Utc};

use widgets::Grid;

#[derive(Clone, Data, Lens)]
struct AppState {
    selected_date: Arc<Date<Utc>>,
}

/// Number of days in a month. Returns i64 because that's what Duration takes
fn month_days(date: Date<Utc>) -> i64 {
    NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd(date.year() + 1, 1, 1))
        .pred()
        .day() as i64
}

fn prev_month_days(date: Date<Utc>) -> i64 {
    date.with_day(1).expect("This should never fail").pred().day() as i64
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
                            let date = date - Duration::days(prev_month_days(date));
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
                            let date = date + Duration::days(month_days(date));
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
        .title(LocalizedString::new("calendars").with_placeholder(String::from("calendars")))
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
