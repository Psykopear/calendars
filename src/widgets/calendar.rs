use druid::kurbo::Size;
use druid::piet::RenderContext;
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Rect, UpdateCtx, Widget,
};

use crate::utils::hsl_to_rgb;

pub struct Calendar<T> {
    selected_month: T,
}

impl<T: Data> Calendar<T> {
    pub fn new(selected_month: impl Into<T>) -> Calendar<T> {
        Calendar {
            selected_month: selected_month.into(),
        }
    }
}

impl<T: Data> Widget<T> for Calendar<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &T, _env: &Env) {
        let rgb1 = hsl_to_rgb(0.5, 0.5, 0.5);
        let rgb2 = hsl_to_rgb(0.2, 0.2, 0.2);
        paint_ctx.clear(Color::rgb8(rgb1.0, rgb1.1, rgb1.2));
        let rect = Rect::from_origin_size((10., 10.), (100., 100.));
        paint_ctx.fill(rect, &Color::rgb8(rgb2.0, rgb2.1, rgb2.2));
    }
}
