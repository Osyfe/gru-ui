use super::*;

pub struct Lensing<U, T, E, W: Widget<T, E>, L: Lens<U, T>>
{
    child: W,
    lens: L,
    _phantom: PhantomData<(U, T, E)>,
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
    #[inline] fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &U) -> Vec2 { self.lens.with(data, |data| self.child.layout_inquire(ctx, data)) }
    #[inline] fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &U, size: Vec2) -> Vec2 { self.lens.with(data, |data| self.child.layout_compute(ctx, data, size)) }
    #[inline] fn paint(&mut self, ctx: &mut PaintCtx, data: &U) { self.lens.with(data, |data| self.child.paint(ctx, data)); }
}

pub struct Owning<U, T, E, W: Widget<T, E>>
{
    child: W,
    data: T,
    _phantom: PhantomData<(U, E)>
}

impl<U, T, E, W: Widget<T, E>> Widget<U, E> for Owning<U, T, E, W>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, _: &mut U)
    {
        match ctx.event
        {
            WidgetEvent::NewData => {},
            _ => self.child.event(ctx, &mut self.data),
        }
    }

    #[inline] fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, _: &U) -> Vec2 { self.child.layout_inquire(ctx, &self.data) }
    #[inline] fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, _: &U, size: Vec2) -> Vec2 { self.child.layout_compute(ctx, &self.data, size) }
    #[inline] fn paint(&mut self, ctx: &mut PaintCtx, _: &U) { self.child.paint(ctx, &self.data); }
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
    _phantom: PhantomData<&'a (U, E)>,
}

impl<'a, U, T, E, W: Widget<T, E>, F: FnMut(&U) -> T + 'a> Widget<U, E> for Map<'a, U, T, E, W, F>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut U)
    {
        match ctx.event
        {
            WidgetEvent::NewData =>
            {
                let mut data_t = (self.f)(data);
                self.child.event(ctx, &mut data_t);
                self.data = Some(data_t);
            },
            _ => self.child.event(ctx, self.data.as_mut().unwrap()),
        }
    }

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, _: &U) -> Vec2
    {
        self.child.layout_inquire(ctx, self.data.as_ref().unwrap())
    }

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, _: &U, size: Vec2) -> Vec2
    {
        self.child.layout_compute(ctx, self.data.as_ref().unwrap(), size)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, _: &U)
    {
        self.child.paint(ctx, self.data.as_ref().unwrap());
    }
}

impl<'a, U, T, E, W: Widget<T, E>, F: FnMut(&U) -> T + 'a> Map<'a, U, T, E, W, F>
{
    pub fn new(widget: W, f: F) -> Self
    {
        Self { child: widget, data: None, f, _phantom: PhantomData }
    }
}

enum UpdateLevel
{
    None,
    Paint,
    Layout,
    Widget,
}

pub struct Cache<T: Clone + PartialEq, E, W: Widget<T, E>>
{
    child: W,
    data: Option<T>,
    level: UpdateLevel,
    _phantom: PhantomData<E>,
}

impl<T: Clone + PartialEq, E, W: Widget<T, E>> Widget<T, E> for Cache<T, E, W>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T)
    {
        match ctx.event
        {
            WidgetEvent::NewData => if self.data.as_ref() != Some(data)
            {
                self.child.event(ctx, data);
                match &self.level
                {
                    UpdateLevel::None => {},
                    UpdateLevel::Paint => ctx.request.paint(),
                    UpdateLevel::Layout => ctx.request.layout(),
                    UpdateLevel::Widget => ctx.request.widget(),
                }
                self.data = Some(data.clone());
            },
            _ => self.child.event(ctx, data),
        }
    }

    impl_layout_inquire_child!(T);
    impl_layout_compute_child!(T);
    impl_paint_child!(T);
}

impl<T: Clone + PartialEq, E, W: Widget<T, E>> Cache<T, E, W>
{
    pub fn new(widget: W) -> Self 
    {
        Self
        {
            child: widget,
            data: None,
            level: UpdateLevel::None,
            _phantom: PhantomData,
        }
    }

    pub fn update_paint(mut self) -> Self
    {
        self.level = UpdateLevel::Paint;
        self
    }

    pub fn update_layout(mut self) -> Self
    {
        self.level = UpdateLevel::Layout;
        self
    }

    pub fn update_widget(mut self) -> Self
    {
        self.level = UpdateLevel::Widget;
        self
    }
}
