use super::*;

pub struct And<T, E, W1: Widget<T, E>, W2: Widget<T, E>>
{
    child1: W1,
    child2: W2,
    _phantom: PhantomData<(T, E)>
}

impl<T, E, W1: Widget<T, E>, W2: Widget<T, E>> Widget<T, E> for And<T, E, W1, W2>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T)
    {
        self.child2.event(ctx, data);
        self.child1.event(ctx, data);
    }

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2
    {
        let size1 = self.child1.layout_inquire(ctx, data);
        let size2 = self.child2.layout_inquire(ctx, data);
        size1.component_max(size2)
    }

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2
    {
        let size1 = self.child1.layout_compute(ctx, data, size);
        let size2 = self.child2.layout_compute(ctx, data, size);
        size1.component_max(size2)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        self.child1.paint(ctx, data);
        self.child2.paint(ctx, data);
    }
}

impl<T, E, W1: Widget<T, E>, W2: Widget<T, E>> And<T, E, W1, W2>
{
    pub fn new(widget1: W1, widget2: W2) -> Self
    {
        Self { child1: widget1, child2: widget2, _phantom: PhantomData }
    }
}

pub struct Set<'a, T, E>
{
    childs: Vec<Box<dyn Widget<T, E> + 'a>>
}

impl<'a, T, E> Widget<T, E> for Set<'a, T, E>
{
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T)
    {
        for child in self.childs.iter_mut().rev() { child.event(ctx, data); }
    }

    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2
    {
        let mut max_size = Vec2::zero();
        for child in &mut self.childs { max_size = child.layout_inquire(ctx, data).component_max(max_size); }
        max_size
    }

    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2
    {
        let mut max_size = Vec2::zero();
        for child in &mut self.childs { max_size = child.layout_compute(ctx, data, size).component_max(max_size); }
        max_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        for child in &mut self.childs { child.paint(ctx, data); }
    }
}

impl<'a, T, E> Set<'a, T, E>
{
    pub fn new() -> Self
    {
        Self { childs: Vec::new() }
    }

    pub fn from(widgets: impl IntoIterator<Item = Box<dyn Widget<T, E> + 'a>>) -> Self
    {
        Self { childs: widgets.into_iter().collect() }
    }

    pub fn with<W: Widget<T, E> + 'a>(mut self, widget: W) -> Self
    {
        self.childs.push(widget.boxed());
        self
    }

    pub fn with_box<W: Widget<T, E> + 'a>(mut self, widget: Box<W>) -> Self
    {
        self.childs.push(widget);
        self
    }

    pub fn add<W: Widget<T, E> + 'a>(&mut self, widget: W) -> &mut Self
    {
        self.childs.push(widget.boxed());
        self
    }

    pub fn add_box<W: Widget<T, E> + 'a>(&mut self, widget: Box<W>) -> &mut Self
    {
        self.childs.push(widget);
        self
    }
}
