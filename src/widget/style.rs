use crate::paint::style::StyleSet;

use super::*;

pub struct Style<T, E, W: Widget<T, E>, F>
{
    child: W,
    styling: F,
    _phantom: PhantomData<(T, E)>
}

impl<T, E, W: Widget<T, E>, F> Widget<T, E> for Style<T, E, W, F>
where F: Styling<T>
{
    impl_event_child!(T);
    impl_layout_inquire_child!(T);
    impl_layout_compute_child!(T);

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        let mut style = ctx.style.clone();
        self.styling.style(data, &mut style);
        std::mem::swap(&mut style, ctx.style);
        self.child.paint(ctx, data);
        std::mem::swap(&mut style, ctx.style);
    }
}

impl<T, E, W: Widget<T, E>, F> Style<T, E, W, F>
where F: Styling<T>
{
    pub fn new(widget: W, styling: F) -> Self
    {
        Self { child: widget, styling, _phantom: PhantomData }
    }
}

pub trait Styling<T>
{
    fn style(&mut self, data: &T, style_set: &mut StyleSet);
}

impl <T, F> Styling<T> for F
where F: FnMut(&T, &mut StyleSet)
{
    fn style(&mut self, data: &T, style_set: &mut StyleSet) {
        (self)(data, style_set)
    }
}

impl <T> Styling<T> for StyleSet
{
    fn style(&mut self, _data: &T, style_set: &mut StyleSet) {
        style_set.clone_from(self)
    }
}
