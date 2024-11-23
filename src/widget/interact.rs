use super::*;

#[derive(Clone, Copy, PartialEq)]
pub enum WidgetState
{
    Cold,
    Hot,
    Hover
}

pub struct Response<'a, T, E: Clone, W: Widget<T, E>>
{
    child: W,
    size: Vec2,
    state: WidgetState,
    action: Option<Box<dyn FnMut(&mut Request, &mut T) + 'a>>,
    event: Option<E>
}

impl<'a, T, E: Clone, W: Widget<T, E>> Widget<T, E> for Response<'a, T, E, W>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx<E>, data: &mut T)
    {
        self.child.event(ctx, data);
        let mut update = false;
        match ctx.event.event
        {
            HardwareEvent::PointerGone =>
            {
                self.state = WidgetState::Cold;
                update = true;
            },
            HardwareEvent::PointerMoved { pos, .. } =>
            {
                let hover = Rect::new_origin(self.size).contains_linf(pos);
                if !hover && self.state != WidgetState::Cold
                {
                    self.state = WidgetState::Cold;
                    update = true;
                }
                if hover && self.state == WidgetState::Cold
                {
                    self.state = WidgetState::Hover;
                    update = true;
                }
            },
            HardwareEvent::PointerClicked { pos, button, pressed } =>
            {
                let mut maybe_button = None;
                let hover = Rect::new_origin(self.size).contains_linf(pos);
                if hover && pressed && !ctx.event.used
                {
                    self.state = WidgetState::Hot;
                    update = true;
                    ctx.event.used = true;
                }
                if hover && self.state == WidgetState::Hot && !pressed
                {
                    self.state = WidgetState::Hover;
                    update = true;
                    ctx.event.used = true;
                    maybe_button = Some(button);
                }
                if !pressed && self.child.respond(data, maybe_button) { update = true; }
                if maybe_button.is_some()
                {
                    if let Some(action) = &mut self.action { action(ctx.request, data); }
                    if let Some(tag) = &self.event
                    {
                        ctx.emit(LogicEvent::Clicked(tag.clone(), button));
                    }
                }
            },
            HardwareEvent::Key { key: keycode, pressed } =>
            {
                if let Some(tag) = &self.event
                {
                    ctx.emit(LogicEvent::Pressed(tag.clone(), keycode, pressed));
                }
            },
            _ => {}
        }
        if update
        {
            ctx.request.paint();
        }
    }

    impl_update_child!(T);
    impl_widget_compute_child!(T);
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
        let old_state = ctx.state;
        ctx.state = self.state;
        self.child.paint(ctx, data);
        ctx.state = old_state;
    }

    impl_respond_empty!(T);
}

impl<'a, T, E: Clone, W: Widget<T, E>> Response<'a, T, E, W>
{
    pub fn new(widget: W) -> Self
    {
        Self
        {
            child: widget,
            size: Vec2::zero(),
            state: WidgetState::Cold,
            action: None,
            event: None
        }
    }

    pub fn action<A: FnMut(&mut Request, &mut T) + 'a>(mut self, action: A) -> Self
    {
        self.action = Some(Box::new(action));
        self
    }

    pub fn event(mut self, tag: E) -> Self
    {
        self.event = Some(tag);
        self
    }
}
