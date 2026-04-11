/*
use super::*;

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
*/
