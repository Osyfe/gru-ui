use super::*;

pub struct Dynamic<'a, T, E, W: Widget<T, E>, F: FnMut(&mut T) -> W + 'a>
{
    child: Option<W>,
    f: F,
    _phantom: PhantomData<&'a (T, E)>
}

impl<'a, T, E, W: Widget<T, E>, F: FnMut(&mut T) -> W + 'a> Widget<T, E> for Dynamic<'a, T, E, W, F>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T)
    {
        if let Some(child) = &mut self.child { child.event(ctx, data); }
    }

    #[inline]
    fn update(&mut self, ctx: &mut UpdateCtx, data: &mut T)
    {
        if let Some(child) = &mut self.child { child.update(ctx, data); }
    }

    #[inline]
    fn widget_compute(&mut self, ctx: &mut WidgetComputeCtx, data: &mut T)
    {
        let mut child = (self.f)(data);
        child.widget_compute(ctx, data);
        self.child = Some(child);
    }

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2
    {
        if let Some(child) = &mut self.child { child.layout_inquire(ctx, data) }
        else { Vec2::zero() }
    }

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2
    {
        if let Some(child) = &mut self.child { child.layout_compute(ctx, data, size) }
        else { Vec2::zero() }
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        if let Some(child) = &mut self.child { child.paint(ctx, data); }
    }

    #[inline]
    fn respond(&mut self, data: &mut T, button: Option<MouseButton>) -> bool
    {
        if let Some(child) = &mut self.child { child.respond(data, button) }
        else { false }
    }
}

impl<'a, T, E, W: Widget<T, E>, F: FnMut(&mut T) -> W + 'a> Dynamic<'a, T, E, W, F>
{
    pub fn new(f: F) -> Self
    {
        Self { child: None, f, _phantom: PhantomData }
    }
}
