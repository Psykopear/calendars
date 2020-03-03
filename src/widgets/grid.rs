use druid::kurbo::Size;
use druid::piet::{
    FontBuilder, PietText, PietTextLayout, RenderContext, Text, TextLayout, TextLayoutBuilder,
};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Rect, UpdateCtx, Widget,
};
use std::collections::HashMap;

use crate::AppState;

const PADDING: f64 = 10.;
const MAIN_COLOR: Color = Color::rgb8(0xc2, 0xc2, 0xc2);
const SECONDARY_COLOR: Color = Color::rgb8(80, 80, 80);
const SIZE: f64 = 100.;

pub struct Grid {
    label: Option<PietTextLayout>,
    font_size: f64,
    month: u8,
}

impl Grid {
    pub fn new(month: u8) -> Self {
        Grid {
            month,
            label: None,
            font_size: SIZE / 7.,
        }
    }

    fn resolve(&mut self, piet_text: &mut PietText, row: u8, col: u8) {
        let font_name = "sans-serif";
        let font = piet_text
            .new_font_by_name(font_name, self.font_size)
            .build()
            .unwrap();

        let mut days = HashMap::new();
        days.insert(1, "Lun");
        days.insert(2, "Mar");
        days.insert(3, "Mer");
        days.insert(4, "Gio");
        days.insert(5, "Ven");
        days.insert(6, "Sab");
        days.insert(7, "Dom");

        let name = match col {
            1 => format!("{} {}", days.get(&row).unwrap(), row),
            x => format!("{}", (7 * (x - 1)) + row),
        };
        self.label = Some(piet_text.new_text_layout(&font, &name).build().unwrap());
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

    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {
        let w0 = SIZE;
        let h0 = SIZE;
        for row in 0..7 {
            for col in 0..6 {
                let origin = (w0 * row as f64 , h0 * col as f64);
                let rect = Rect::from_origin_size(origin, (SIZE, SIZE));
                paint_ctx.stroke(rect, &SECONDARY_COLOR, 1.);
                self.resolve(paint_ctx.text(), row + 1, col + 1);
                if let Some(label) = &self.label {
                    let origin = Point::from((
                        origin.0 + SIZE - label.width() - PADDING,
                        origin.1 + SIZE / 5.,
                    ));
                    paint_ctx.draw_text(&label, origin, &MAIN_COLOR);
                }
            }
        }
    }
}
