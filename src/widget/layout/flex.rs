use super::*;

pub enum FlexLayout
{
    Front,
    Center,
    Back,
    PadBetween,
    PadAll,
    GrowChilds
}

pub struct Flex<'a, const ROW: bool, T, E>
{
    childs: Vec<Box<dyn Widget<T, E> + 'a>>,
    padding: f32,
    layout: FlexLayout,
    //layout cache
    child_size_or_offset: Vec<f32>,
    total_primary_size: f32
}

impl<'a, const ROW: bool, T, E> Widget<T, E> for Flex<'a, ROW, T, E>
{
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T)
    {
        for (i, child) in self.childs.iter_mut().enumerate()
        {
            let offset = if ROW { Vec2(self.child_size_or_offset[i], 0.0) } else { Vec2(0.0, self.child_size_or_offset[i]) };
            ctx.event.event.offset(-offset);
            child.event(ctx, data);
            ctx.event.event.offset(offset);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, data: &mut T)
    {
        for child in &mut self.childs { child.update(ctx, data); }
    }

    fn widget_compute(&mut self, ctx: &mut WidgetComputeCtx, data: &mut T)
    {
        for child in &mut self.childs { child.widget_compute(ctx, data); }
    }

    fn layout_inquire(&mut self, ctx: &mut LayoutInquireCtx, data: &T) -> Vec2
    {
        let mut size = Vec2::zero();
        let (primary, secondary) = if ROW { (&mut size.0, &mut size.1) } else { (&mut size.1, &mut size.0) };
        for (i, child) in self.childs.iter_mut().enumerate()
        {
            let child_size = child.layout_inquire(ctx, data);
            let (child_primary, child_secondary) = if ROW { (child_size.0, child_size.1) } else { (child_size.1, child_size.0) };
            self.child_size_or_offset[i] = child_primary;
            *primary += child_primary;
            *secondary = secondary.max(child_secondary);
        }
        *primary += self.padding * (self.childs.len().max(1) - 1) as f32;
        self.total_primary_size = *primary;
        size
    }

    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2
    {
        let (primary, secondary) = if ROW { (size.0, size.1) } else { (size.1, size.0) };
        let mut delta = primary - self.total_primary_size;
        if delta < -ERR
        {
            ctx.does_not_fit();
            delta = 0.0;
        }
        let child_count = self.childs.len() as f32;
        let mids = child_count.max(1.0) - 1.0;
        let (pad_front, mut pad_mid, pad_back, growth) = match self.layout
        {
            FlexLayout::Front => (0.0, 0.0, delta, 0.0),
            FlexLayout::Center => (delta / 2.0, 0.0, delta / 2.0, 0.0),
            FlexLayout::Back => (delta, 0.0, 0.0, 0.0),
            FlexLayout::PadBetween => (0.0, delta / mids.max(1.0), 0.0, 0.0),
            FlexLayout::PadAll =>
            {
                let num = mids + 1.0;
                let pad = delta / num;
                (pad / 2.0, pad, pad / 2.0, 0.0)
            },
            FlexLayout::GrowChilds => (0.0, 0.0, 0.0, delta / child_count.max(1.0))
        };
        pad_mid += self.padding;
        let mut offset = pad_front;
        for (i, child) in self.childs.iter_mut().enumerate()
        {
            let child_primary = self.child_size_or_offset[i] + growth;
            let child_secondary = secondary;
            let child_size = if ROW { Vec2(child_primary, child_secondary) } else { Vec2(child_secondary, child_primary) };
            child.layout_compute(ctx, data, child_size);
            self.child_size_or_offset[i] = offset;
            offset += child_primary + pad_mid;
        }
        if self.childs.len() > 0 { offset -= pad_mid; }
        offset += pad_back;
        if ROW { Vec2(offset, secondary) } else { Vec2(secondary, offset) }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        for (i, child) in self.childs.iter_mut().enumerate()
        {
            let offset = if ROW { Vec2(self.child_size_or_offset[i], 0.0) } else { Vec2(0.0, self.child_size_or_offset[i]) };
            ctx.add_offset(offset);
            child.paint(ctx, data);
            ctx.add_offset(-offset);
        }
    }

    impl_respond_empty!(T);
}

impl<'a, const ROW: bool, T, E> Flex<'a, ROW, T, E>
{
    fn new() -> Self
    {
        Self
        {
            childs: Vec::new(),
            padding: 0.0,
            layout: FlexLayout::Front,
            child_size_or_offset: Vec::new(),
            total_primary_size: 0.0
        }
    }

    pub fn padding(mut self, padding: f32) -> Self
    {
        self.padding = padding;
        self
    }

    pub fn layout(mut self, layout: FlexLayout) -> Self
    {
        self.layout = layout;
        self
    }

    pub fn with<W: Widget<T, E> + 'a>(mut self, widget: W) -> Self
    {
        self.childs.push(widget.boxed());
        self.child_size_or_offset.push(0.0);
        self
    }

    pub fn with_box<W: Widget<T, E> + 'a>(mut self, widget: Box<W>) -> Self
    {
        self.childs.push(widget);
        self.child_size_or_offset.push(0.0);
        self
    }

    pub fn add<W: Widget<T, E> + 'a>(&mut self, widget: W) -> &mut Self
    {
        self.childs.push(widget.boxed());
        self.child_size_or_offset.push(0.0);
        self
    }

    pub fn add_box<W: Widget<T, E> + 'a>(&mut self, widget: Box<W>) -> &mut Self
    {
        self.childs.push(widget);
        self.child_size_or_offset.push(0.0);
        self
    }
}

impl<'a, T, E> Flex<'a, true, T, E> { pub fn row() -> Self { Self::new() } }
impl<'a, T, E> Flex<'a, false, T, E> { pub fn column() -> Self { Self::new() } }
