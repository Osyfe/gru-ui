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

enum WatchLevel
{
    Paint,
    Layout,
    Widget
}

pub struct Watch<T: Clone + PartialEq, E, W: Widget<T, E>>
{
    child: W,
    data: Option<T>,
    level: WatchLevel,
    _phantom: PhantomData<E>
}

impl<T: Clone + PartialEq, E, W: Widget<T, E>> Widget<T, E> for Watch<T, E, W>
{
    impl_event_child!(T);

    #[inline]
    fn update(&mut self, ctx: &mut UpdateCtx, data: &mut T)
    {
        self.child.update(ctx, data);
        if self.data.as_ref() != Some(data)
        {
            match &self.level
            {
                WatchLevel::Paint => ctx.request.paint(),
                WatchLevel::Layout => ctx.request.layout(),
                WatchLevel::Widget => ctx.request.widget()
            }
            self.data = Some(data.clone());
        }
    }

    impl_widget_compute_child!(T);
    impl_layout_inquire_child!(T);
    impl_layout_compute_child!(T);
    impl_paint_child!(T);
    impl_respond_child!(T);
}

impl<T: Clone + PartialEq, E, W: Widget<T, E>> Watch<T, E, W>
{
    pub fn new(widget: W) -> Self 
    {
        Self
        {
            child: widget,
            data: None,
            level: WatchLevel::Paint,
            _phantom: PhantomData
        }
    }

    pub fn paint(mut self) -> Self
    {
        self.level = WatchLevel::Paint;
        self
    }

    pub fn layout(mut self) -> Self
    {
        self.level = WatchLevel::Layout;
        self
    }

    pub fn widget(mut self) -> Self
    {
        self.level = WatchLevel::Widget;
        self
    }
}
