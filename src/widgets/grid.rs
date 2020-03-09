use chrono::{Datelike, Weekday};
use druid::kurbo::Size;
use druid::piet::{
    FontBuilder, PietText, PietTextLayout, RenderContext, Text, TextLayout, TextLayoutBuilder,
};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Rect, UpdateCtx, Widget,
};

use chrono::{Date, Utc};

use crate::AppState;

const PADDING: f64 = 10.;
const MAIN_COLOR: Color = Color::rgb8(0xc2, 0xc2, 0xc2);
const SECONDARY_COLOR: Color = Color::rgb8(80, 80, 80);
const SIZE: f64 = 100.;

pub struct Grid {
    label: Option<PietTextLayout>,
    font_size: f64,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            label: None,
            font_size: SIZE / 7.,
        }
    }

    fn resolve(&mut self, piet_text: &mut PietText, date: Date<Utc>, show_weekday: bool) {
        let font_name = "sans-serif";
        let font = piet_text
            .new_font_by_name(font_name, self.font_size)
            .build()
            .unwrap();

        let label = if show_weekday {
            format!("{} {}", date.weekday(), date.day())
        } else {
            date.day().to_string()
        };
        self.label = Some(piet_text.new_text_layout(&font, &label).build().unwrap());
    }
}

impl Widget<AppState> for Grid {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.constrain((SIZE * 7., SIZE * 6.))
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let w0 = SIZE;
        let h0 = SIZE;
        for row in 0..7 {
            for col in 0..6 {
                let origin = (w0 * row as f64, h0 * col as f64);
                let rect = Rect::from_origin_size(origin, (SIZE, SIZE));
                paint_ctx.stroke(rect, &SECONDARY_COLOR, 1.);

                let first_day: Date<Utc> = data
                    .selected_date
                    .with_day(1)
                    .expect("Couldn't get first day of the month");
                let weekday = match row {
                    0 => Weekday::Mon,
                    1 => Weekday::Tue,
                    2 => Weekday::Wed,
                    3 => Weekday::Thu,
                    4 => Weekday::Fri,
                    5 => Weekday::Sat,
                    6 => Weekday::Sun,
                    // If we get here, I think the program can reasonably crash
                    _ => unreachable!(),
                };
                let offset = weekday.number_from_monday() as i64
                    - first_day.weekday().number_from_monday() as i64;
                let diff = (7 * col as i64) + offset;
                let date = first_day + chrono::Duration::days(diff);

                // Paint lighter background if selected day
                if date == *data.selected_date {
                    paint_ctx.fill(
                        Rect::from_origin_size(origin, Size::new(w0, h0)),
                        &Color::rgba8(0xff, 0xff, 0xff, 0x09),
                    );
                }

                self.resolve(paint_ctx.text(), date, col == 0);

                if let Some(label) = &self.label {
                    let origin = Point::from((
                        origin.0 + SIZE - label.width() - PADDING,
                        origin.1 + SIZE / 5.,
                    ));
                    let color = if date.month() == data.selected_date.month() {
                        MAIN_COLOR
                    } else {
                        SECONDARY_COLOR
                    };
                    paint_ctx.draw_text(&label, origin, &color);
                }
            }
        }
    }
}
