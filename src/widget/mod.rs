use super::*;
use layout::{self, ScaleWish, SizeWish};
use style::StyleSet;
use math::{Vec2, Rect};
use text::Align;
use std::{borrow::Borrow, marker::PhantomData};

impl<'a, T> Widget<T> for Box<dyn Widget<T> + 'a>
{
    #[inline] fn event(&mut self, ctx: &mut EventCtx, data: &mut T) { self.as_mut().event(ctx, data); }
    #[inline] fn update(&mut self, ctx: &mut UpdateCtx, data: &mut T) { self.as_mut().update(ctx, data); }
    #[inline] fn widget_compute(&mut self, ctx: &mut WidgetComputeCtx, data: &mut T) { self.as_mut().widget_compute(ctx, data); }
    #[inline] fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> SizeWish { self.as_mut().layout_inquire(ctx, data) }
    #[inline] fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) { self.as_mut().layout_compute(ctx, data, size); }
    #[inline] fn paint(&mut self, ctx: &mut PaintCtx, data: &T, style: &StyleSet) { self.as_mut().paint(ctx, data, style); }
}

pub struct Label<T: Borrow<str>>
{
    text_size: f32,
    size: Vec2,
    _phantom: PhantomData<T>
}

impl<T: Borrow<str>> Widget<T> for Label<T>
{
    impl_event_empty!();
    impl_update_empty!();
    impl_widget_compute_empty!();

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> SizeWish
    {
        let width = ctx.text_with(data.borrow(), self.text_size);
        self.size = Vec2(width, self.text_size);
        SizeWish::new_fixed(self.size)
    }

    impl_layout_compute_empty!();

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, style: &StyleSet)
    {
        ctx.draw_text(Rect::new_origin(self.size), data.borrow(), self.text_size, Align::Left, false, style.text);
    }
}

impl<T: Borrow<str>> Label<T>
{
    pub fn new() -> Self
    {
        Self { text_size: 1.0, size: Vec2::zero(), _phantom: PhantomData }
    }

    pub fn size(mut self, text_size: f32) -> Self
    {
        self.text_size = text_size;
        self
    }
}
