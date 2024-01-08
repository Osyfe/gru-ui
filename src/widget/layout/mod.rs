mod flex;
pub use flex::*;

use super::*;

const ERR: f32 = 1e-3;

pub struct Empty;

impl<T, E> Widget<T, E> for Empty
{
    impl_event_empty!(T);
    impl_update_empty!(T);
    impl_widget_compute_empty!(T);
    impl_layout_inquire_empty!(T);
    impl_layout_compute_empty!(T);
    impl_paint_empty!(T);
    impl_respond_empty!(T);
}

pub struct Fix<T, E, W: Widget<T, E>>
{
    child: W,
    width: Option<f32>,
    height: Option<f32>,
    _phantom: PhantomData<(T, E)>
}

impl<T, E, W: Widget<T, E>> Widget<T, E> for Fix<T, E, W>
{
    impl_event_child!(T);
    impl_update_child!(T);
    impl_widget_compute_child!(T);

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2
    {
        let mut size = self.child.layout_inquire(ctx, data);
        if let Some(width) = self.width { size.0 = width; }
        if let Some(height) = self.height { size.1 = height; }
        size
    }

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, mut size: Vec2) -> Vec2
    {
        if let Some(width) = self.width { size.0 = width; }
        if let Some(height) = self.height { size.1 = height; }
        self.child.layout_compute(ctx, data, size)
    }

    impl_paint_child!(T);
    impl_respond_child!(T);
}

impl<T, E, W: Widget<T, E>> Fix<T, E, W>
{
    pub fn new(widget: W) -> Self
    {
        Self { child: widget, width: None, height: None, _phantom: PhantomData }
    }

    pub fn width(mut self, width: f32) -> Self
    {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self
    {
        self.height = Some(height);
        self
    }
}

pub enum AlignLayout
{
    Front,
    Mid,
    Back
}

pub struct Align<T, E, W: Widget<T, E>>
{
    child: W,
    horizontal: AlignLayout,
    vertical: AlignLayout,
    _phantom: PhantomData<(T, E)>,
    //layout cache
    size_or_offset: Vec2
}

impl<T, E, W: Widget<T, E>> Widget<T, E> for Align<T, E, W>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T)
    {
        ctx.event.event.offset(-self.size_or_offset);
        self.child.event(ctx, data);
        ctx.event.event.offset(self.size_or_offset);
    }

    impl_update_child!(T);
    impl_widget_compute_child!(T);

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2
    {
        self.size_or_offset = self.child.layout_inquire(ctx, data);
        self.size_or_offset
    }

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2
    {
        let delta = Vec2((size.0 - self.size_or_offset.0).max(0.0), (size.1 - self.size_or_offset.1).max(0.0));
        self.size_or_offset = delta;
        match self.horizontal
        {
            AlignLayout::Front => self.size_or_offset.0 = 0.0,
            AlignLayout::Mid => self.size_or_offset.0 /= 2.0,
            AlignLayout::Back => {}
        }
        match self.vertical
        {
            AlignLayout::Front => self.size_or_offset.1 = 0.0,
            AlignLayout::Mid => self.size_or_offset.1 /= 2.0,
            AlignLayout::Back => {}
        }
        let child_size = self.child.layout_compute(ctx, data, size - delta);
        Vec2(size.0.max(child_size.0), size.1.max(child_size.1))
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        ctx.add_offset(self.size_or_offset);
        self.child.paint(ctx, data);
        ctx.add_offset(-self.size_or_offset);
    }

    impl_respond_child!(T);
}

impl<T, E, W: Widget<T, E>> Align<T, E, W>
{
    pub fn new(widget: W) -> Self
    {
        Self
        {
            child: widget,
            horizontal: AlignLayout::Front,
            vertical: AlignLayout::Front,
            _phantom: PhantomData,
            size_or_offset: Vec2::zero()
        }
    }

    pub fn horizontal(mut self, layout: AlignLayout) -> Self
    {
        self.horizontal = layout;
        self
    }

    pub fn vertical(mut self, layout: AlignLayout) -> Self
    {
        self.vertical = layout;
        self
    }
    
    pub fn left(self) -> Self { self.horizontal(AlignLayout::Front) }
    pub fn right(self) -> Self { self.horizontal(AlignLayout::Back) }
    pub fn up(self) -> Self { self.vertical(AlignLayout::Front) }
    pub fn down(self) -> Self { self.vertical(AlignLayout::Back) }
    pub fn center_h(self) -> Self { self.horizontal(AlignLayout::Mid) }
    pub fn center_v(self) -> Self { self.vertical(AlignLayout::Mid) }
    pub fn center(self) -> Self { self.center_h().center_v() }
}

pub struct Padding<T, E, W: Widget<T, E>>
{
    child: W,
    padding: Rect,
    _phantom: PhantomData<(T, E)>
}

impl<T, E, W: Widget<T, E>> Widget<T, E> for Padding<T, E, W>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T)
    {
        ctx.event.event.offset(-self.padding.min);
        self.child.event(ctx, data);
        ctx.event.event.offset(self.padding.min);
    }

    impl_update_child!(T);
    impl_widget_compute_child!(T);

    #[inline]
    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2
    {
        self.child.layout_inquire(ctx, data) + self.padding.min + self.padding.max
    }

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2
    {
        let size = size - self.padding.min - self.padding.max;
        if size.0 <= 0.0 || size.1 <= 0.0 { ctx.does_not_fit(); }
        let size = self.child.layout_compute(ctx, data, size);
        size + self.padding.min + self.padding.max
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        ctx.add_offset(self.padding.min);
        self.child.paint(ctx, data);
        ctx.add_offset(-self.padding.min);
    }

    impl_respond_child!(T);
}

impl<T, E, W: Widget<T, E>> Padding<T, E, W>
{
    pub fn new(widget: W) -> Self
    {
        Self { child: widget, padding: Rect::new_origin(Vec2::zero()), _phantom: PhantomData }
    }

    pub fn left(mut self, padding: f32) -> Self
    {
        self.padding.min.0 = padding;
        self
    }

    pub fn right(mut self, padding: f32) -> Self
    {
        self.padding.max.0 = padding;
        self
    }

    pub fn up(mut self, padding: f32) -> Self
    {
        self.padding.min.1 = padding;
        self
    }

    pub fn down(mut self, padding: f32) -> Self
    {
        self.padding.max.1 = padding;
        self
    }

    pub fn horizontal(mut self, padding: f32) -> Self
    {
        self.padding.min.0 = padding;
        self.padding.max.0 = padding;
        self
    }

    pub fn vertical(mut self, padding: f32) -> Self
    {
        self.padding.min.1 = padding;
        self.padding.max.1 = padding;
        self
    }

    pub fn all(mut self, padding: f32) -> Self
    {
        self.padding.min.0 = padding;
        self.padding.max.0 = padding;
        self.padding.min.1 = padding;
        self.padding.max.1 = padding;
        self
    }
}
