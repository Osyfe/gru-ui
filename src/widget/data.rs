use super::*;

pub struct Lensing<U, T, E, W: Widget<T, E>, L: Lens<U, T>>
{
    child: W,
    lens: L,
    _phantom: PhantomData<(U, T, E)>
}

impl<U, T, E, W: Widget<T, E>, L: Lens<U, T>> Lensing<U, T, E, W, L>
{
    pub fn new(widget: W, lens: L) -> Self
    {
        Self { child: widget, lens, _phantom: PhantomData }
    }
}

impl<U, T, E, W: Widget<T, E>, L: Lens<U, T>> Widget<U, E> for Lensing<U, T, E, W, L>
{
    #[inline] fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut U) { self.lens.with_mut(data, |data| self.child.event(ctx, data)); }
    #[inline] fn update(&mut self, ctx: &mut UpdateCtx, data: &mut U) { self.lens.with_mut(data, |data| self.child.update(ctx, data)); }
    #[inline] fn widget_compute(&mut self, ctx: &mut WidgetComputeCtx, data: &mut U) { self.lens.with_mut(data, |data| self.child.widget_compute(ctx, data)); }
    #[inline] fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &U) -> Vec2 { self.lens.with(data, |data| self.child.layout_inquire(ctx, data)) }
    #[inline] fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &U, size: Vec2) -> Vec2 { self.lens.with(data, |data| self.child.layout_compute(ctx, data, size)) }
    #[inline] fn paint(&mut self, ctx: &mut PaintCtx, data: &U) { self.lens.with(data, |data| self.child.paint(ctx, data)); }
    #[inline] fn respond(&mut self, data: &mut U, button: Option<MouseButton>) -> bool { self.lens.with_mut(data, |data| self.child.respond(data, button)) }
}

pub struct Owning<U, T, E, W: Widget<T, E>>
{
    child: W,
    data: T,
    _phantom: PhantomData<(U, E)>
}

impl<U, T, E, W: Widget<T, E>> Widget<U, E> for Owning<U, T, E, W>
{
    impl_event_empty!(U);
    impl_update_empty!(U);
    impl_widget_compute_empty!(U);

    #[inline] fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, _: &U) -> Vec2 { self.child.layout_inquire(ctx, &self.data) }
    #[inline] fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, _: &U, size: Vec2) -> Vec2 { self.child.layout_compute(ctx, &self.data, size) }
    #[inline] fn paint(&mut self, ctx: &mut PaintCtx, _: &U) { self.child.paint(ctx, &self.data); }

    impl_respond_empty!(U);
}

impl<U, T, E, W: Widget<T, E>> Owning<U, T, E, W>
{
    pub fn new(widget: W, data: T) -> Self
    {
        Self { child: widget, data, _phantom: PhantomData }
    }
}

pub struct Map<'a, U, T, E, W: Widget<T, E>, F: FnMut(&U) -> T + 'a>
{
    child: W,
    data: Option<T>,
    f: F,
    _phantom: PhantomData<&'a (U, E)>
}

impl<'a, U, T, E, W: Widget<T, E>, F: FnMut(&U) -> T + 'a> Widget<U, E> for Map<'a, U, T, E, W, F>
{
    impl_event_empty!(U);
    impl_update_empty!(U);
    impl_widget_compute_empty!(U);

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &U) -> Vec2
    {
        self.data = Some((self.f)(data));
        match &self.data
        {
            Some(data) => self.child.layout_inquire(ctx, data),
            None => Vec2::zero()
        }
    }

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, _: &U, size: Vec2) -> Vec2
    {
        match &self.data
        {
            Some(data) => self.child.layout_compute(ctx, data, size),
            None => Vec2::zero()
        }
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, _: &U)
    {
        self.data.as_ref().map(|data| self.child.paint(ctx, data));
    }

    impl_respond_empty!(U);
}

impl<'a, U, T, E, W: Widget<T, E>, F: FnMut(&U) -> T + 'a> Map<'a, U, T, E, W, F>
{
    pub fn new(widget: W, f: F) -> Self
    {
        Self { child: widget, data: None, f, _phantom: PhantomData }
    }
}
