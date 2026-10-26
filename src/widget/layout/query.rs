use super::*;

#[derive(PartialEq)]
pub struct QueryData
{
    pub placement: Rect, //pixel
}

pub struct Query<T, E, W: Widget<T, E>, F>
{
    child: W,
    size: Vec2,
    last_submit: Option<QueryData>,
    to_submit: Option<QueryData>,
    f: F,
    _phantom: PhantomData<(T, E)>,
}

impl<T, E, W: Widget<T, E>, F> Widget<T, E> for Query<T, E, W, F>
where F: Fn(&QueryData) -> E
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T)
    {
        if matches!(ctx.event, WidgetEvent::NewData)
        {
            if let Some(query_data) = self.to_submit.take()
            {
                if self.last_submit.as_ref().is_none_or(|last| *last != query_data)
                {
                    ctx.emit(LogicEvent::Query((self.f)(&query_data)));
                }
                self.last_submit = Some(query_data);
            }
        }
        self.child.event(ctx, data);
    }

    impl_layout_inquire_child!(T);

    #[inline]
    fn layout_compute(&mut self, ctx: &mut LayoutComputeCtx, data: &T, size: Vec2) -> Vec2
    {
        self.size = self.child.layout_compute(ctx, data, size);
        self.size
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T)
    {
        let offset = ctx.current_offset();
        let scale = ctx.current_scale();
        let query_data = QueryData
        {
            placement: Rect { min: offset, max: offset + self.size } * scale
        };
        self.to_submit = Some(query_data);
        self.child.paint(ctx, data);
    }
}

impl<T, E, W: Widget<T, E>, F> Query<T, E, W, F>
where F: Fn(&QueryData) -> E
{
    pub fn new(widget: W, to_event: F) -> Self
    {
        Self { child: widget, last_submit: None, to_submit: None, size: Vec2::zero(), f: to_event, _phantom: PhantomData }
    }
}